use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_cooldown, format_time, style_text, get_random, get_random_int};

const COOLDOWN_MS: i64 = 2 * 60 * 60 * 1000; // 2 hours
const SUCCESS_RATE: i64 = 60; // 60% success

const CRIMES: &[&str] = &[
    "robaste a un turista confundido",
    "vendiste DVDs piratas en el mercado",
    "hackeaste la WiFi de tu vecino",
    "hiciste trampa en un torneo de ajedrez online",
    "vendiste un auto que no era tuyo",
    "falsificaste entradas para un concierto",
    "robaste el almuerzo de tus colegas de la nevera de la oficina",
    "manipulaste la máquina de chicles del supermercado",
    "vendiste pollo sin certificado sanitario",
    "hiciste una estafa de prívate en la playa",
];

const FAIL_MSGS: &[&str] = &[
    "te atrapó la policía",
    "tu cómplice te traicionó",
    "dejaste tus huellas en todos lados",
    "un vecino te filmó con su teléfono",
    "tropezaste al huir y te cayó encima todo",
];

pub struct CrimeCommand;

#[async_trait]
impl Command for CrimeCommand {
    fn triggers(&self) -> &[&str] { &["crime", "crimen"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Comete un crimen para ganar (o perder) monedas (cooldown: 2h)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let cooldown = get_cooldown(user.economy.last_crime, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Ya cometiste un crimen recientemente.\nVuelve en: *{}*",
                format_time(cooldown)
            )).await;
        }

        let success = get_random_int(1, 100) <= SUCCESS_RATE;
        let amount = get_random_int(10_000, 50_000);

        let (new_coins, msg) = if success {
            let crime = get_random(CRIMES).unwrap_or(&"cometiste un crimen");
            let coins = user.economy.coins + amount;
            (coins, format!(
                "ꕣ *CRIMEN EXITOSO*\n\n> {}\n\n⟡ Ganaste: *¥{}* {}",
                crime, format_number(amount), currency
            ))
        } else {
            let fail = get_random(FAIL_MSGS).unwrap_or(&"te atraparon");
            let fine = get_random_int(5_000, 25_000).min(user.economy.coins);
            let coins = user.economy.coins - fine;
            (coins, format!(
                "ꕢ *CRIMEN FALLIDO*\n\n> {}\n\n⟡ Perdiste: *¥{}* {}",
                fail, format_number(fine), currency
            ))
        };

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins,
            "economy.lastCrime": now
        }).await?;

        ctx.reply_styled(&msg).await
    }
}
