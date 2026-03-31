use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random;

const DARES: &[&str] = &[
    "Haz 20 flexiones ahora mismo",
    "Escribe un poema de amor para el grupo en 2 minutos",
    "Cambia tu foto de perfil por un meme por 24 horas",
    "Escribe tu mayor vergüenza en el grupo",
    "Habla durante 1 minuto sin parar sobre cualquier tema",
    "Envía una selfie rara al grupo",
    "Escribe con la nariz un mensaje en el chat",
    "Canta 10 segundos de una canción y envía el audio",
    "Envía el último meme que viste",
    "Escribe con la mano que no usas normalmente",
];

const TRUTHS: &[&str] = &[
    "¿Cuál es tu mayor secreto?",
    "¿A quién del grupo le tienes más confianza?",
    "¿Cuál es la cosa más loca que has hecho?",
    "¿Cuánto tiempo llevas sin ducharte?",
    "¿Cuál es tu fear más ridículo?",
    "¿Has mentido hoy? ¿En qué?",
    "¿Cuál es tu crush del grupo (si hay alguno)?",
    "¿Cuál fue tu peor momento?",
    "¿Qué harías con un millón de dólares?",
    "¿A quién seguirías en redes sociales sin que se enteren?",
];

pub struct DareCommand;

#[async_trait]
impl Command for DareCommand {
    fn triggers(&self) -> &[&str] { &["dare", "reto", "verdad"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Verdad o reto aleatorio" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let is_dare = ctx.command == "dare" || ctx.command == "reto";
        let (list, title) = if is_dare {
            (DARES.as_ref(), "🎯 *RETO*")
        } else {
            (TRUTHS.as_ref(), "💬 *VERDAD*")
        };
        let item = get_random(list).unwrap_or(&"Nada disponible");
        ctx.reply_styled(&format!("{}\n\n> {}", title, item)).await
    }
}
