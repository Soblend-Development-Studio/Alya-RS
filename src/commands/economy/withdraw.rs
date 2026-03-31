use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct WithdrawCommand;

#[async_trait]
impl Command for WithdrawCommand {
    fn triggers(&self) -> &[&str] { &["withdraw", "retirar", "with"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Retira monedas de tu banco" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let bank = user.economy.bank;

        let amount = if ctx.args.first().map(|a| a.as_str()) == Some("all") || ctx.args.first().map(|a| a.as_str()) == Some("todo") {
            bank
        } else {
            match ctx.args.first().and_then(|a| a.replace(',', "").parse::<i64>().ok()) {
                Some(n) if n > 0 => n,
                _ => return ctx.reply_styled("ꕢ Uso: `#withdraw <cantidad|all>`").await,
            }
        };

        if amount > bank {
            return ctx.reply_styled(&format!(
                "ꕢ No tienes suficiente en el banco. Tienes *¥{}*",
                format_number(bank)
            )).await;
        }

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.bank": bank - amount,
            "economy.coins": user.economy.coins + amount
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *RETIRO EXITOSO*\n\n\
             ⟡ Retirado: *¥{}* {}\n\
             ⟡ Nuevo balance billetera: *¥{}*",
            format_number(amount), currency,
            format_number(user.economy.coins + amount)
        )).await
    }
}
