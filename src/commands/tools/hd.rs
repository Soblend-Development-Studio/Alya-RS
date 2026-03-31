use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct HdCommand;
#[async_trait]
impl Command for HdCommand {
    fn triggers(&self) -> &[&str] { &["hd", "enhance"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Mejora la resolución de una imagen con IA" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ Responde a una imagen con `#hd` para mejorar su calidad.").await
    }
}
