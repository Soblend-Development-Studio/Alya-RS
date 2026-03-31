use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;
use crate::utils::helpers::extract_mentions;

pub struct HaremCommand;

#[async_trait]
impl Command for HaremCommand {
    fn triggers(&self) -> &[&str] { &["harem", "coleccion", "mis-waifus"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Muestra tu colección de personajes" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender).clone();

        let user = ctx.db.get_user(&target).await?;
        let chars = &user.gacha.characters;

        if chars.is_empty() {
            return ctx.reply_styled("ꕢ No tienes personajes en tu colección. Usa *#rollwaifu* para conseguir uno!").await;
        }

        let name = user.name.as_deref().unwrap_or(target.split('@').next().unwrap_or("?"));
        let page: usize = ctx.args.first().and_then(|a| a.parse().ok()).unwrap_or(1);
        let per_page = 15;
        let total = chars.len();
        let pages = (total + per_page - 1) / per_page;
        let page = page.min(pages).max(1);
        let start = (page - 1) * per_page;
        let end = (start + per_page).min(total);

        let mut msg = format!(
            "♛ *Colección de {}*\n\
             📊 Total: {} personajes | Pág. {}/{}\n\n",
            name, total, page, pages
        );

        for (i, c) in chars[start..end].iter().enumerate() {
            msg.push_str(&format!(
                "{}. **{}** _({})\n",
                start + i + 1,
                c.name,
                c.source.as_deref().unwrap_or("?")
            ));
        }

        if pages > 1 {
            msg.push_str(&format!("\n> Usa `#harem {}` para ver la siguiente página", page + 1));
        }

        ctx.reply_styled(&msg).await
    }
}
