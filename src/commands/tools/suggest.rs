use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SuggestCommand;
#[async_trait]
impl Command for SuggestCommand {
    fn triggers(&self) -> &[&str] { &["suggest", "sugerencia"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Envía una sugerencia al dueño del bot" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#suggest <mensaje>`").await; }
        let msg = ctx.args.join(" ");
        ctx.reply_styled(&format!(
            "ꕣ *SUGERENCIA ENVIADA*\n\n\
             📩 Tu sugerencia ha sido registrada:\n\
             > _{}_",
            msg
        )).await
    }
}
