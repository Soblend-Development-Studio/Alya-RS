use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;
use super::kiss::fetch_anime_gif;

pub struct SlapCommand;
#[async_trait]
impl Command for SlapCommand {
    fn triggers(&self) -> &[&str] { &["slap", "bofetada", "cachetear"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let sender = ctx.sender.split('@').next().unwrap_or("?");
        let target = mentions.first().map(|t| format!("a @{}", t.split('@').next().unwrap_or("?"))).unwrap_or_else(|| "al vacío".to_string());
        let gif = fetch_anime_gif("slap").await;
        let caption = format!("👋 *@{}* abofetea {} 👋", sender, target);
        if let Some(url) = gif { ctx.send_image(&url, &caption).await } else { ctx.reply_styled(&caption).await }
    }
}
