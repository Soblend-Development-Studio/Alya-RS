use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use super::kiss::fetch_anime_gif;
pub struct CryCommand;
#[async_trait]
impl Command for CryCommand {
    fn triggers(&self) -> &[&str] { &["cry", "llorar"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let gif = fetch_anime_gif("cry").await;
        let caption = "😭 ¡Estoy llorando!";
        if let Some(url) = gif { ctx.send_image(&url, caption).await } else { ctx.reply_styled(caption).await }
    }
}
