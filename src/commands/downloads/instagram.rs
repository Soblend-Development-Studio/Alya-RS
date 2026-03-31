use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct InstagramCommand;
#[async_trait]
impl Command for InstagramCommand {
    fn triggers(&self) -> &[&str] { &["ig", "instagram", "insta"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga video/imagen de Instagram" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#ig <url>`").await; }
        ctx.reply_styled("📸 _Descargando Instagram..._\n\n_Función en desarrollo_").await
    }
}
