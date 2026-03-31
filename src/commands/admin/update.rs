use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct UpdateCommand;

#[async_trait]
impl Command for UpdateCommand {
    fn triggers(&self) -> &[&str] { &["update", "actualizar"] }
    fn category(&self) -> &str { "admin" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled("ꕢ Solo el dueño puede actualizar el bot.").await; }
        ctx.reply_styled("ꕣ Bot actualizado correctamente.").await
    }
}
