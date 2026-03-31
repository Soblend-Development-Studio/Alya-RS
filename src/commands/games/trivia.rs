use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random;

const QUESTIONS: &[(&str, &[&str], usize)] = &[
    ("¿Cuántos lados tiene un hexágono?", &["4", "5", "6", "8"], 2),
    ("¿En qué continente está Brasil?", &["Europa", "Asia", "África", "América del Sur"], 3),
    ("¿Cuál es el planeta más grande del sistema solar?", &["Saturno", "Júpiter", "Neptuno", "Urano"], 1),
    ("¿Cuántos colores tiene el arcoíris?", &["5", "6", "7", "8"], 2),
    ("¿Qué idioma se habla en Brasil?", &["Español", "Inglés", "Portugués", "Francés"], 2),
    ("¿Cuál es el océano más grande del mundo?", &["Atlántico", "Índico", "Ártico", "Pacífico"], 3),
];

pub struct TriviaCommand;

#[async_trait]
impl Command for TriviaCommand {
    fn triggers(&self) -> &[&str] { &["trivia"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Pregunta de trivia aleatoria" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let idx = crate::utils::format::get_random_int(0, QUESTIONS.len() as i64 - 1) as usize;
        let (question, options, answer_idx) = QUESTIONS[idx];
        let cache_key = format!("trivia_{}", ctx.chat_id);

        ctx.cache.set(&cache_key, &options[*answer_idx], 60);

        let mut msg = format!("🧩 *TRIVIA*\n\n{}\n\n", question);
        for (i, opt) in options.iter().enumerate() {
            msg.push_str(&format!("{}) {}\n", (b'a' + i as u8) as char, opt));
        }
        msg.push_str("\n_Responde con la letra en 60 segundos!_");

        ctx.reply_styled(&msg).await
    }
}
