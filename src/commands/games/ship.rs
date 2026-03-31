use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::get_random_int;
use crate::utils::helpers::extract_mentions;

pub struct ShipCommand;

#[async_trait]
impl Command for ShipCommand {
    fn triggers(&self) -> &[&str] { &["ship", "compatibilidad"] }
    fn category(&self) -> &str { "games" }
    fn help(&self) -> &str { "Calcula la compatibilidad amorosa entre dos usuarios" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        let percent = get_random_int(0, 100);
        let bar_len = (percent / 10) as usize;
        let bar = "💗".repeat(bar_len) + &"🖤".repeat(10 - bar_len);

        let label = if percent >= 80 { "¡Pareja perfecta! 💑" }
                    else if percent >= 60 { "¡Hay potencial! 💕" }
                    else if percent >= 40 { "Puede que funcione... 🤔" }
                    else { "Son incompatibles... 💔" };

        let (user1, user2) = if mentions.len() >= 2 {
            (mentions[0].split('@').next().unwrap_or("?"), mentions[1].split('@').next().unwrap_or("?"))
        } else if mentions.len() == 1 {
            (ctx.sender.split('@').next().unwrap_or("?"), mentions[0].split('@').next().unwrap_or("?"))
        } else {
            return ctx.reply_styled("ꕢ Menciona a alguien para calcular compatibilidad.").await;
        };

        ctx.reply_styled(&format!(
            "💕 *SHIP*\n\n\
             @{} ❤️ @{}\n\n\
             {}\n\
             Compatibilidad: *{}%*\n\n\
             {}",
            user1, user2, bar, percent, label
        )).await
    }
}
