use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SoundcloudCommand;
#[async_trait]
impl Command for SoundcloudCommand {
    fn triggers(&self) -> &[&str] { &["soundcloud", "sc"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca y descarga música de SoundCloud" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#soundcloud <canción>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("☁️ Buscando en SoundCloud: *{}*...\n\n_Función en desarrollo_", q)).await
    }
}
