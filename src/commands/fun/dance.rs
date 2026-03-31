use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use super::kiss::fetch_anime_gif;
pub struct DanceCommand;
#[async_trait]
impl Command for DanceCommand {
    fn triggers(&self) -> &[&str] { &["dance", "bailar", "baile"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let sender = ctx.sender.split('@').next().unwrap_or("?");
        let gif = fetch_anime_gif("dance").await;
        let caption = format!("💃 *@{}* está bailando! 🕺", sender);
        if let Some(url) = gif { ctx.send_image(&url, &caption).await } else { ctx.reply_styled(&caption).await }
    }
}
