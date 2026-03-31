use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_random_int};

pub struct CoinflipCommand;

#[async_trait]
impl Command for CoinflipCommand {
    fn triggers(&self) -> &[&str] { &["coinflip", "cf", "moneda"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Apuesta monedas al cara o cruz" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;

        if ctx.args.len() < 2 {
            return ctx.reply_styled("ꕢ Uso: `#coinflip <cara/cruz> <cantidad>`").await;
        }

        let side = ctx.args[0].to_lowercase();
        if side != "cara" && side != "cruz" && side != "heads" && side != "tails" {
            return ctx.reply_styled("ꕢ Elige *cara* o *cruz*.").await;
        }

        let amount: i64 = match ctx.args[1].replace(',', "").parse() {
            Ok(n) if n > 0 => n,
            _ => return ctx.reply_styled("ꕢ Cantidad inválida.").await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < amount {
            return ctx.reply_styled(&format!("ꕢ No tienes *¥{}*.", format_number(amount))).await;
        }

        let result = if get_random_int(0, 1) == 0 { "cara" } else { "cruz" };
        let won = (side == result) || (side == "heads" && result == "cara") || (side == "tails" && result == "cruz");
        let emoji = if result == "cara" { "🪙" } else { "⚜️" };

        let (new_coins, msg) = if won {
            (user.economy.coins + amount, format!(
                "ꕣ *COINFLIP* {}\n\n> Resultado: *{}*\n> ¡Ganaste! +¥{} {}",
                emoji, result, format_number(amount), currency
            ))
        } else {
            (user.economy.coins - amount, format!(
                "ꕢ *COINFLIP* {}\n\n> Resultado: *{}*\n> ¡Perdiste! -¥{} {}",
                emoji, result, format_number(amount), currency
            ))
        };

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins
        }).await?;

        ctx.reply_styled(&msg).await
    }
}
