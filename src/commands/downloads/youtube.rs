use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct YoutubeCommand;

#[async_trait]
impl Command for YoutubeCommand {
    fn triggers(&self) -> &[&str] { &["yt", "youtube", "ytsearch"] }
    fn category(&self) -> &str { "downloads" }
    fn help(&self) -> &str { "Busca en YouTube" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#yt <búsqueda>`").await; }
        let query = ctx.args.join(" ");

        let url = format!("https://api.xteam.xyz/ytsearch?q={}", urlencoding::encode(&query));
        let client = reqwest::Client::new();

        match client.get(&url).timeout(std::time::Duration::from_secs(10)).send().await {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                if let Some(results) = json["result"].as_array().or_else(|| json["items"].as_array()) {
                    let mut msg = format!("▶️ *YouTube: {}*\n\n", query);
                    for (i, item) in results.iter().take(5).enumerate() {
                        let title = item["title"].as_str().unwrap_or("Sin título");
                        let link = item["url"].as_str().or_else(|| item["link"].as_str()).unwrap_or("");
                        msg.push_str(&format!("{}. *{}*\n   {}\n\n", i + 1, title, link));
                    }
                    ctx.reply_styled(&msg).await
                } else {
                    ctx.reply_styled(&format!("ꕢ Sin resultados para *{}*", query)).await
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al buscar en YouTube.").await
        }
    }
}
