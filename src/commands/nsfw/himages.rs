use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct HimagesCommand;

#[async_trait]
impl Command for HimagesCommand {
    fn triggers(&self) -> &[&str] { &["himages", "himage"] }
    fn category(&self) -> &str { "nsfw" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group {
            let group = ctx.db.get_group(&ctx.chat_id).await?;
            if !group.settings.nsfw {
                return ctx.reply_styled("🔞 El contenido NSFW está desactivado en este grupo.\nUsa `#nsfw` para activarlo (solo admins).").await;
            }
        }

        let client = reqwest::Client::new();
        let resp = client.get("https://api.waifu.pics/nsfw/waifu").timeout(std::time::Duration::from_secs(10)).send().await;
        match resp {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                if let Some(url) = json["url"].as_str() {
                    ctx.send_image(url, "🔞").await
                } else {
                    ctx.reply_styled("ꕢ No se pudo obtener imagen.").await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al obtener imagen.").await
        }
    }
}
