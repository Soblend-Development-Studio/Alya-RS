use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct AntiroboCommand;

#[async_trait]
impl Command for AntiroboCommand {
    fn triggers(&self) -> &[&str] { &["antirobo"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Activa protección anti-robo de personajes" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let cost = 10_000_i64;
        let user = ctx.db.get_user(&ctx.sender).await?;

        if user.antirobo > 0 {
            return ctx.reply_styled(&format!(
                "ꕣ Ya tienes *{}* escudos anti-robo activos.",
                user.antirobo
            )).await;
        }

        if user.economy.coins < cost {
            return ctx.reply_styled(&format!(
                "ꕢ Necesitas *¥{}* {} para activar el anti-robo.",
                format_number(cost), currency
            )).await;
        }

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": user.economy.coins - cost,
            "antirobo": 3
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *ANTI-ROBO ACTIVADO*\n\n\
             🛡️ Tienes *3* escudos activos.\n\
             Costo: *¥{}* {}",
            format_number(cost), currency
        )).await
    }
}
