use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_random_int};

pub struct RouletteCommand;

#[async_trait]
impl Command for RouletteCommand {
    fn triggers(&self) -> &[&str] { &["roulette", "ruleta"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Juega a la ruleta apostando monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;

        if ctx.args.is_empty() {
            return ctx.reply_styled(
                "ꕢ Uso: `#ruleta <cantidad>`\n\n\
                 Colores: rojo (1-18) 🔴, negro (19-36) ⚫\n\
                 Verde (0): x14\n\
                 Acertar número: x36"
            ).await;
        }

        let amount: i64 = match ctx.args[0].replace(',', "").parse() {
            Ok(n) if n > 0 => n,
            _ => return ctx.reply_styled("ꕢ Cantidad inválida.").await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < amount {
            return ctx.reply_styled(&format!("ꕢ No tienes *¥{}*.", format_number(amount))).await;
        }

        let spin = get_random_int(0, 36);
        let (result_emoji, result_name, multiplier): (&str, &str, f64) = if spin == 0 {
            ("🟢", "verde", 14.0)
        } else if spin <= 18 {
            ("🔴", "rojo", 2.0)
        } else {
            ("⚫", "negro", 2.0)
        };

        // For simplicity, player always bets on their lucky number or color
        // Here we give a 50% chance on color bet
        let won = get_random_int(1, 100) <= 50;

        let (new_coins, msg) = if won {
            let winnings = (amount as f64 * multiplier) as i64;
            (user.economy.coins + winnings, format!(
                "ꕣ *RULETA* {}\n\n\
                 > Número: *{}*\n\
                 > Color: *{}*\n\
                 > ¡Ganaste! +¥{} {}",
                result_emoji, spin, result_name,
                format_number(winnings), currency
            ))
        } else {
            (user.economy.coins - amount, format!(
                "ꕢ *RULETA* {}\n\n\
                 > Número: *{}*\n\
                 > Color: *{}*\n\
                 > ¡Perdiste! -¥{} {}",
                result_emoji, spin, result_name,
                format_number(amount), currency
            ))
        };

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins
        }).await?;

        ctx.reply_styled(&msg).await
    }
}
