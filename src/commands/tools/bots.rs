use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct BotsCommand;
#[async_trait]
impl Command for BotsCommand {
    fn triggers(&self) -> &[&str] { &["bots", "botlist"] }
    fn category(&self) -> &str { "tools" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        ctx.reply_styled(
            "ꕣ *ALYA KUJOU BOT*\n\n\
             🤖 Bot principal de este servidor\n\
             📚 +159 comandos disponibles\n\
             🎌 Sistema gacha, economía, IA y más!\n\n\
             _Usa `#help` para ver todos los comandos_"
        ).await
    }
}
