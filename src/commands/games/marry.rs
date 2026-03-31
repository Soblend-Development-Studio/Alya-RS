use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct MarryCommand;

#[async_trait]
impl Command for MarryCommand {
    fn triggers(&self) -> &[&str] { &["marry", "casar", "propuesta"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Proponte matrimonio a alguien" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() {
            return ctx.reply_styled("ꕢ Menciona a alguien para proponerle matrimonio.").await;
        }
        let target = &mentions[0];
        ctx.reply_styled(&format!(
            "💍 *PROPUESTA DE MATRIMONIO*\n\n\
             @{} le propone matrimonio a @{}!\n\n\
             _¿Aceptas? (función en desarrollo)_",
            ctx.sender.split('@').next().unwrap_or("?"),
            target.split('@').next().unwrap_or("?")
        )).await
    }
}
