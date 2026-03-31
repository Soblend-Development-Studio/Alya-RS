use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn triggers(&self) -> &[&str] { &["ping", "speed"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Verifica la latencia del bot" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let start = std::time::Instant::now();
        let _ = ctx.db.get_user_count().await;
        let elapsed = start.elapsed().as_millis();

        ctx.reply_styled(&format!(
            "🏓 *PONG!*\n\n\
             ⟡ Latencia DB: *{}ms*\n\
             ⟡ Estado: *Online* ✅",
            elapsed
        )).await
    }
}
