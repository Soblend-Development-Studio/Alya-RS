use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TextproCommand;
#[async_trait]
impl Command for TextproCommand {
    fn triggers(&self) -> &[&str] { &["textpro", "texto"] }
    fn category(&self) -> &str { "fun" }
    fn help(&self) -> &str { "Crea imágenes de texto con efectos" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#textpro <texto>`").await; }
        let text = ctx.args.join(" ");
        let url = format!("https://api.textpro.me/all/neon-text.php?text={}", urlencoding::encode(&text));
        ctx.send_image(&url, &text).await
    }
}
