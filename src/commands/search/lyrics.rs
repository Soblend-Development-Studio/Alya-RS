use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct LyricsCommand;

#[async_trait]
impl Command for LyricsCommand {
    fn triggers(&self) -> &[&str] { &["lyrics", "letra"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca la letra de una canción" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#lyrics <canción - artista>`").await; }
        let query = ctx.args.join(" ");

        let url = format!("https://api.xteam.xyz/lyrics?title={}", urlencoding::encode(&query));
        let client = reqwest::Client::new();

        match client.get(&url).timeout(std::time::Duration::from_secs(10)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let lyrics = json["lyrics"].as_str().or_else(|| json["result"].as_str()).unwrap_or("Sin letra encontrada");
                let snippet = &lyrics[..lyrics.len().min(2000)];
                ctx.reply_styled(&format!("🎵 *Letra: {}*\n\n{}", query, snippet)).await
            }
            Err(_) => ctx.reply_styled("ꕢ No encontré la letra.").await
        }
    }
}
