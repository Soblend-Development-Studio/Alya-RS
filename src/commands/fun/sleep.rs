use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use super::kiss::fetch_anime_gif;
pub struct SleepCommand;
#[async_trait]
impl Command for SleepCommand {
    fn triggers(&self) -> &[&str] { &["sleep", "dormir", "nap"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let gif = fetch_anime_gif("sleep").await;
        let caption = "😴 Me voy a dormir... buenas noches";
        if let Some(url) = gif { ctx.send_image(&url, caption).await } else { ctx.reply_styled(caption).await }
    }
}
