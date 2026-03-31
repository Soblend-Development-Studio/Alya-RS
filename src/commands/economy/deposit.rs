use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct DepositCommand;

#[async_trait]
impl Command for DepositCommand {
    fn triggers(&self) -> &[&str] { &["deposit", "depositar", "dep"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Deposita monedas en tu banco" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let coins = user.economy.coins;

        let amount = if ctx.args.first().map(|a| a.as_str()) == Some("all") || ctx.args.first().map(|a| a.as_str()) == Some("todo") {
            coins
        } else {
            match ctx.args.first().and_then(|a| a.replace(',', "").parse::<i64>().ok()) {
                Some(n) if n > 0 => n,
                _ => return ctx.reply_styled("ꕢ Uso: `#deposit <cantidad|all>`").await,
            }
        };

        if amount > coins {
            return ctx.reply_styled(&format!(
                "ꕢ No tienes suficientes monedas. Tienes *¥{}*",
                format_number(coins)
            )).await;
        }

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": coins - amount,
            "economy.bank": user.economy.bank + amount
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *DEPÓSITO EXITOSO*\n\n\
             ⟡ Depositado: *¥{}* {}\n\
             ⟡ Nuevo balance banco: *¥{}*",
            format_number(amount), currency,
            format_number(user.economy.bank + amount)
        )).await
    }
}
