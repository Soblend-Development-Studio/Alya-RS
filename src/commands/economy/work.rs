use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_cooldown, format_time, style_text, get_random};

const COOLDOWN_MS: i64 = 60 * 60 * 1000; // 1 hour

const JOBS: &[&str] = &[
    "un señor te contrató para vender pan en la calle",
    "tu papá te dio dinero para que vayas a comprar un kilo de arroz y te dieron vuelto de más",
    "te encontraste monedas en el suelo",
    "vendiste tu riñón",
    "viste algo que no debías y te sobornaron por quedarte callado",
    "tu vecino te pagó por cuidar a su loro que no para de gritar",
    "hiciste dulces caseros y los vendiste afuera de la escuela",
    "te pagaron por pretender ser el novio/novia falso de alguien",
    "conseguiste trabajo repartiendo volantes disfrazado de pizza",
    "lavaste autos en el semáforo con una esponja sospechosamente sucia",
    "tu tía te dio dinero solo por decirle que estaba más joven",
    "participaste en un experimento raro de la universidad",
    "vendiste tus apuntes de clase a compañeros desesperados",
    "te pagaron por hacer fila en lugar de otra persona",
    "encontraste una billetera y te quedaste con la recompensa",
    "trabajaste como extra en una película que nadie va a ver",
    "tu abuela te pagó por enseñarle a usar WhatsApp por centésima vez",
    "vendiste cosas viejas en un mercado de pulgas",
    "hiciste mandados para los vecinos del edificio",
    "ganaste una apuesta absurda con tus amigos",
    "te pagaron por ser el chofer de alguien muy cansado",
    "vendiste limonada en la puerta de tu casa y solo compró tu mamá",
    "tu primo te pagó por hacerle la tarea de matemáticas",
    "conseguiste trabajo como payaso en fiestas infantiles traumatizantes",
    "te pagaron por aplaudir en un evento súper aburrido",
    "vendiste fotos de tus pies en internet",
    "limpiaste la casa de un señor que tiene 47 gatos",
    "te pagaron por fingir que eres amigo de alguien en redes sociales",
    "trabajaste cargando bolsas en el supermercado",
    "le cortaste el pasto a tu vecino con unas tijeras",
];

pub struct WorkCommand;

#[async_trait]
impl Command for WorkCommand {
    fn triggers(&self) -> &[&str] { &["work", "trabajar", "trabajo"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Trabaja para ganar monedas (cooldown: 1h)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let user = ctx.db.get_user(&ctx.sender).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let cooldown = get_cooldown(user.economy.last_work, COOLDOWN_MS);

        if cooldown > 0 {
            return ctx.reply_styled(&format!(
                "ꕢ Ya trabajaste recientemente.\nVuelve en: *{}*",
                format_time(cooldown)
            )).await;
        }

        let reward = crate::utils::format::get_random_int(5_000, 25_000);
        let new_coins = user.economy.coins + reward;
        let job = get_random(JOBS).unwrap_or(&"trabajaste duro");

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins,
            "economy.lastWork": now
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *TRABAJO*\n\n\
             > {}\n\n\
             ⟡ Ganaste: *¥{}* {}",
            job,
            format_number(reward),
            currency
        )).await
    }
}
