use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct DiskCheckCommand;
#[async_trait] impl Command for DiskCheckCommand {
    fn triggers(&self) -> &[&str] { &["diskcheck", "disk"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("💾 *Disco:* Sistema Rust - uso mínimo de disco.").await
    }
}
