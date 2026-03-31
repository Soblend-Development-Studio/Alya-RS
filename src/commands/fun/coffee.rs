use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;
pub struct CoffeeCommand;
#[async_trait]
impl Command for CoffeeCommand {
    fn triggers(&self) -> &[&str] { &["coffee", "cafe", "café"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let sender = ctx.sender.split('@').next().unwrap_or("?");
        let target = mentions.first().map(|t| format!("a @{}", t.split('@').next().unwrap_or("?"))).unwrap_or_else(|| "al grupo".to_string());
        ctx.reply_styled(&format!("☕ *@{}* le sirve un café {} ☕", sender, target)).await
    }
}
