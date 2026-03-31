use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;

pub struct ResetwaifusCommand;
#[async_trait]
impl Command for ResetwaifusCommand {
    fn triggers(&self) -> &[&str] { &["resetwaifus"] }
    fn category(&self) -> &str { "gacha" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if !ctx.is_owner { return ctx.reply_styled("ꕢ Solo el dueño puede usar este comando.").await; }
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender).clone();
        ctx.db.update_user(&target, mongodb::bson::doc! { "gacha.characters": [] }).await?;
        ctx.reply_styled(&format!("ꕣ Waifus de @{} reseteadas.", target.split('@').next().unwrap_or("?"))).await
    }
}
