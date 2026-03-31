use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct TradeCommand;

#[async_trait]
impl Command for TradeCommand {
    fn triggers(&self) -> &[&str] { &["trade", "intercambio"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Intercambia personajes con otro usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled(
            "ꕢ El sistema de intercambio requiere una confirmación de ambos usuarios.\n\
             Por ahora, usa *#dar* para regalar personajes directamente."
        ).await
    }
}
