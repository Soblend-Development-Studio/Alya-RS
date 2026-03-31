use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};
use crate::utils::helpers::extract_mentions;

pub struct GivecoinsCommand;

#[async_trait]
impl Command for GivecoinsCommand {
    fn triggers(&self) -> &[&str] { &["givecoins", "give", "transferir", "donar"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Transfiere monedas a otro usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let mentions = extract_mentions(&ctx.body);

        if mentions.is_empty() || ctx.args.len() < 2 {
            return ctx.reply_styled("ꕢ Uso: `#give @usuario <cantidad>`").await;
        }

        let target = &mentions[0];
        if target == &ctx.sender {
            return ctx.reply_styled("ꕢ No puedes transferirte monedas a ti mismo.").await;
        }

        let amount: i64 = match ctx.args.iter().find(|a| a.chars().all(|c| c.is_ascii_digit() || c == ',')).and_then(|a| a.replace(',', "").parse().ok()) {
            Some(n) if n > 0 => n,
            _ => return ctx.reply_styled("ꕢ Cantidad inválida.").await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < amount {
            return ctx.reply_styled(&format!(
                "ꕢ No tienes *¥{}*. Solo tienes *¥{}*.",
                format_number(amount),
                format_number(user.economy.coins)
            )).await;
        }

        let target_user = ctx.db.get_user(target).await?;

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": user.economy.coins - amount
        }).await?;
        ctx.db.update_user(target, mongodb::bson::doc! {
            "economy.coins": target_user.economy.coins + amount
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *TRANSFERENCIA EXITOSA*\n\n\
             ⟡ Para: @{}\n\
             ⟡ Cantidad: *¥{}* {}",
            target.split('@').next().unwrap_or(""),
            format_number(amount),
            currency
        )).await
    }
}
