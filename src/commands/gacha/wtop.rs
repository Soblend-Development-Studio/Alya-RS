use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct WtopCommand;

#[async_trait]
impl Command for WtopCommand {
    fn triggers(&self) -> &[&str] { &["wtop", "waifutop"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Top de coleccionistas por número de personajes" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕢ El ranking de waifus está siendo calculado...").await
    }
}
