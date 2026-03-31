use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct WcowCommand;

#[async_trait]
impl Command for WcowCommand {
    fn triggers(&self) -> &[&str] { &["wcow", "cowgirl"] }
    fn category(&self) -> &str { "gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let chars = ctx.gacha.get_by_source("Cow Girl").await;
        if chars.is_empty() {
            return ctx.reply_styled("ꕢ No hay personajes de tipo cow girl disponibles.").await;
        }
        use crate::utils::format::get_random;
        if let Some(c) = get_random(&chars) {
            if let Some(img) = c.img.first() {
                return ctx.send_image(img, &c.name).await;
            }
        }
        ctx.reply_styled("ꕢ No hay imágenes disponibles.").await
    }
}
