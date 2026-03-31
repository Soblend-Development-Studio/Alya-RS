use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GiveallCommand;
#[async_trait]
impl Command for GiveallCommand {
    fn triggers(&self) -> &[&str] { &["giveall"] }
    fn category(&self) -> &str { "gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled("ꕢ Solo el dueño puede usar este comando.").await; }
        ctx.reply_styled("ꕢ Función disponible próximamente.").await
    }
}
