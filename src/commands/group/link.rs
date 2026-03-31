use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GroupLinkCommand;
#[async_trait]
impl Command for GroupLinkCommand {
    fn triggers(&self) -> &[&str] { &["link", "grouplink"] }
    fn category(&self) -> &str { "group" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }
        ctx.reply_styled("ꕣ Función de obtener link del grupo disponible próximamente.").await
    }
}
