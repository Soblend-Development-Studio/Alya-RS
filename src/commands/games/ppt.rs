use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random;

const OPTIONS: &[&str] = &["piedra", "papel", "tijera"];

pub struct PptCommand;

#[async_trait]
impl Command for PptCommand {
    fn triggers(&self) -> &[&str] { &["ppt", "piedra", "rps"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Juega piedra, papel, tijera contra el bot" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let choice = match ctx.args.first() {
            Some(c) => c.to_lowercase(),
            None => return ctx.reply_styled("ꕢ Uso: `#ppt <piedra/papel/tijera>`").await,
        };

        if !["piedra", "papel", "tijera", "rock", "paper", "scissors"].contains(&choice.as_str()) {
            return ctx.reply_styled("ꕢ Elige *piedra*, *papel* o *tijera*.").await;
        }

        let bot_choice = get_random(OPTIONS).unwrap_or(&"piedra");
        let player = if choice == "rock" || choice == "piedra" { "piedra" }
                     else if choice == "paper" || choice == "papel" { "papel" }
                     else { "tijera" };

        let result = match (player, bot_choice) {
            ("piedra", "tijera") | ("papel", "piedra") | ("tijera", "papel") => "¡Ganaste! 🎉",
            (a, b) if a == b => "¡Empate! 🤝",
            _ => "¡Perdiste! 😔",
        };

        ctx.reply_styled(&format!(
            "✊ *PIEDRA, PAPEL O TIJERA*\n\n\
             > Tú: *{}*\n\
             > Bot: *{}*\n\n\
             {}",
            player, bot_choice, result
        )).await
    }
}
