use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct SettingsCommand;

#[async_trait]
impl Command for SettingsCommand {
    fn triggers(&self) -> &[&str] { &["settings", "config", "configuracion"] }
    fn category(&self) -> &str { "admin" }
    fn help(&self) -> &str { "Muestra la configuración del grupo" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_group { return ctx.reply_styled(crate::config::errors::GROUPS_ONLY).await; }

        let group = ctx.db.get_group(&ctx.chat_id).await?;
        let s = &group.settings;

        ctx.reply_styled(&format!(
            "⚙️ *CONFIGURACIÓN DEL GRUPO*\n\n\
             🌸 Bienvenida: {}\n\
             👋 Despedida: {}\n\
             🔗 Anti-link: {}\n\
             💰 Economía: {}\n\
             🔞 NSFW: {}\n\
             🔔 Alertas: {}\n\
             💎 Moneda: *{}*",
            toggle(s.welcome), toggle(s.goodbye), toggle(s.antilink),
            toggle(s.economy), toggle(s.nsfw), toggle(s.alerts),
            s.currency_name
        )).await
    }
}

fn toggle(v: bool) -> &'static str {
    if v { "✅" } else { "❌" }
}
