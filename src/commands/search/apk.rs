use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct ApkCommand;
#[async_trait]
impl Command for ApkCommand {
    fn triggers(&self) -> &[&str] { &["apk"] }
    fn category(&self) -> &str { "search" }
    fn help(&self) -> &str { "Busca y descarga APKs" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() { return ctx.reply_styled("ꕢ Uso: `#apk <nombre de la app>`").await; }
        let q = ctx.args.join(" ");
        ctx.reply_styled(&format!("📱 Buscando APK: *{}*...\n\n_Función en desarrollo_", q)).await
    }
}
