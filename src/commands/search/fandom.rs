use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct FandomCommand;
#[async_trait]
impl Command for FandomCommand {
    fn triggers(&self) -> &[&str] { &["fandom", "wiki2"] }
    fn category(&self) -> &str { "search" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#fandom <personaje>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("📖 Buscando en Fandom: *{}*...", q)).await
    }
}
