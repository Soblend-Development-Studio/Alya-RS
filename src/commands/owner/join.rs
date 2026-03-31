use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct JoinCommand;
#[async_trait] impl Command for JoinCommand {
    fn triggers(&self) -> &[&str] { &["join"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("ꕢ Función de unirse a grupos en desarrollo.").await
    }
}
