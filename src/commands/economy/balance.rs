use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number_large, style_text};

pub struct BalanceCommand;

#[async_trait]
impl Command for BalanceCommand {
    fn triggers(&self) -> &[&str] { &["balance", "bal", "saldo"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Muestra tu balance de monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let economy = &user.economy;
        let coins = economy.coins;
        let bank = economy.bank;
        let total = coins + bank;

        ctx.reply(&style_text(&format!(
            "ꕣ *Balance de Usuario*\n\n\
             ⟡ Billetera: *¥{}* {}\n\
             ⟡ Banco: *¥{}* {}\n\
             ⟡ Total: *¥{}* {}",
            format_number_large(coins), currency,
            format_number_large(bank), currency,
            format_number_large(total), currency
        ))).await
    }
}
