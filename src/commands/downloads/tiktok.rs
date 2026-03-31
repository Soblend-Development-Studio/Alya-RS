use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TiktokCommand;

#[async_trait]
impl Command for TiktokCommand {
    fn triggers(&self) -> &[&str] { &["tiktok", "tt"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Descarga video de TikTok sin marca de agua" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#tiktok <url>`").await; }
        let url = &ctx.args[0];
        ctx.reply_styled("⏳ _Descargando TikTok..._").await?;

        let api_url = format!("https://api.xteam.xyz/tiktok?url={}", urlencoding::encode(url));
        let client = reqwest::Client::new();

        match client.get(&api_url).timeout(std::time::Duration::from_secs(30)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let vid_url = json["video"]["noWatermark"].as_str()
                    .or_else(|| json["url"].as_str())
                    .or_else(|| json["download"].as_str());
                match vid_url {
                    Some(u) => ctx.client.send_video(&ctx.chat_id, u, "TikTok").await,
                    None => ctx.reply_styled("ꕢ No se pudo descargar el video.").await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al descargar.").await
        }
    }
}
