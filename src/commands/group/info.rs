use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct GroupInfoCommand;

#[async_trait]
impl Command for GroupInfoCommand {
    fn triggers(&self) -> &[&str] { &["groupinfo", "ginfo", "infogrupo"] }
    fn category(&self) -> &str { "group" }
    fn help(&self) -> &str { "Muestra información del grupo" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }

        let group = ctx.db.get_group(&ctx.chat_id).await?;
        let participants = ctx.client.get_group_participants(&ctx.chat_id).await.unwrap_or_default();
        let admins = ctx.client.get_group_admins(&ctx.chat_id).await.unwrap_or_default();

        ctx.reply_styled(&format!(
            "ꕣ *INFO DEL GRUPO*\n\n\
             📌 ID: `{}`\n\
             👥 Miembros: *{}*\n\
             👑 Admins: *{}*\n\
             💰 Economía: *{}*\n\
             🌸 Bienvenida: *{}*\n\
             🔗 Anti-link: *{}*\n\
             🔞 NSFW: *{}*",
            ctx.chat_id,
            participants.len(),
            admins.len(),
            if group.settings.economy { "✅" } else { "❌" },
            if group.settings.welcome { "✅" } else { "❌" },
            if group.settings.antilink { "✅" } else { "❌" },
            if group.settings.nsfw { "✅" } else { "❌" }
        )).await
    }
}
