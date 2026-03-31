use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct KickCommand;

#[async_trait]
impl Command for KickCommand {
    fn triggers(&self) -> &[&str] { &["kick", "expulsar", "eliminar"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Expulsa a un miembro del grupo" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        let bot_admin = ctx.client.is_bot_admin(&ctx.chat_id).await.unwrap_or(false);
        if !bot_admin { return ctx.reply_styled(crate::config::errors::BOT_ADMIN_REQUIRED).await; }

        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() { return ctx.reply_styled("ꕢ Menciona a alguien para expulsar.").await; }

        for target in &mentions {
            if target == &ctx.sender { continue; }
            let _ = ctx.client.kick_participant(&ctx.chat_id, target).await;
        }

        ctx.reply_styled(&format!(
            "ꕣ @{} fue expulsado del grupo.",
            mentions[0].split('@').next().unwrap_or("?")
        )).await
    }
}
