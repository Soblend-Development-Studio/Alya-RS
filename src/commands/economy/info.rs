use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};
use crate::utils::helpers::extract_mentions;

pub struct InfoCommand;

#[async_trait]
impl Command for InfoCommand {
    fn triggers(&self) -> &[&str] { &["ecoinfo", "userinfo"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Muestra info económica de un usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender).clone();

        let user = ctx.db.get_user(&target).await?;
        let rank = ctx.db.get_user_rank(&target).await.unwrap_or(-1);

        let name = user.name.as_deref().unwrap_or(target.split('@').next().unwrap_or("?"));

        ctx.reply_styled(&format!(
            "ꕣ *INFO ECONÓMICA*\n\n\
             👤 Usuario: *{}*\n\
             💰 Billetera: *¥{}*\n\
             🏦 Banco: *¥{}*\n\
             💎 Total: *¥{}*\n\
             📊 Rank: *#{}*\n\
             📈 Nivel: *{}*\n\
             ⭐ XP: *{}*",
            name,
            format_number(user.economy.coins),
            format_number(user.economy.bank),
            format_number(user.economy.coins + user.economy.bank),
            rank,
            user.level.lvl,
            format_number(user.level.xp)
        )).await
    }
}
