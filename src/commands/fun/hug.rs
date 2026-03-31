use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

macro_rules! simple_action {
    ($name:ident, $triggers:expr, $action:expr, $emoji:expr) => {
        pub struct $name;
        #[async_trait::async_trait]
        impl crate::commands::Command for $name {
            fn triggers(&self) -> &[&str] { $triggers }
            fn category(&self) -> &str { "fun" }
            async fn execute(&self, ctx: &crate::commands::CommandContext) -> anyhow::Result<()> {
                let mentions = extract_mentions(&ctx.body);
                let sender = ctx.sender.split('@').next().unwrap_or("?");
                let target_str = if let Some(t) = mentions.first() {
                    format!("a @{}", t.split('@').next().unwrap_or("?"))
                } else {
                    "al vacío".to_string()
                };
                ctx.reply_styled(&format!(
                    "{} *@{} {} {}* {}",
                    $emoji, sender, $action, target_str, $emoji
                )).await
            }
        }
    };
}

pub struct HugCommand;

#[async_trait]
impl Command for HugCommand {
    fn triggers(&self) -> &[&str] { &["hug", "abrazar", "abrazo"] }
    fn category(&self) -> &str { "fun" }
    fn help(&self) -> &str { "Abraza a alguien" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let sender = ctx.sender.split('@').next().unwrap_or("?");

        let api_url = "https://api.waifu.pics/sfw/hug";
        let client = reqwest::Client::new();
        let gif = match client.get(api_url).send().await {
            Ok(r) => r.json::<serde_json::Value>().await.ok()
                .and_then(|j| j["url"].as_str().map(|s| s.to_string())),
            Err(_) => None,
        };

        let target_str = if let Some(t) = mentions.first() {
            format!("a @{}", t.split('@').next().unwrap_or("?"))
        } else {
            "al grupo".to_string()
        };

        let caption = format!("🤗 *@{}* le da un abrazo {} 🤗", sender, target_str);

        if let Some(url) = gif {
            ctx.send_image(&url, &caption).await
        } else {
            ctx.reply_styled(&caption).await
        }
    }
}
