use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_random_int};

const SYMBOLS: &[&str] = &["🍎", "🍋", "🍇", "⭐", "💎", "🔔", "🍒", "7️⃣"];

pub struct SlotCommand;

#[async_trait]
impl Command for SlotCommand {
    fn triggers(&self) -> &[&str] { &["slot", "slots", "maquina"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Juega a las tragamonedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let amount: i64 = match ctx.args.first().and_then(|a| a.replace(',', "").parse().ok()) {
            Some(n) if n > 0 => n,
            _ => return ctx.reply_styled("ꕢ Uso: `#slot <cantidad>`").await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < amount {
            return ctx.reply_styled(&format!("ꕢ No tienes *¥{}*.", format_number(amount))).await;
        }

        let s1 = SYMBOLS[get_random_int(0, SYMBOLS.len() as i64 - 1) as usize];
        let s2 = SYMBOLS[get_random_int(0, SYMBOLS.len() as i64 - 1) as usize];
        let s3 = SYMBOLS[get_random_int(0, SYMBOLS.len() as i64 - 1) as usize];

        let (multiplier, label) = if s1 == s2 && s2 == s3 {
            if s1 == "💎" { (10.0, "💎 ¡JACKPOT! 💎") }
            else if s1 == "7️⃣" { (5.0, "7️⃣ ¡TRIPLE 7! 🎰") }
            else { (3.0, "¡TRES IGUALES! 🎉") }
        } else if s1 == s2 || s2 == s3 || s1 == s3 {
            (1.5, "Dos iguales!")
        } else {
            (0.0, "Sin suerte 😢")
        };

        let won = if multiplier > 0.0 {
            let win = (amount as f64 * multiplier) as i64;
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": user.economy.coins + win - amount
            }).await?;
            format!("⟡ Ganaste: *+¥{}* {}", format_number(win), currency)
        } else {
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": user.economy.coins - amount
            }).await?;
            format!("⟡ Perdiste: *-¥{}* {}", format_number(amount), currency)
        };

        ctx.reply_styled(&format!(
            "🎰 *TRAGAMONEDAS*\n\n\
             ╔ {} ═ {} ═ {} ╗\n\n\
             {}\n\n{}",
            s1, s2, s3, label, won
        )).await
    }
}
