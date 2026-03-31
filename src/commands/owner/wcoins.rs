use async_trait::async_trait; use anyhow::Result; use crate::commands::{Command, CommandContext};
use crate::utils::format::format_number;
pub struct WcoinsCommand;
#[async_trait] impl Command for WcoinsCommand {
    fn triggers(&self) -> &[&str] { &["wcoins"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        if ctx.args.len() < 2 { return ctx.reply_styled("ꕢ Uso: `#wcoins <jid> <cantidad>`").await; }
        let target = &ctx.args[0];
        let amount: i64 = match ctx.args[1].parse() { Ok(n) => n, Err(_) => return ctx.reply_styled("ꕢ Cantidad inválida").await };
        let user = ctx.db.get_user(target).await?;
        ctx.db.update_user(target, mongodb::bson::doc! { "economy.coins": user.economy.coins + amount }).await?;
        ctx.reply_styled(&format!("ꕣ Se dieron *¥{}* a @{}.", format_number(amount), target.split('@').next().unwrap_or("?"))).await
    }
}
