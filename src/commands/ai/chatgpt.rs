use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct ChatgptCommand;

#[async_trait]
impl Command for ChatgptCommand {
    fn triggers(&self) -> &[&str] { &["gpt", "chatgpt", "ai", "ia"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Habla con ChatGPT (GPT-4)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#gpt <pregunta>`").await;
        }

        let prompt = ctx.args.join(" ");
        ctx.reply_styled("🤖 _Procesando tu pregunta..._").await?;

        let response = call_ai_api("gpt-4o-mini", &prompt).await
            .unwrap_or_else(|_| "ꕢ No pude obtener respuesta del servidor AI.".to_string());

        ctx.reply_styled(&format!("🤖 *ChatGPT*\n\n{}", response)).await
    }
}

async fn call_ai_api(model: &str, prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.kastg.xyz/api/ai/chatgptV4")
        .query(&[("prompt", prompt)])
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?;
    
    let json: serde_json::Value = resp.json().await?;
    let result = json["result"].as_str()
        .or_else(|| json["message"].as_str())
        .or_else(|| json["response"].as_str())
        .unwrap_or("Sin respuesta")
        .to_string();

    Ok(result)
}
