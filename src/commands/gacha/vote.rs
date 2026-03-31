use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct VoteCommand;
#[async_trait]
impl Command for VoteCommand {
    fn triggers(&self) -> &[&str] { &["vote", "votar"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Vota por el bot para obtener recompensas extra" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled("ꕣ *VOTAR POR EL BOT*\n\n\
            Vota en Top.gg para obtener +5 rolls adicionales!\n\
            🔗 https://top.gg/bot/alya-kujou\n\n\
            _Los votos se registran automáticamente._").await
    }
}
