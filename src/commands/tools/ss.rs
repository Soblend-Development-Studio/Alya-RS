use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SsCommand;

#[async_trait]
impl Command for SsCommand {
    fn triggers(&self) -> &[&str] { &["ss", "screenshot", "captura"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Toma captura de pantalla de una URL" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let url = match ctx.args.first() {
            Some(u) => u.clone(),
            None => return ctx.reply_styled("ꕢ Uso: `#ss <url>`").await,
        };

        ctx.reply_styled("📸 _Tomando captura..._").await?;

        let api_url = format!("https://api.nexoracle.com/misc/web-screenshot?apikey=free_key&url={}",
            urlencoding::encode(&url));

        let client = reqwest::Client::new();
        match client.get(&api_url).timeout(std::time::Duration::from_secs(30)).send().await {
            Ok(r) if r.status().is_success() => {
                ctx.client.send_image(&ctx.chat_id, &api_url, &url).await
            }
            _ => ctx.reply_styled("ꕢ No se pudo tomar la captura.").await
        }
    }
}
