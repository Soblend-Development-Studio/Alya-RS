use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SpotifySearchCommand;
#[async_trait]
impl Command for SpotifySearchCommand {
    fn triggers(&self) -> &[&str] { &["spsearch", "spotifysearch"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca canciones en Spotify" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#spsearch <canción>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("🎵 Buscando en Spotify: *{}*\n\n_Usa `#spotify` para descargar._", q)).await
    }
}
