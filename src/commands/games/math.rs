use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random_int;

pub struct MathCommand;

#[async_trait]
impl Command for MathCommand {
    fn triggers(&self) -> &[&str] { &["math", "matematica", "calcula"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Resuelve un problema matemático para ganar monedas" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let a = get_random_int(1, 100);
        let b = get_random_int(1, 100);
        let ops = ['+', '-', '*'];
        let op = ops[get_random_int(0, 2) as usize];
        let answer: i64 = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            _ => a + b,
        };

        let cache_key = format!("math_{}", ctx.chat_id);
        ctx.cache.set(&cache_key, &answer.to_string(), 30);

        ctx.reply_styled(&format!(
            "🧮 *MATEMÁTICA*\n\n¿Cuánto es *{} {} {}*?\n\n_Responde en 30 segundos para ganar ¥500!_",
            a, op, b
        )).await
    }
}
