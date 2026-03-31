use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TiktokSearchCommand;
#[async_trait]
impl Command for TiktokSearchCommand {
    fn triggers(&self) -> &[&str] { &["ttsearch", "tiktokedit"] }
    fn category(&self) -> &str { "search" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#ttsearch <término>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("🎵 Buscando en TikTok: *{}*...", q)).await
    }
}
