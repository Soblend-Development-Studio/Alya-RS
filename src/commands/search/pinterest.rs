use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct PinterestCommand;

#[async_trait]
impl Command for PinterestCommand {
    fn triggers(&self) -> &[&str] { &["pinterest", "pin"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca imágenes en Pinterest" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#pinterest <búsqueda>`").await; }
        let query = ctx.args.join(" ");

        let url = format!("https://api.castellumx.com/pinterest?query={}", urlencoding::encode(&query));
        let client = reqwest::Client::new();

        match client.get(&url).timeout(std::time::Duration::from_secs(15)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let img = json[0]["url"].as_str()
                    .or_else(|| json["results"][0]["image"].as_str())
                    .or_else(|| json["url"].as_str());

                match img {
                    Some(img_url) => ctx.send_image(img_url, &query).await,
                    None => ctx.reply_styled(&format!("ꕢ No encontré imágenes de *{}*.", query)).await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al buscar en Pinterest.").await
        }
    }
}
