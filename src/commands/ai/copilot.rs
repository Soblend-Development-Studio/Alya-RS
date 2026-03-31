use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct CopilotCommand;
#[async_trait]
impl Command for CopilotCommand {
    fn triggers(&self) -> &[&str] { &["copilot", "bing"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Habla con Microsoft Copilot" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#copilot <pregunta>`").await; }
        let prompt = ctx.args.join(" ");
        ctx.reply_styled("🔵 _Consultando a Copilot..._").await?;
        ctx.reply_styled(&format!("🔵 *Copilot*\n\n_(Respuesta para: {})_\n\n(Servicio en configuración)", prompt)).await
    }
}
