use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct DelbaltopCommand;
#[async_trait] impl Command for DelbaltopCommand {
    fn triggers(&self) -> &[&str] { &["delbaltop"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("ꕣ Función disponible próximamente.").await
    }
}
