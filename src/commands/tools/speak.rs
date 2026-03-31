use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SpeakCommand;

#[async_trait]
impl Command for SpeakCommand {
    fn triggers(&self) -> &[&str] { &["speak", "tts", "voz"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Convierte texto a voz" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#speak <texto>`").await;
        }
        let text = ctx.args.join(" ");

        let client = reqwest::Client::new();
        let url = format!("https://api.xteam.xyz/tts?text={}&lang=es",
            urlencoding::encode(&text));

        match client.get(&url).timeout(std::time::Duration::from_secs(15)).send().await {
            Ok(r) if r.status().is_success() => {
                let bytes = r.bytes().await?;
                ctx.client.send_audio(&ctx.chat_id, &url).await?;
            }
            _ => {
                ctx.reply_styled("ꕢ No se pudo generar el audio.").await?;
            }
        }
        Ok(())
    }
}
