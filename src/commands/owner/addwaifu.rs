use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct AddwaifuCommand;
#[async_trait] impl Command for AddwaifuCommand {
    fn triggers(&self) -> &[&str] { &["addwaifu", "addchar"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        ctx.reply_styled("ꕣ Para agregar personajes al gacha edita el archivo characters.json y reinicia el bot.").await
    }
}
