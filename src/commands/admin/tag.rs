use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct TagCommand;

#[async_trait]
impl Command for TagCommand {
    fn triggers(&self) -> &[&str] { &["tag", "all", "everyone"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Menciona a todos los miembros del grupo" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }

        let participants = ctx.client.get_group_participants(&ctx.chat_id).await?;
        let msg = ctx.args.join(" ");
        let mut text = if msg.is_empty() {
            "📢 *Atención a todos:*\n\n".to_string()
        } else {
            format!("📢 *{}*\n\n", msg)
        };

        for p in &participants {
            text.push_str(&format!("@{} ", p.id.split('@').next().unwrap_or("")));
        }

        ctx.reply_styled(&text).await
    }
}
