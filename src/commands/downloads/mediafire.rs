use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct MediafireCommand;
#[async_trait]
impl Command for MediafireCommand {
    fn triggers(&self) -> &[&str] { &["mediafire", "mf"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga archivo de MediaFire" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#mf <url de mediafire>`").await; }
        ctx.reply_styled("📦 _Obteniendo enlace de descarga..._\n\n_Función en desarrollo_").await
    }
}
