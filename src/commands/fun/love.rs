use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;
pub struct LoveCommand;
#[async_trait]
impl Command for LoveCommand {
    fn triggers(&self) -> &[&str] { &["love", "amor", "te amo"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().map(|t| format!("@{}", t.split('@').next().unwrap_or("?"))).unwrap_or_else(|| "a todos".to_string());
        ctx.reply_styled(&format!("❤️ *@{}* le manda amor a {} ❤️", ctx.sender.split('@').next().unwrap_or("?"), target)).await
    }
}
