use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GeminiCommand;

#[async_trait]
impl Command for GeminiCommand {
    fn triggers(&self) -> &[&str] { &["gemini"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Habla con Google Gemini" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#gemini <pregunta>`").await;
        }
        let prompt = ctx.args.join(" ");
        ctx.reply_styled("✨ _Consultando a Gemini..._").await?;

        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.castellumx.com/gemini")
            .query(&[("q", &prompt)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await;

        let response = match resp {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                json["result"].as_str()
                    .or_else(|| json["response"].as_str())
                    .unwrap_or("Sin respuesta").to_string()
            }
            Err(_) => "ꕢ Error al conectar con Gemini.".to_string()
        };

        ctx.reply_styled(&format!("✨ *Gemini*\n\n{}", response)).await
    }
}
