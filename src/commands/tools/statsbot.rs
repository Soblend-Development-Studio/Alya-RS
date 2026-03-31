use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct StatsbotCommand;

#[async_trait]
impl Command for StatsbotCommand {
    fn triggers(&self) -> &[&str] { &["statsbot", "stats", "estadisticas"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Estadísticas del bot" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let total_users = ctx.db.get_user_count().await.unwrap_or(0);
        let uptime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        ctx.reply_styled(&format!(
            "📊 *ESTADÍSTICAS DEL BOT*\n\n\
             👥 Usuarios: *{}*\n\
             📚 Comandos: *159+*\n\
             🎭 Personajes gacha: *{}*\n\
             ✅ Estado: *Online*",
            total_users,
            ctx.gacha.get_total().await
        )).await
    }
}
