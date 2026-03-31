use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_cooldown, format_time, get_random_int, get_random};

const COOLDOWN_MS: i64 = 30 * 60 * 1000;

const BEG_MSGS: &[&str] = &[
    "un señor con pinta de millonario te lanzó algunas monedas",
    "una viejita te dio dinero del fondo de su bolsa",
    "encontraste monedas en el piso después de mendigar 10 minutos",
    "un turista te dio propina pensando que eras un artista callejero",
    "te dieron el vuelto que sobraba en el autoservicio",
];

const REFUSE_MSGS: &[&str] = &[
    "nadie te hizo caso, ni el perro callejero",
    "te ignoraron completamente",
    "alguien te dio un chicle ya masticado en lugar de dinero",
];

pub struct BegCommand;

#[async_trait]
impl Command for BegCommand {
    fn triggers(&self) -> &[&str] { &["beg", "mendigar", "pedir"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Mendiga para conseguir monedas (no siempre funciona)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let now = chrono::Utc::now().timestamp_millis();
        let cache_key = format!("beg_{}", ctx.sender);

        if ctx.cache.has(&cache_key) {
            return ctx.reply_styled(&format!(
                "ꕢ Ya mendiste hace poco. Espera un momento."
            )).await;
        }

        ctx.cache.set(&cache_key, "1", 1800); // 30 min

        let success = get_random_int(1, 100) <= 60;

        if success {
            let amount = get_random_int(100, 3_000);
            let user = ctx.db.get_user(&ctx.sender).await?;
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": user.economy.coins + amount
            }).await?;

            let msg = get_random(BEG_MSGS).unwrap_or(&"alguien te ayudó");
            ctx.reply_styled(&format!(
                "🙏 *LIMOSNA*\n\n> {}\n\n⟡ Recibiste: *¥{}* {}",
                msg, format_number(amount), currency
            )).await
        } else {
            let msg = get_random(REFUSE_MSGS).unwrap_or(&"nadie te ayudó");
            ctx.reply_styled(&format!("😔 *LIMOSNA FALLIDA*\n\n> {}", msg)).await
        }
    }
}
