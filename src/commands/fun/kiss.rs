use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct KissCommand;
#[async_trait]
impl Command for KissCommand {
    fn triggers(&self) -> &[&str] { &["kiss", "beso", "besar"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let sender = ctx.sender.split('@').next().unwrap_or("?");
        let target = mentions.first().map(|t| format!("a @{}", t.split('@').next().unwrap_or("?"))).unwrap_or_else(|| "al vacío".to_string());
        let gif = fetch_anime_gif("kiss").await;
        let caption = format!("💋 *@{}* besa {} 💋", sender, target);
        if let Some(url) = gif { ctx.send_image(&url, &caption).await } else { ctx.reply_styled(&caption).await }
    }
}

pub async fn fetch_anime_gif(action: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.waifu.pics/sfw/{}", action);
    client.get(&url).send().await.ok()?
        .json::<serde_json::Value>().await.ok()?
        ["url"].as_str().map(|s| s.to_string())
}
