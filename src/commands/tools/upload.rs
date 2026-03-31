use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct UploadCommand;
#[async_trait]
impl Command for UploadCommand {
    fn triggers(&self) -> &[&str] { &["upload", "subir"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Sube un archivo a internet y devuelve la URL" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ Responde a un archivo con `#upload` para subirlo.").await
    }
}
