use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use super::kiss::fetch_anime_gif;
pub struct AngryCommand;
#[async_trait]
impl Command for AngryCommand {
    fn triggers(&self) -> &[&str] { &["angry", "enojado", "rage"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let gif = fetch_anime_gif("angry").await;
        let caption = "😡 ¡ESTOY ENOJADO!";
        if let Some(url) = gif { ctx.send_image(&url, caption).await } else { ctx.reply_styled(caption).await }
    }
}
