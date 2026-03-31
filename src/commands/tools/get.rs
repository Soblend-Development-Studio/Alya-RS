use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GetCommand;
#[async_trait]
impl Command for GetCommand {
    fn triggers(&self) -> &[&str] { &["get"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Obtiene contenido de una URL" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#get <url>`").await; }
        let url = &ctx.args[0];
        let client = reqwest::Client::new();
        match client.get(url).timeout(std::time::Duration::from_secs(10)).send().await {
            Ok(r) => {
                let text = r.text().await.unwrap_or_default();
                let preview = &text[..text.len().min(500)];
                ctx.reply_styled(&format!("```\n{}\n```", preview)).await
            }
            Err(_) => ctx.reply_styled("ꕢ No se pudo obtener el contenido.").await
        }
    }
}
