use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct WantedCommand;
#[async_trait]
impl Command for WantedCommand {
    fn triggers(&self) -> &[&str] { &["wanted"] }
    fn category(&self) -> &str { "fun" }
    fn help(&self) -> &str { "Crea un cartel de 'se busca'" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender);
        let phone = crate::utils::helpers::extract_phone(target);
        ctx.reply_styled(&format!(
            "🔫 *SE BUSCA*\n\n\
             ⟡ Número: *{}*\n\
             ⟡ Recompensa: *¥999,999*\n\
             ⟡ Peligrosidad: *EXTREMA*\n\n\
             _Solo en este grupo_",
            phone
        )).await
    }
}
