use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct Ytmp4Command;

#[async_trait]
impl Command for Ytmp4Command {
    fn triggers(&self) -> &[&str] { &["ytmp4", "mp4"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga video de YouTube" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#ytmp4 <url de youtube>`").await; }
        let url = &ctx.args[0];
        ctx.reply_styled("🎬 _Descargando video..._").await?;

        let api_url = format!("https://api.xteam.xyz/ytdl/mp4?url={}", urlencoding::encode(url));
        let client = reqwest::Client::new();

        match client.get(&api_url).timeout(std::time::Duration::from_secs(60)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let dl_url = json["url"].as_str().or_else(|| json["download"].as_str());
                match dl_url {
                    Some(u) => ctx.client.send_video(&ctx.chat_id, u, "YouTube Video").await,
                    None => ctx.reply_styled("ꕢ No se pudo descargar el video.").await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al descargar video.").await
        }
    }
}
