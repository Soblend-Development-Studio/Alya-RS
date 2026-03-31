use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct AinfoCommand;
#[async_trait]
impl Command for AinfoCommand {
    fn triggers(&self) -> &[&str] { &["ainfo"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Información de un anime/fuente en la base de datos gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#ainfo <fuente>`").await;
        }
        let query = ctx.args.join(" ");
        let chars = ctx.gacha.get_by_source(&query).await;
        ctx.reply_styled(&format!(
            "ꕣ *INFO DE FUENTE: {}*\n\n📊 Personajes: *{}*",
            query, chars.len()
        )).await
    }
}
