use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct ListwaifuCommand;

#[async_trait]
impl Command for ListwaifuCommand {
    fn triggers(&self) -> &[&str] { &["listwaifu", "listw"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Lista todos los personajes disponibles" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let total = ctx.gacha.get_total().await;
        let query = ctx.args.join(" ");

        if query.is_empty() {
            return ctx.reply_styled(&format!(
                "ꕣ *BASE DE DATOS GACHA*\n\n\
                 📊 Total de personajes: *{}*\n\n\
                 > Usa `#listwaifu <fuente>` para filtrar por fuente.",
                total
            )).await;
        }

        let results = ctx.gacha.get_by_source(&query).await;
        if results.is_empty() {
            return ctx.reply_styled(&format!("ꕢ No se encontraron personajes de *{}*.", query)).await;
        }

        let mut msg = format!("ꕣ *Personajes de {}*\n\n", query);
        for (i, c) in results.iter().take(20).enumerate() {
            msg.push_str(&format!("{}. **{}** (`{}`)\n", i + 1, c.name, c.id));
        }
        if results.len() > 20 {
            msg.push_str(&format!("\n_... y {} más_", results.len() - 20));
        }
        ctx.reply_styled(&msg).await
    }
}
