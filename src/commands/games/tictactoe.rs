use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TictactoeCommand;
#[async_trait]
impl Command for TictactoeCommand {
    fn triggers(&self) -> &[&str] { &["ttt", "tictactoe", "gato"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Juega al gato/tic-tac-toe contra el bot" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled(
            "❌⭕ *TIC TAC TOE*\n\n\
             El juego de gato está en desarrollo.\n\
             Próximamente disponible!"
        ).await
    }
}
