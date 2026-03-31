use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::contains_whatsapp_link;

pub struct AntilinkCommand;

#[async_trait]
impl Command for AntilinkCommand {
    fn triggers(&self) -> &[&str] { &["antilink"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Activa/desactiva el filtro antilink del grupo" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }
        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if !is_admin && !ctx.is_owner { return ctx.reply_styled(crate::config::errors::ADMIN_ONLY).await; }

        let group = ctx.db.get_group(&ctx.chat_id).await?;
        let new_state = !group.settings.antilink;
        ctx.db.update_group(&ctx.chat_id, mongodb::bson::doc! { "settings.antilink": new_state }).await?;
        ctx.reply_styled(&format!("ꕣ Anti-link *{}*.",
            if new_state { "ACTIVADO ✅" } else { "DESACTIVADO ❌" })).await
    }

    async fn before(&self, ctx: &CommandContext) -> Result<bool> {
        if !ctx.is_group { return Ok(true); }
        let group = match ctx.db.get_group(&ctx.chat_id).await {
            Ok(g) => g,
            Err(_) => return Ok(true),
        };
        if !group.settings.antilink { return Ok(true); }

        let is_admin = ctx.client.is_admin(&ctx.chat_id, &ctx.sender).await.unwrap_or(false);
        if is_admin || ctx.is_owner { return Ok(true); }

        if contains_whatsapp_link(&ctx.body) {
            let _ = ctx.reply_styled("ꕢ Los links de WhatsApp no están permitidos en este grupo.").await;
            return Ok(false);
        }

        Ok(true)
    }
}
