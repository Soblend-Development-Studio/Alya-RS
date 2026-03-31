use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct AntiprivadoCommand;
#[async_trait] impl Command for AntiprivadoCommand {
    fn triggers(&self) -> &[&str] { &["antiprivado"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("ꕣ Función anti-privado disponible próximamente.").await
    }
}
