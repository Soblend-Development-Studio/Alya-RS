use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SoraCommand;
#[async_trait]
impl Command for SoraCommand {
    fn triggers(&self) -> &[&str] { &["sora", "imagine", "genimage"] }
    fn category(&self) -> &str { "ai" }
    fn help(&self) -> &str { "Genera imágenes con IA" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#sora <descripción>`").await; }
        let prompt = ctx.args.join(" ");
        ctx.reply_styled("🎨 _Generando imagen..._").await?;

        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.castellumx.com/dalle3")
            .query(&[("prompt", &prompt)])
            .timeout(std::time::Duration::from_secs(60))
            .send().await;

        match resp {
            Ok(r) => {
                let json: serde_json::Value = r.json().await.unwrap_or_default();
                let url = json["url"].as_str().or_else(|| json["image"].as_str());
                match url {
                    Some(u) => ctx.send_image(u, &prompt).await,
                    None => ctx.reply_styled("ꕢ No se pudo generar la imagen.").await,
                }
            }
            Err(_) => ctx.reply_styled("ꕢ Error al generar imagen.").await
        }
    }
}
