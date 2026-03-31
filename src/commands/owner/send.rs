use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SendCommand;
#[async_trait]
impl Command for SendCommand {
    fn triggers(&self) -> &[&str] { &["send", "enviar"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        if ctx.args.len() < 2 { return ctx.reply_styled("ꕢ Uso: `#send <jid> <mensaje>`").await; }
        let target = &ctx.args[0];
        let msg = ctx.args[1..].join(" ");
        ctx.client.send_text(target, &msg).await?;
        ctx.reply_styled("ꕣ Mensaje enviado.").await
    }
}
