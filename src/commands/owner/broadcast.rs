use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct BroadcastCommand;
#[async_trait]
impl Command for BroadcastCommand {
    fn triggers(&self) -> &[&str] { &["broadcast", "bc"] }
    fn category(&self) -> &str { "owner" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled(crate::config::errors::PERMISSION_DENIED).await; }
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#bc <mensaje>`").await; }
        let msg = ctx.args.join(" ");
        let groups = ctx.db.get_all_groups().await?;
        let count = groups.len();
        for group in groups {
            let _ = ctx.client.send_text(&group.id, &msg).await;
        }
        ctx.reply_styled(&format!("ꕣ Broadcast enviado a *{}* grupos.", count)).await
    }
}
