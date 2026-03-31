use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, get_random_int, get_random};
use crate::utils::helpers::extract_mentions;

const FIGHT_MSGS: &[&str] = &[
    "le lanzó un mueble encima",
    "le hizo un judo throw",
    "lo golpeó con un nunchaku de plástico",
    "lo sorprendió con un ataque especial",
    "lo venció en un duelo de miradas intimidantes",
];

pub struct FightCommand;

#[async_trait]
impl Command for FightCommand {
    fn triggers(&self) -> &[&str] { &["fight", "pelear", "atacar"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Reta a alguien a pelear" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() {
            return ctx.reply_styled("ꕢ Menciona a alguien para pelear.").await;
        }

        let target = &mentions[0];
        let msg = get_random(FIGHT_MSGS).unwrap_or(&"ganó la pelea");

        ctx.reply_styled(&format!(
            "⚔️ *PELEA*\n\n\
             @{} {} a @{}!",
            ctx.sender.split('@').next().unwrap_or("?"),
            msg,
            target.split('@').next().unwrap_or("?")
        )).await
    }
}
