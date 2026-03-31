use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::helpers::extract_mentions;
use crate::utils::format::get_random;

const KILL_MSGS: &[&str] = &["le lanzó un rayo", "lo eliminó del grupo", "lo hizo desaparecer mágicamente", "activó el modo ultra instinto y lo vaporizó"];

pub struct KillCommand;
#[async_trait]
impl Command for KillCommand {
    fn triggers(&self) -> &[&str] { &["kill", "matar"] }
    fn category(&self) -> &str { "fun" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let sender = ctx.sender.split('@').next().unwrap_or("?");
        let target = mentions.first().map(|t| format!("@{}", t.split('@').next().unwrap_or("?"))).unwrap_or_else(|| "al vacío".to_string());
        let msg = get_random(KILL_MSGS).unwrap_or(&"atacó");
        ctx.reply_styled(&format!("💀 *@{}* {} a {} 💀", sender, msg, target)).await
    }
}
