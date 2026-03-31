use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct AddrwCommand;
#[async_trait]
impl Command for AddrwCommand {
    fn triggers(&self) -> &[&str] { &["addrw"] }
    fn category(&self) -> &str { "gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner {
            return ctx.reply_styled("ꕢ Solo el dueño puede usar este comando.").await;
        }
        ctx.reply_styled("ꕢ Usa el panel de administración para agregar personajes al gacha.").await
    }
}
