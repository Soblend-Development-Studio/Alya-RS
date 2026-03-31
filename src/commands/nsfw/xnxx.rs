use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct XnxxCommand;
#[async_trait] impl Command for XnxxCommand {
    fn triggers(&self) -> &[&str] { &["xnxx"] }
    fn category(&self) -> &str { "nsfw" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group { let g = ctx.db.get_group(&ctx.chat_id).await?; if !g.settings.nsfw { return ctx.reply_styled("🔞 NSFW desactivado.").await; } }
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#xnxx <búsqueda>`").await; }
        ctx.reply_styled("🔞 _(Función en desarrollo)_").await
    }
}
