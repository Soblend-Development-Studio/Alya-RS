use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct ObtenerinfoCommand;
#[async_trait]
impl Command for ObtenerinfoCommand {
    fn triggers(&self) -> &[&str] { &["obtenerinfo", "getinfo"] }
    fn category(&self) -> &str { "tools" }
    fn help(&self) -> &str { "Obtiene información de un número de WhatsApp" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let target = ctx.args.first().cloned().unwrap_or_else(|| ctx.sender.clone());
        let phone = crate::utils::helpers::extract_phone(&target);
        ctx.reply_styled(&format!(
            "ꕣ *INFO DEL NÚMERO*\n\n📱 Número: *{}*\n📡 JID: `{}@s.whatsapp.net`",
            phone, phone
        )).await
    }
}
