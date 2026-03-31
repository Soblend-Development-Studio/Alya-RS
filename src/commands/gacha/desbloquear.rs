use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct DesbloquearCommand;

#[async_trait]
impl Command for DesbloquearCommand {
    fn triggers(&self) -> &[&str] { &["desbloquear", "unlock"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Desbloquea slots extra para tu colección gacha" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let cost = 50_000_i64;
        let user = ctx.db.get_user(&ctx.sender).await?;

        if user.economy.coins < cost {
            return ctx.reply_styled(&format!(
                "ꕢ Necesitas *¥{}* {} para desbloquear un slot extra.",
                format_number(cost), currency
            )).await;
        }

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": user.economy.coins - cost,
            "desbloqueo": user.desbloqueo + 1
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *SLOT DESBLOQUEADO*\n\n\
             🔓 Tienes *{}* slots adicionales desbloqueados.\n\
             Costo: *¥{}* {}",
            user.desbloqueo + 1,
            format_number(cost), currency
        )).await
    }
}
