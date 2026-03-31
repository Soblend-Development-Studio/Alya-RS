use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_random_int};

pub struct BlackjackCommand;

fn card_value(card: i64) -> i64 {
    match card {
        1 => 11, // Ace
        11 | 12 | 13 => 10, // Jack, Queen, King
        n => n,
    }
}

fn card_name(card: i64) -> &'static str {
    match card {
        1 => "As",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => "?",
    }
}

fn format_card(card: i64) -> String {
    match card {
        1 => "🂡 As".to_string(),
        11 => "🂫 J".to_string(),
        12 => "🂭 Q".to_string(),
        13 => "🂮 K".to_string(),
        n => format!("🃏 {}", n),
    }
}

#[async_trait]
impl Command for BlackjackCommand {
    fn triggers(&self) -> &[&str] { &["blackjack", "bj", "21"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Juega al blackjack apostando monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;

        let amount: i64 = match ctx.args.first().and_then(|a| a.replace(',', "").parse().ok()) {
            Some(n) if n > 0 => n,
            _ => return ctx.reply_styled("ꕢ Uso: `#bj <cantidad>`").await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < amount {
            return ctx.reply_styled(&format!("ꕢ No tienes *¥{}*.", format_number(amount))).await;
        }

        // Simulate a round
        let player_cards = [get_random_int(1, 13), get_random_int(1, 13)];
        let dealer_cards = [get_random_int(1, 13), get_random_int(1, 13)];

        let player_sum: i64 = player_cards.iter().map(|&c| card_value(c)).sum();
        let dealer_sum: i64 = dealer_cards.iter().map(|&c| card_value(c)).sum();

        let dealer_sum = dealer_sum.min(21);
        let player_sum = player_sum.min(21);

        let player_won = player_sum > dealer_sum || dealer_sum > 21;
        let is_blackjack = player_sum == 21;

        let multiplier = if is_blackjack { 2.5_f64 } else { 2.0_f64 };

        let (new_coins, result_msg) = if player_sum > 21 {
            (user.economy.coins - amount, format!("💥 *¡Te pasaste!* ({}) ¡Perdiste! -¥{}", player_sum, format_number(amount)))
        } else if player_won {
            let win = (amount as f64 * multiplier) as i64;
            let extra = if is_blackjack { " 🃏 ¡BLACKJACK!" } else { "" };
            (user.economy.coins + win, format!("ꕣ *¡GANASTE!*{} (+¥{})", extra, format_number(win)))
        } else {
            (user.economy.coins - amount, format!("ꕢ *¡Perdiste!* (-¥{})", format_number(amount)))
        };

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins
        }).await?;

        ctx.reply_styled(&format!(
            "🃏 *BLACKJACK*\n\n\
             > Tus cartas: {} + {} = *{}*\n\
             > Dealer: {} + {} = *{}*\n\n\
             {}",
            format_card(player_cards[0]), format_card(player_cards[1]), player_sum,
            format_card(dealer_cards[0]), format_card(dealer_cards[1]), dealer_sum,
            result_msg
        )).await
    }
}
