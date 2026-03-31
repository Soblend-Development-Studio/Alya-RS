use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
pub struct FuckCommand;
#[async_trait] impl Command for FuckCommand {
    fn triggers(&self) -> &[&str] { &["fuck"] }
    fn category(&self) -> &str { "nsfw" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group { let g = ctx.db.get_group(&ctx.chat_id).await?; if !g.settings.nsfw { return ctx.reply_styled("🔞 NSFW desactivado.").await; } }
        let client = reqwest::Client::new();
        let resp = client.get("https://api.waifu.pics/nsfw/blowjob").send().await;
        match resp { Ok(r) => { let j: serde_json::Value = r.json().await.unwrap_or_default(); if let Some(u) = j["url"].as_str() { ctx.send_image(u, "🔞").await } else { ctx.reply_styled("ꕢ Error").await } } Err(_) => ctx.reply_styled("ꕢ Error").await }
    }
}
