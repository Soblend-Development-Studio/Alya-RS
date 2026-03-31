use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct DemoteCommand;

#[async_trait]
impl Command for DemoteCommand {
    fn triggers(&self) -> &[&str] { &["demote", "desadmin"] }
    fn category(&self) -> &str { "admin" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() { return ctx.reply_styled("ꕢ Menciona al administrador a remover.").await; }
        ctx.client.demote_participant(&ctx.chat_id, &mentions[0]).await?;
        ctx.reply_styled(&format!("ꕣ @{} ya no es administrador.", mentions[0].split('@').next().unwrap_or("?"))).await
    }
}
