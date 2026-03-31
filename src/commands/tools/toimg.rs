use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct ToimgCommand;
#[async_trait]
impl Command for ToimgCommand {
    fn triggers(&self) -> &[&str] { &["toimg", "toimage"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Convierte un sticker en imagen" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ Responde a un sticker con `#toimg` para convertirlo en imagen.").await
    }
}
