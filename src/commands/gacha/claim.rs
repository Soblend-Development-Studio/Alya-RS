use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{get_cooldown, format_time, style_text};

const COOLDOWN_MS: i64 = 30 * 60 * 1000;
const CLAIM_WINDOW_MS: i64 = 60 * 1000;

pub struct ClaimCommand;

#[async_trait]
impl Command for ClaimCommand {
    fn triggers(&self) -> &[&str] { &["claim", "c"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Reclama el personaje que salió en el roll" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let user = ctx.db.get_user(&ctx.sender).await?;
        let cooldown = get_cooldown(user.gacha.last_claim, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Ya reclamaste un personaje recientemente.\nVuelve en: {}",
                format_time(cooldown)
            )).await;
        }

        let rolled_id = match &user.gacha.rolled {
            Some(id) => id.clone(),
            None => return ctx.reply_styled("ꕢ Primero debes girar la ruleta con *#rollwaifu* (#rw) para obtener un personaje.").await,
        };

        let now = chrono::Utc::now().timestamp_millis();
        let time_since_roll = now - user.gacha.last_roll;

        if time_since_roll > CLAIM_WINDOW_MS {
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "gacha.rolled": bson::Bson::Null
            }).await?;
            return ctx.reply_styled("ꕢ ¡Demasiado tarde! El personaje escapó porque no lo reclamaste en 60 segundos.").await;
        }

        let character = match ctx.gacha.get_by_id(&rolled_id).await {
            Some(c) => c,
            None => {
                ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                    "gacha.rolled": bson::Bson::Null
                }).await?;
                return ctx.reply_styled("ꕢ El personaje que giraste ya no está disponible.").await;
            }
        };

        // Add character to user's collection
        let char_doc = bson::doc! {
            "id": &character.id,
            "name": &character.name,
            "source": character.source.as_deref().unwrap_or(""),
            "value": character.value,
            "img": character.img.first().unwrap_or(&String::new())
        };

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "$push": { "gacha.characters": char_doc },
            "gacha.rolled": bson::Bson::Null,
            "gacha.lastClaim": now
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *¡PERSONAJE RECLAMADO!*\n\n\
             ♛ *{}* es ahora parte de tu colección!\n\
             ➭ Fuente: *{}*",
            character.name,
            character.source.as_deref().unwrap_or("Desconocido")
        )).await
    }
}
