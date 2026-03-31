use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct ClaudeCommand;
#[async_trait]
impl Command for ClaudeCommand {
    fn triggers(&self) -> &[&str] { &["claude", "sonnet"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Habla con Claude AI de Anthropic" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#claude <pregunta>`").await; }
        let prompt = ctx.args.join(" ");
        ctx.reply_styled("🎭 _Consultando a Claude..._").await?;
        ctx.reply_styled(&format!("🎭 *Claude*\n\n_Respuesta para: {}_\n\n(Servicio en configuración)", prompt)).await
    }
}
