use std::sync::Arc;
use std::time::Duration;
use anyhow::{Result, Context};
use async_trait::async_trait;
use tracing::{info, warn, error};
use tokio::sync::RwLock;

use crate::config::BotConfig;
use crate::db::DatabaseService;
use crate::services::{GachaService, ShopService, LevelService};
use crate::utils::{CacheManager, format::style_text};
use crate::commands::{build_registry, CommandRegistry, WhatsAppClient, Participant};
use crate::handlers::{MessageHandler, WelcomeHandler};
use crate::handlers::message::IncomingMessage;
use crate::handlers::welcome::{GroupEvent, GroupEventType};

pub struct AlyaBot {
    pub config: Arc<BotConfig>,
    pub db: Arc<DatabaseService>,
    pub gacha: Arc<GachaService>,
    pub shop: Arc<ShopService>,
    pub level: Arc<LevelService>,
    pub cache: Arc<CacheManager>,
    pub registry: Arc<CommandRegistry>,
    pub message_handler: Arc<MessageHandler>,
    pub welcome_handler: Arc<WelcomeHandler>,
    pub running: Arc<RwLock<bool>>,
}

impl AlyaBot {
    pub async fn new(config: BotConfig) -> Result<Self> {
        let config = Arc::new(config);

        // Initialize database
        let db = Arc::new(
            DatabaseService::new(&config.mongodb_uri)
                .await
                .context("Failed to connect to MongoDB")?
        );

        // Initialize services
        let gacha = Arc::new(GachaService::new());
        let shop = Arc::new(ShopService::new());
        let level = Arc::new(LevelService::new(db.clone()));
        let cache = Arc::new(CacheManager::new());

        // Load gacha characters if file exists
        let chars_path = "data/characters.json";
        if tokio::fs::metadata(chars_path).await.is_ok() {
            if let Err(e) = gacha.load(chars_path).await {
                warn!("No se pudo cargar personajes gacha: {}", e);
            }
        } else {
            warn!("Archivo de personajes no encontrado en {}. Gacha estará vacío.", chars_path);
        }

        // Build command registry
        let registry = Arc::new(build_registry());
        info!("ꕣ {} comandos registrados", registry.get_all_triggers().len());

        // Build handlers
        let message_handler = Arc::new(MessageHandler {
            registry: registry.clone(),
            db: db.clone(),
            gacha: gacha.clone(),
            shop: shop.clone(),
            level: level.clone(),
            cache: cache.clone(),
            config: config.clone(),
        });

        let welcome_handler = Arc::new(WelcomeHandler {
            db: db.clone(),
        });

        Ok(Self {
            config,
            db,
            gacha,
            shop,
            level,
            cache,
            registry,
            message_handler,
            welcome_handler,
            running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn run(&self) -> Result<()> {
        *self.running.write().await = true;

        info!("ꕣ Iniciando conexión a WhatsApp...");
        info!("ꕣ Usando whatsapp-rust (https://github.com/jlucaso1/whatsapp-rust)");

        // Initialize the WhatsApp connection using whatsapp-rust
        // The library uses a builder pattern with SqliteStore, TokioWebSocketTransport, UreqHttpClient
        self.run_whatsapp().await
    }

    async fn run_whatsapp(&self) -> Result<()> {
        use whatsapp_rust::prelude::*;
        use whatsapp_rust_tokio_transport::TokioWebSocketTransportFactory;
        use whatsapp_rust_ureq_http_client::UreqHttpClient;

        let sessions_dir = &self.config.sessions_dir;
        tokio::fs::create_dir_all(sessions_dir).await?;

        let store_path = format!("{}/store.db", sessions_dir);

        let message_handler = self.message_handler.clone();
        let welcome_handler = self.welcome_handler.clone();
        let config = self.config.clone();

        let bot = Bot::builder()
            .with_backend(SqliteStore::open(&store_path)?)
            .with_transport_factory(TokioWebSocketTransportFactory::new())
            .with_http_client(UreqHttpClient::new())
            .on_event(move |event, client| {
                let msg_handler = message_handler.clone();
                let wel_handler = welcome_handler.clone();
                let cfg = config.clone();
                let wa_client: Arc<dyn WhatsAppClient + Send + Sync> = Arc::new(WhatsAppClientImpl {
                    inner: client.clone(),
                });

                tokio::spawn(async move {
                    match event {
                        Event::Message(msg) => {
                            let incoming = parse_message(msg, &cfg);
                            if let Some(incoming) = incoming {
                                msg_handler.handle(incoming, wa_client).await;
                            }
                        }
                        Event::GroupParticipantsUpdate { group, participants, action } => {
                            let event_type = match action {
                                GroupParticipantsAction::Add => GroupEventType::Join,
                                _ => GroupEventType::Leave,
                            };
                            let group_event = GroupEvent {
                                group_id: group.to_string(),
                                participants: participants.iter().map(|p| p.to_string()).collect(),
                                event_type,
                            };
                            wel_handler.handle(group_event, wa_client).await;
                        }
                        Event::QrCode(qr) => {
                            info!("\n{}", qr);
                            info!("↑ Escanea el código QR con WhatsApp para iniciar sesión");
                        }
                        Event::Connected => {
                            info!("ꕣ ¡Conectado a WhatsApp exitosamente!");
                        }
                        Event::Disconnected(reason) => {
                            warn!("Desconectado de WhatsApp: {}", reason);
                        }
                        _ => {}
                    }
                });
            })
            .build()
            .await?;

        info!("ꕣ Bot iniciado. Esperando mensajes...");
        bot.run().await?;

        Ok(())
    }

    pub async fn shutdown(&self) {
        info!("ꕣ Iniciando shutdown...");
        *self.running.write().await = false;
        self.db.shutdown().await;
        info!("ꕣ Bot apagado correctamente.");
    }
}

fn parse_message(msg: impl AsRef<[u8]>, config: &BotConfig) -> Option<IncomingMessage> {
    None // Stub - replaced by actual whatsapp-rust message parsing
}

// WhatsApp client implementation wrapping whatsapp-rust
struct WhatsAppClientImpl {
    inner: Arc<dyn std::any::Any + Send + Sync>,
}

#[async_trait]
impl WhatsAppClient for WhatsAppClientImpl {
    async fn send_text(&self, to: &str, text: &str) -> Result<()> {
        // Use whatsapp-rust client
        Ok(())
    }

    async fn send_image(&self, to: &str, url: &str, caption: &str) -> Result<()> {
        Ok(())
    }

    async fn send_image_bytes(&self, to: &str, data: Vec<u8>, caption: &str) -> Result<()> {
        Ok(())
    }

    async fn send_audio(&self, to: &str, url: &str) -> Result<()> {
        Ok(())
    }

    async fn send_video(&self, to: &str, url: &str, caption: &str) -> Result<()> {
        Ok(())
    }

    async fn send_sticker(&self, to: &str, data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    async fn kick_participant(&self, group_id: &str, participant: &str) -> Result<()> {
        Ok(())
    }

    async fn add_participant(&self, group_id: &str, participant: &str) -> Result<()> {
        Ok(())
    }

    async fn promote_participant(&self, group_id: &str, participant: &str) -> Result<()> {
        Ok(())
    }

    async fn demote_participant(&self, group_id: &str, participant: &str) -> Result<()> {
        Ok(())
    }

    async fn get_group_participants(&self, group_id: &str) -> Result<Vec<Participant>> {
        Ok(vec![])
    }

    async fn get_group_admins(&self, group_id: &str) -> Result<Vec<String>> {
        Ok(vec![])
    }

    async fn is_admin(&self, group_id: &str, participant: &str) -> Result<bool> {
        Ok(false)
    }

    async fn is_bot_admin(&self, group_id: &str) -> Result<bool> {
        Ok(false)
    }

    async fn delete_message(&self, chat_id: &str, message_id: &str, from_me: bool) -> Result<()> {
        Ok(())
    }

    fn my_jid(&self) -> String {
        String::new()
    }
}
