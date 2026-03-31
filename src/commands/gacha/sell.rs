use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct SellCommand;

#[async_trait]
impl Command for SellCommand {
    fn triggers(&self) -> &[&str] { &["sell", "vender"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Vende un personaje de tu colección por monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;

        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#sell <nombre o ID del personaje>`").await;
        }

        let query = ctx.args.join(" ");
        let user = ctx.db.get_user(&ctx.sender).await?;

        let char_idx = user.gacha.characters.iter().position(|c| {
            c.id == query || c.name.to_lowercase() == query.to_lowercase()
        });

        let char_idx = match char_idx {
            Some(i) => i,
            None => return ctx.reply_styled(&format!("ꕢ No tienes el personaje *{}* en tu colección.", query)).await,
        };

        let character = user.gacha.characters[char_idx].clone();
        let sell_price = (character.value as f64 * 0.8) as i64;

        // Remove from collection and add coins
        let mut chars = user.gacha.characters.clone();
        chars.remove(char_idx);

        let chars_bson: Vec<bson::Document> = chars.iter().map(|c| bson::doc! {
            "id": &c.id,
            "name": &c.name,
            "source": c.source.as_deref().unwrap_or(""),
            "value": c.value,
        }).collect();

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "gacha.characters": chars_bson,
            "economy.coins": user.economy.coins + sell_price
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *VENTA EXITOSA*\n\n\
             ♛ Vendiste: *{}*\n\
             💰 Ganaste: *¥{}* {}",
            character.name,
            format_number(sell_price),
            currency
        )).await
    }
}
