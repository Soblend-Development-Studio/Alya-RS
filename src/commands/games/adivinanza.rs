use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random_int;

const RIDDLES: &[(&str, &str)] = &[
    ("Soy alto cuando joven, y bajo cuando viejo. ¿Qué soy?", "una vela"),
    ("Cuantas más me quitas, más grande soy. ¿Qué soy?", "un hoyo"),
    ("Tengo ciudades, pero no casas. Tengo montañas, pero sin árboles. Tengo agua, pero sin peces. ¿Qué soy?", "un mapa"),
    ("¿Qué tiene dientes pero no puede morder?", "un peine"),
    ("¿Qué se puede romper sin tocarlo?", "el silencio"),
    ("Vuelo sin alas, lloro sin ojos. ¿Qué soy?", "una nube"),
];

pub struct AdivinanzaCommand;

#[async_trait]
impl Command for AdivinanzaCommand {
    fn triggers(&self) -> &[&str] { &["adivinanza", "riddle"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Adivinanza aleatoria" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let idx = get_random_int(0, RIDDLES.len() as i64 - 1) as usize;
        let (riddle, answer) = RIDDLES[idx];
        let cache_key = format!("riddle_{}", ctx.chat_id);
        ctx.cache.set(&cache_key, answer, 60);

        ctx.reply_styled(&format!(
            "🧩 *ADIVINANZA*\n\n{}\n\n_Responde en 60 segundos!_",
            riddle
        )).await
    }
}
