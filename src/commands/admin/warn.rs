use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct WarnCommand;

#[async_trait]
impl Command for WarnCommand {
    fn triggers(&self) -> &[&str] { &["warn", "advertir"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Advierte a un usuario. 3 advertencias = expulsión" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }

        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() { return ctx.reply_styled("ꕢ Menciona al usuario a advertir.").await; }

        let target = &mentions[0];
        let warn_key = format!("warn_{}_{}", ctx.chat_id, target);
        let current_warns: i32 = ctx.cache.get(&warn_key).and_then(|v| v.parse().ok()).unwrap_or(0);
        let new_warns = current_warns + 1;

        ctx.cache.set(&warn_key, &new_warns.to_string(), 86400 * 7);

        if new_warns >= 3 {
            let bot_admin = ctx.client.is_bot_admin(&ctx.chat_id).await.unwrap_or(false);
            if bot_admin {
                let _ = ctx.client.kick_participant(&ctx.chat_id, target).await;
                ctx.cache.delete(&warn_key);
                return ctx.reply_styled(&format!(
                    "ꕢ @{} fue expulsado por acumular 3 advertencias.",
                    target.split('@').next().unwrap_or("?")
                )).await;
            }
        }

        ctx.reply_styled(&format!(
            "⚠️ *ADVERTENCIA*\n\n\
             @{} tiene *{}/3* advertencias.\n\
             Al llegar a 3 será expulsado.",
            target.split('@').next().unwrap_or("?"),
            new_warns
        )).await
    }
}
