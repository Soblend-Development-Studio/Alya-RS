use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct VcharsCommand;
#[async_trait]
impl Command for VcharsCommand {
    fn triggers(&self) -> &[&str] { &["vchars", "verchars"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Ve los personajes que posee un usuario" }
    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        use crate::utils::helpers::extract_mentions;
        let mentions = extract_mentions(&ctx.body);
        let target = mentions.first().unwrap_or(&ctx.sender).clone();
        let user = ctx.db.get_user(&target).await?;
        if user.gacha.characters.is_empty() {
            return ctx.reply_styled("ꕢ Este usuario no tiene personajes.").await;
        }
        let mut msg = format!("♛ *Personajes de @{}*\n\n", target.split('@').next().unwrap_or("?"));
        for (i, c) in user.gacha.characters.iter().take(20).enumerate() {
            msg.push_str(&format!("{}. *{}* _({})\n", i + 1, c.name, c.source.as_deref().unwrap_or("?")));
        }
        ctx.reply_styled(&msg).await
    }
}
