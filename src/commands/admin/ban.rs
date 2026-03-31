use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct BanCommand;

#[async_trait]
impl Command for BanCommand {
    fn triggers(&self) -> &[&str] { &["ban"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Banea a un usuario (lo expulsa y lo bloquea)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        let bot_admin = ctx.client.is_bot_admin(&ctx.chat_id).await.unwrap_or(false);
        if !bot_admin { return ctx.reply_styled(crate::config::errors::BOT_ADMIN_REQUIRED).await; }

        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() { return ctx.reply_styled("ꕢ Menciona a alguien para banear.").await; }

        for target in &mentions {
            let _ = ctx.client.kick_participant(&ctx.chat_id, target).await;
        }

        ctx.reply_styled(&format!(
            "ꕢ @{} fue baneado del grupo.",
            mentions[0].split('@').next().unwrap_or("?")
        )).await
    }
}
