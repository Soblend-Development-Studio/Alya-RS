use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct StickerCommand;

#[async_trait]
impl Command for StickerCommand {
    fn triggers(&self) -> &[&str] { &["sticker", "s", "stiker"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Convierte una imagen en sticker" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled(
            "ꕢ Responde a una imagen o sticker con `#sticker` para convertirlo.\n\n\
             _Función de conversión disponible cuando se adjunta imagen._"
        ).await
    }
}
