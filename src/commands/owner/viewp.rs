use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct ViewpCommand;
#[async_trait] impl Command for ViewpCommand {
    fn triggers(&self) -> &[&str] { &["viewp"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("ꕣ Función viewonce disponible próximamente.").await
    }
}
