use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct WikipediaCommand;

#[async_trait]
impl Command for WikipediaCommand {
    fn triggers(&self) -> &[&str] { &["wiki", "wikipedia"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca información en Wikipedia" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#wiki <búsqueda>`").await; }
        let query = ctx.args.join(" ");

        let url = format!("https://es.wikipedia.org/api/rest_v1/page/summary/{}", urlencoding::encode(&query));
        let client = reqwest::Client::new();

        match client.get(&url).header("User-Agent", "AlyaBot/1.0").timeout(std::time::Duration::from_secs(10)).send().await {
            Ok(r) if r.status().is_success() => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let title = json["title"].as_str().unwrap_or(&query);
                let extract = json["extract"].as_str().unwrap_or("Sin información");
                let url_link = json["content_urls"]["desktop"]["page"].as_str().unwrap_or("");
                let snippet = &extract[..extract.len().min(800)];
                ctx.reply_styled(&format!(
                    "📚 *Wikipedia: {}*\n\n{}\n\n🔗 {}",
                    title, snippet, url_link
                )).await
            }
            _ => ctx.reply_styled(&format!("ꕢ No encontré información sobre *{}*.", query)).await
        }
    }
}
