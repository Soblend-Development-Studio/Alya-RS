use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_cooldown, format_time, style_text};

const COOLDOWN_MS: i64 = 24 * 60 * 60 * 1000;
const STREAK_LIMIT_MS: i64 = 48 * 60 * 60 * 1000;

pub struct DailyCommand;

#[async_trait]
impl Command for DailyCommand {
    fn triggers(&self) -> &[&str] { &["daily", "diario"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Reclama tu recompensa diaria de monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.is_group {
            let group = ctx.db.get_group(&ctx.chat_id).await?;
            if !group.settings.economy {
                return ctx.reply_styled("ꕢ El sistema de economía está desactivado en este grupo.").await;
            }
        }

        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let last_daily = user.economy.last_daily;
        let cooldown = get_cooldown(last_daily, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Ya reclamaste tu recompensa diaria.\nVuelve en: *{}*",
                format_time(cooldown)
            )).await;
        }

        let time_since_last = now - last_daily;
        let mut streak = user.economy.daily_streak;
        if time_since_last < STREAK_LIMIT_MS && last_daily != 0 {
            streak += 1;
        } else {
            streak = 1;
        }

        let reward = streak as i64 * 10_000;
        let new_coins = user.economy.coins + reward;

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins,
            "economy.lastDaily": now,
            "economy.dailyStreak": streak
        }).await?;

        let mut msg = format!(
            "ꕣ *RECOMPENSA DIARIA*\n\n\
             > Día » ¥{}\n\
             > Recompensa » *¥{}* {}",
            streak,
            format_number(reward),
            currency
        );

        if streak > 1 {
            msg.push_str("\n\n_¡Mantén la racha para ganar más!_");
        } else if last_daily != 0 {
            msg.push_str("\n\n_¡Perdiste tu racha! Vuelve mañana para continuar._");
        }

        ctx.reply_styled(&msg).await
    }
}
