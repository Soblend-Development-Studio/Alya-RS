use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct WvideoCommand;
#[async_trait]
impl Command for WvideoCommand {
    fn triggers(&self) -> &[&str] { &["wvideo"] }
    fn category(&self) -> &str { "gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ El comando de video de personaje no está disponible aún.").await
    }
}
