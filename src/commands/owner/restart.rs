use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct RestartCommand;
#[async_trait] impl Command for RestartCommand {
    fn triggers(&self) -> &[&str] { &["restart", "reiniciar"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("🔄 Reiniciando bot...").await?;
        std::process::exit(0);
    }
}
