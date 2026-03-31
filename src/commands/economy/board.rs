use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number_large, style_text};

pub struct BoardCommand;

#[async_trait]
impl Command for BoardCommand {
    fn triggers(&self) -> &[&str] { &["board", "leaderboard", "top", "ranking"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Muestra el ranking de economía" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let top = ctx.db.get_leaderboard(10).await?;

        if top.is_empty() {
            return ctx.reply_styled("ꕢ No hay usuarios en el ranking aún.").await;
        }

        let mut msg = format!("🏆 *TOP 10 ECONOMÍA* ({})\n\n", currency);
        let medals = ["🥇", "🥈", "🥉"];

        for (i, user) in top.iter().enumerate() {
            let medal = medals.get(i).copied().unwrap_or("⟡");
            let name = user.name.as_deref().unwrap_or(&user.id.split('@').next().unwrap_or("?").to_string()[..]);
            let total = user.economy.coins + user.economy.bank;
            msg.push_str(&format!(
                "{} *{}* — ¥{}\n",
                medal, name, format_number_large(total)
            ));
        }

        ctx.reply_styled(&msg).await
    }
}
