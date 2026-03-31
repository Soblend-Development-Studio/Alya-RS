use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_random_int};
use crate::utils::helpers::extract_mentions;

pub struct StealCommand;

#[async_trait]
impl Command for StealCommand {
    fn triggers(&self) -> &[&str] { &["steal", "robar", "rob"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Intenta robar monedas a otro usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let mentions = extract_mentions(&ctx.body);

        if mentions.is_empty() {
            return ctx.reply_styled("ꕢ Debes mencionar a alguien para robarle.\n> Uso: `#steal @usuario`").await;
        }

        let target = &mentions[0];
        if target == &ctx.sender {
            return ctx.reply_styled("ꕢ No puedes robarte a ti mismo.").await;
        }

        let user = ctx.db.get_user(&ctx.sender).await?;
        let target_user = ctx.db.get_user(target).await?;

        if target_user.economy.coins < 1000 {
            return ctx.reply_styled("ꕢ Esa persona es demasiado pobre, no tiene ni para robarte.").await;
        }

        // Check antirobo
        if target_user.antirobo > 0 {
            ctx.db.update_user(target, mongodb::bson::doc! {
                "antirobo": target_user.antirobo - 1
            }).await?;
            return ctx.reply_styled(&format!(
                "ꕢ @{} tiene un escudo anti-robo activo. Tu intento fue bloqueado!",
                target.split('@').next().unwrap_or("")
            )).await;
        }

        let success = get_random_int(1, 100) <= 40; // 40% success

        if success {
            let steal_amount = (target_user.economy.coins as f64 * 0.1) as i64;
            let steal_amount = steal_amount.min(50_000).max(100);

            ctx.db.update_user(target, mongodb::bson::doc! {
                "economy.coins": target_user.economy.coins - steal_amount
            }).await?;
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": user.economy.coins + steal_amount
            }).await?;

            ctx.reply_styled(&format!(
                "ꕣ *ROBO EXITOSO*\n\n\
                 > Le robaste a @{}\n\
                 ⟡ Ganaste: *¥{}* {}",
                target.split('@').next().unwrap_or(""),
                format_number(steal_amount),
                currency
            )).await
        } else {
            let fine = get_random_int(1_000, 10_000).min(user.economy.coins);
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": user.economy.coins - fine
            }).await?;

            ctx.reply_styled(&format!(
                "ꕢ *ROBO FALLIDO*\n\n\
                 > Te atraparon intentando robar a @{}\n\
                 ⟡ Multa: *¥{}* {}",
                target.split('@').next().unwrap_or(""),
                format_number(fine),
                currency
            )).await
        }
    }
}
