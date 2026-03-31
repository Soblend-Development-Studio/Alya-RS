use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct FacebookCommand;
#[async_trait]
impl Command for FacebookCommand {
    fn triggers(&self) -> &[&str] { &["fb", "facebook"] }
    fn category(&self) -> &str { "downloads" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#fb <url>`").await; }
        ctx.reply_styled("📘 _Descargando Facebook..._\n\n_Función en desarrollo_").await
    }
}
