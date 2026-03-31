use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct PornvideoCommand;
#[async_trait] impl Command for PornvideoCommand {
    fn triggers(&self) -> &[&str] { &["pornvideo", "xvideo"] }
    fn category(&self) -> &str { "nsfw" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group { let g = ctx.db.get_group(&ctx.chat_id).await?; if !g.settings.nsfw { return ctx.reply_styled("🔞 NSFW desactivado.").await; } }
        ctx.reply_styled("🔞 _(Función en desarrollo)_").await
    }
}
