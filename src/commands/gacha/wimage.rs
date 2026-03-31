use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct WimageCommand;

#[async_trait]
impl Command for WimageCommand {
    fn triggers(&self) -> &[&str] { &["wimage", "wimagen"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Muestra una imagen de un personaje" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#wimage <nombre o ID>`").await;
        }
        let query = ctx.args.join(" ");
        let char = ctx.gacha.get_by_name(&query).await
            .or_else(|| None);

        match char {
            Some(c) if !c.img.is_empty() => ctx.send_image(&c.img[0], &c.name).await,
            Some(_) => ctx.reply_styled("ꕢ Este personaje no tiene imagen.").await,
            None => ctx.reply_styled(&format!("ꕢ No encontré el personaje *{}*.", query)).await,
        }
    }
}
