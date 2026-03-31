use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct PajawomanCommand;
#[async_trait] impl Command for PajawomanCommand {
    fn triggers(&self) -> &[&str] { &["pajawoman"] }
    fn category(&self) -> &str { "nsfw" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group { let g = ctx.db.get_group(&ctx.chat_id).await?; if !g.settings.nsfw { return ctx.reply_styled("🔞 NSFW desactivado.").await; } }
        ctx.reply_styled("🔞 _(Contenido disponible próximamente)_").await
    }
}
