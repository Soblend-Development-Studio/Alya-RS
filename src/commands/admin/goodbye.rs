use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GoodbyeCommand;

#[async_trait]
impl Command for GoodbyeCommand {
    fn triggers(&self) -> &[&str] { &["goodbye", "despedida"] }
    fn category(&self) -> &str { "admin" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }

        let group = ctx.db.get_group(&ctx.chat_id).await?;
        let new_state = !group.settings.goodbye;
        ctx.db.update_group(&ctx.chat_id, mongodb::bson::doc! { "settings.goodbye": new_state }).await?;
        ctx.reply_styled(&format!("ꕣ Mensajes de despedida *{}*.",
            if new_state { "ACTIVADOS ✅" } else { "DESACTIVADOS ❌" })).await
    }
}
