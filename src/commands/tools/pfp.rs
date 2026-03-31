use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct PfpCommand;

#[async_trait]
impl Command for PfpCommand {
    fn triggers(&self) -> &[&str] { &["pfp", "foto", "avatar"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Obtiene la foto de perfil de un usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender);

        ctx.reply_styled(&format!(
            "ꕣ Foto de perfil de @{}\n\n_Función de obtención de foto disponible próximamente._",
            target.split('@').next().unwrap_or("?")
        )).await
    }
}
