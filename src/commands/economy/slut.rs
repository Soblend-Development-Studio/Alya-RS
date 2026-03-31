use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_cooldown, format_time, style_text, get_random, get_random_int};

const COOLDOWN_MS: i64 = 3 * 60 * 60 * 1000;

const SERVICES: &[&str] = &[
    "le hiciste un masaje a un señor con espalda tensa",
    "vendiste fotos de tus pies en internet",
    "hiciste un directo de ASMR toda la noche",
    "trabajaste como modelo de manos para anuncios de cremas",
    "vendiste autógrafos haciéndote pasar por famoso",
    "le cantaste canciones de cuna por teléfono a desconocidos",
    "te pagaron por ignorar mensajes de exes ajenos",
    "hiciste de doble de un actor de bajo presupuesto",
];

pub struct SlutCommand;

#[async_trait]
impl Command for SlutCommand {
    fn triggers(&self) -> &[&str] { &["slut", "prostituir"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Gana monedas de forma... creativa (cooldown: 3h)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let cooldown = get_cooldown(user.economy.last_slut, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Estás agotado/a. Vuelve en: *{}*",
                format_time(cooldown)
            )).await;
        }

        let reward = get_random_int(8_000, 40_000);
        let service = get_random(SERVICES).unwrap_or(&"hiciste algo dudoso");
        let new_coins = user.economy.coins + reward;

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins,
            "economy.lastSlut": now
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *SERVICIO ESPECIAL*\n\n> {}\n\n⟡ Ganaste: *¥{}* {}",
            service, format_number(reward), currency
        )).await
    }
}
