use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::services::level::LevelService;
use crate::utils::format::{format_number, style_text};
use crate::utils::helpers::extract_mentions;

pub struct ProfileCommand;

#[async_trait]
impl Command for ProfileCommand {
    fn triggers(&self) -> &[&str] { &["profile", "perfil", "level", "nivel", "rank"] }
    fn category(&self) -> &str { "level" }
    fn help(&self) -> &str { "Muestra tu perfil y nivel" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender).clone();

        let user = ctx.db.get_user(&target).await?;
        let rank = ctx.db.get_user_rank(&target).await.unwrap_or(-1);
        let xp_needed = LevelService::xp_for_level(user.level.lvl);
        let progress = if xp_needed > 0 { (user.level.xp * 10) / xp_needed } else { 10 };
        let bar = "█".repeat(progress as usize) + &"░".repeat(10 - progress as usize);
        let name = user.name.as_deref().unwrap_or(target.split('@').next().unwrap_or("?"));

        ctx.reply_styled(&format!(
            "ꕣ *PERFIL*\n\n\
             👤 *{}*\n\
             📊 Rank: *#{}*\n\
             ⭐ Nivel: *{}*\n\
             ✨ XP: *{}/{}*\n\
             [{}]\n\n\
             💰 Monedas: *¥{}*\n\
             🏦 Banco: *¥{}*\n\
             🎌 Waifus: *{}*",
            name, rank,
            user.level.lvl,
            format_number(user.level.xp), format_number(xp_needed),
            bar,
            format_number(user.economy.coins),
            format_number(user.economy.bank),
            user.gacha.characters.len()
        )).await
    }
}
