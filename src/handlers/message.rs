use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn, error, debug};

use crate::commands::{CommandRegistry, CommandContext, WhatsAppClient};
use crate::db::DatabaseService;
use crate::services::{GachaService, ShopService, LevelService};
use crate::utils::CacheManager;
use crate::utils::format::style_text;
use crate::config::BotConfig;

pub struct MessageHandler {
    pub registry: Arc<CommandRegistry>,
    pub db: Arc<DatabaseService>,
    pub gacha: Arc<GachaService>,
    pub shop: Arc<ShopService>,
    pub level: Arc<LevelService>,
    pub cache: Arc<CacheManager>,
    pub config: Arc<BotConfig>,
}

pub struct IncomingMessage {
    pub sender: String,
    pub sender_lid: Option<String>,
    pub chat_id: String,
    pub body: String,
    pub is_group: bool,
    pub is_from_me: bool,
    pub push_name: Option<String>,
    pub raw: Option<serde_json::Value>,
}

impl MessageHandler {
    pub async fn handle(
        &self,
        msg: IncomingMessage,
        client: Arc<dyn WhatsAppClient + Send + Sync>,
    ) {
        // Skip if from self
        if msg.is_from_me { return; }

        // Check for spam
        let spam_key = format!("spam_{}", msg.sender);
        let spam_count: u32 = self.cache.get(&spam_key).and_then(|v| v.parse().ok()).unwrap_or(0);

        if spam_count >= 10 {
            let timeout_key = format!("timeout_{}", msg.sender);
            if !self.cache.has(&timeout_key) {
                self.cache.set(&timeout_key, "1", 30);
                let _ = client.send_text(&msg.chat_id, &style_text(
                    "ꕢ Has sido silenciado por 30 segundos por spam."
                )).await;
            }
            return;
        }

        self.cache.set(&spam_key, &(spam_count + 1).to_string(), 10);

        // Parse command
        let body = msg.body.trim().to_string();
        let prefix = self.config.prefixes.iter()
            .find(|p| body.starts_with(p.as_str()))
            .cloned();

        let is_owner = self.config.is_owner(&msg.sender);

        // Run before-handlers (middleware)
        for before_cmd in self.registry.before_handlers() {
            let ctx = CommandContext {
                client: client.clone(),
                sender: msg.sender.clone(),
                sender_lid: msg.sender_lid.clone(),
                sender_phone: Some(crate::utils::helpers::extract_phone(&msg.sender)),
                chat_id: msg.chat_id.clone(),
                is_group: msg.is_group,
                body: body.clone(),
                args: vec![],
                command: String::new(),
                prefix: String::new(),
                is_from_me: msg.is_from_me,
                is_owner,
                push_name: msg.push_name.clone(),
                db: self.db.clone(),
                gacha: self.gacha.clone(),
                shop: self.shop.clone(),
                level: self.level.clone(),
                cache: self.cache.clone(),
                config: self.config.clone(),
                raw_message: msg.raw.clone(),
            };
            match before_cmd.before(&ctx).await {
                Ok(true) => {}
                Ok(false) => return,
                Err(e) => {
                    error!("Error en before handler: {}", e);
                    return;
                }
            }
        }

        // Only process if command prefix found
        let prefix = match prefix {
            Some(p) => p,
            None => {
                // Add XP for regular messages
                let xp_key = format!("xp_{}", msg.sender);
                if !self.cache.has(&xp_key) {
                    self.cache.set(&xp_key, "1", 60);
                    let _ = self.level.add_xp(&msg.sender, 5).await;
                }
                return;
            }
        };

        // Parse command name and args
        let without_prefix = &body[prefix.len()..];
        let mut parts: Vec<String> = without_prefix
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parts.is_empty() { return; }

        let command_name = parts.remove(0).to_lowercase();
        let args = parts;

        debug!("Comando: {} | Args: {:?} | From: {}", command_name, args, msg.sender);

        // Look up command
        let cmd = match self.registry.get(&command_name) {
            Some(c) => c,
            None => return, // Silently ignore unknown commands
        };

        // Build context
        let ctx = CommandContext {
            client: client.clone(),
            sender: msg.sender.clone(),
            sender_lid: msg.sender_lid.clone(),
            sender_phone: Some(crate::utils::helpers::extract_phone(&msg.sender)),
            chat_id: msg.chat_id.clone(),
            is_group: msg.is_group,
            body: body.clone(),
            args,
            command: command_name.clone(),
            prefix: prefix.clone(),
            is_from_me: msg.is_from_me,
            is_owner,
            push_name: msg.push_name.clone(),
            db: self.db.clone(),
            gacha: self.gacha.clone(),
            shop: self.shop.clone(),
            level: self.level.clone(),
            cache: self.cache.clone(),
            config: self.config.clone(),
            raw_message: msg.raw.clone(),
        };

        // Rate limiting per user
        let rate_key = format!("rate_{}", msg.sender);
        if self.cache.has(&rate_key) && !is_owner {
            let _ = client.send_text(&msg.chat_id, "ꕢ Espera un momento antes de usar otro comando.").await;
            return;
        }
        self.cache.set(&rate_key, "1", 1);

        // Save name if we have it
        if let Some(ref name) = msg.push_name {
            let name_key = format!("name_{}", msg.sender);
            if !self.cache.has(&name_key) {
                self.cache.set(&name_key, name, 300);
                let _ = self.db.update_user(&msg.sender, mongodb::bson::doc! {
                    "name": name
                }).await;
            }
        }

        // Execute command
        info!("▶ {} → #{} {:?}", msg.sender, command_name, ctx.args);
        if let Err(e) = cmd.execute(&ctx).await {
            error!("ꕢ Error ejecutando #{}: {}", command_name, e);
            let _ = client.send_text(
                &msg.chat_id,
                &style_text("ꕢ Ocurrió un error al ejecutar el comando.")
            ).await;
        }

        // Add stats
        let _ = self.db.update_user(&msg.sender, mongodb::bson::doc! {
            "$inc": { "stats.commands": 1 }
        }).await;
    }
}
