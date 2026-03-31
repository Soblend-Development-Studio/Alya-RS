use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct VisionCommand;
#[async_trait]
impl Command for VisionCommand {
    fn triggers(&self) -> &[&str] { &["vision", "describe", "ocr"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Analiza una imagen con IA" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ Responde a una imagen con `#vision <pregunta>` para analizarla.").await
    }
}
