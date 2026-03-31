use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};
use crate::utils::helpers::extract_mentions;

pub struct SetcoinsCommand;

#[async_trait]
impl Command for SetcoinsCommand {
    fn triggers(&self) -> &[&str] { &["setcoins", "setmonedas"] }
    fn category(&self) -> &str { "economy" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner {
            return ctx.reply_styled("ꕢ Solo el dueño puede usar este comando.").await;
        }

        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() || ctx.args.len() < 2 {
            return ctx.reply_styled("ꕢ Uso: `#setcoins @usuario <cantidad>`").await;
        }

        let target = &mentions[0];
        let amount: i64 = match ctx.args.iter().find(|a| a.chars().all(|c| c.is_ascii_digit())).and_then(|a| a.parse().ok()) {
            Some(n) => n,
            _ => return ctx.reply_styled("ꕢ Cantidad inválida.").await,
        };

        ctx.db.update_user(target, mongodb::bson::doc! {
            "economy.coins": amount
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ Se establecieron *¥{}* monedas para @{}",
            format_number(amount),
            target.split('@').next().unwrap_or("")
        )).await
    }
}
