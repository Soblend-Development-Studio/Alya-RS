use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
pub struct BoredCommand;
#[async_trait]
impl Command for BoredCommand {
    fn triggers(&self) -> &[&str] { &["bored", "aburrido"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("😪 Estoy muy aburrido/a... alguien hable con migo 😢").await
    }
}
