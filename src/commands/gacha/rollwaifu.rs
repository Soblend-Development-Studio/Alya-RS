use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_cooldown, format_time, style_text};
use crate::utils::helpers::{get_stars, get_rarity_text};

const COOLDOWN_MS: i64 = 10 * 60 * 1000;

pub struct RollwaifuCommand;

#[async_trait]
impl Command for RollwaifuCommand {
    fn triggers(&self) -> &[&str] { &["rollwaifu", "rw"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Gira la ruleta del gacha para obtener un personaje" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let user = ctx.db.get_user(&ctx.sender).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let cooldown = get_cooldown(user.gacha.last_roll, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Debes esperar *{}* para volver a hacer roll.\n\n> _*❐ Cooldown: 10 minutos*_",
                format_time(cooldown)
            )).await;
        }

        let character = match ctx.gacha.get_random_character().await {
            Some(c) => c,
            None => return ctx.reply_styled("ꕢ No hay personajes disponibles.").await,
        };

        let stars = get_stars(character.value);
        let rarity = get_rarity_text(character.value);
        let sell_price = (character.value as f64 * 0.8) as i64;
        let owner_str = character.user.as_deref().map(|u| format!("@{}", u.split('@').next().unwrap_or(""))).unwrap_or_else(|| "Nadie".to_string());

        let mut msg = format!(
            "ꕣ Nombre » *{}*\n\n\
             ➭ Fuente » *{}*\n\
             𖧧 Rareza » *{} {}*\n\
             苳 Valor » *{}*\n\
             ₿ Precio » *{}*\n\
             ♛ Dueño » *{}*\n\n\
             > _*❐ Usa #claim en 60 segundos o se perderá!*_",
            character.name,
            character.source.as_deref().unwrap_or("Desconocido"),
            stars, rarity,
            format_number(character.value),
            format_number(sell_price),
            owner_str
        );

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "gacha.rolled": &character.id,
            "gacha.lastRoll": now
        }).await?;

        if let Some(img_url) = character.img.first() {
            ctx.send_image(img_url, &style_text(&msg)).await?;
        } else {
            ctx.reply_styled(&msg).await?;
        }

        Ok(())
    }
}
