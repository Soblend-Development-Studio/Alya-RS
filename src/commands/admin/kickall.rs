use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct KickallCommand;

#[async_trait]
impl Command for KickallCommand {
    fn triggers(&self) -> &[&str] { &["kickall"] }
    fn category(&self) -> &str { "admin" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        let bot_admin = ctx.client.is_bot_admin(&ctx.chat_id).await.unwrap_or(false);
        if !bot_admin { return ctx.reply_styled(crate::config::errors::BOT_ADMIN_REQUIRED).await; }

        let participants = ctx.client.get_group_participants(&ctx.chat_id).await?;
        let my_jid = ctx.client.my_jid();
        let mut kicked = 0;

        for p in &participants {
            if p.is_admin || p.id == my_jid { continue; }
            if let Ok(_) = ctx.client.kick_participant(&ctx.chat_id, &p.id).await {
                kicked += 1;
            }
        }

        ctx.reply_styled(&format!("ꕣ Se expulsaron *{}* miembros.", kicked)).await
    }
}
