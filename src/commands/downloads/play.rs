use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct PlayCommand;
#[async_trait]
impl Command for PlayCommand {
    fn triggers(&self) -> &[&str] { &["play", "reproducir"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Reproduce música desde YouTube" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#play <canción>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("🎵 _Buscando: {}..._", q)).await?;
        ctx.reply_styled("ꕢ Usa `#ytmp3 <url>` para descargar el audio directamente.").await
    }
}
