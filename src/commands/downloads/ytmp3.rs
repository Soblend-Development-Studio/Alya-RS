use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct Ytmp3Command;

#[async_trait]
impl Command for Ytmp3Command {
    fn triggers(&self) -> &[&str] { &["ytmp3", "mp3"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga audio de YouTube" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#ytmp3 <url de youtube>`").await; }
        let url = &ctx.args[0];

        if !url.contains("youtube.com") && !url.contains("youtu.be") {
            return ctx.reply_styled("ꕢ Proporciona una URL válida de YouTube.").await;
        }

        ctx.reply_styled("🎵 _Descargando audio..._").await?;

        let api_url = format!("https://api.xteam.xyz/ytdl/mp3?url={}", urlencoding::encode(url));
        let client = reqwest::Client::new();

        match client.get(&api_url).timeout(std::time::Duration::from_secs(60)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let dl_url = json["url"].as_str().or_else(|| json["download"].as_str());
                match dl_url {
                    Some(u) => ctx.client.send_audio(&ctx.chat_id, u).await,
                    None => ctx.reply_styled("ꕢ No se pudo descargar el audio.").await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al descargar audio.").await
        }
    }
}
