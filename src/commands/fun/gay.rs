use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random_int;
use crate::utils::helpers::extract_mentions;

pub struct GayCommand;
#[async_trait]
impl Command for GayCommand {
    fn triggers(&self) -> &[&str] { &["gay"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender);
        let percent = get_random_int(0, 100);
        let bar = "🏳️‍🌈".repeat((percent / 10) as usize);
        ctx.reply_styled(&format!(
            "🏳️‍🌈 *Gay Meter*\n\n@{}\n{}\n*{}%* Gay",
            target.split('@').next().unwrap_or("?"),
            bar,
            percent
        )).await
    }
}
