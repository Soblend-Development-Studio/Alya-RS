use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SpotifyCommand;
#[async_trait]
impl Command for SpotifyCommand {
    fn triggers(&self) -> &[&str] { &["spotify", "sp"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga canción de Spotify" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#spotify <url o nombre>`").await; }
        ctx.reply_styled("🎵 _Procesando Spotify..._\n\n_Función en desarrollo_").await
    }
}
