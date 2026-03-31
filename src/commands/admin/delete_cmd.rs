use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct DeleteCommand;

#[async_trait]
impl Command for DeleteCommand {
    fn triggers(&self) -> &[&str] { &["delete", "del", "borrar"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Elimina el mensaje respondido" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        ctx.reply_styled("ꕣ Mensaje eliminado.").await
    }
}
