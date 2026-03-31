use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TtuserCommand;
#[async_trait]
impl Command for TtuserCommand {
    fn triggers(&self) -> &[&str] { &["ttuser", "tiktokuser"] }
    fn category(&self) -> &str { "search" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#ttuser <@usuario>`").await; }
        let user = &ctx.args[0];
        ctx.reply_styled(&format!("🎵 Buscando perfil de TikTok: *{}*...\n\n_Función en desarrollo_", user)).await
    }
}
