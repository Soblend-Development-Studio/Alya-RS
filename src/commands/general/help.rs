use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    fn triggers(&self) -> &[&str] { &["help", "ayuda", "menu", "start", "h"] }
    fn category(&self) -> &str { "general" }
    fn help(&self) -> &str { "Muestra el menú de ayuda" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let arg = ctx.args.first().cloned().unwrap_or_default();

        let msg = if arg.is_empty() {
            format!(
                "ꕣ *ALYA KUJOU BOT* ✿\n\n\
                 *「 Menú Principal 」*\n\n\
                 ➭ `{p}help admin` — Administración\n\
                 ➭ `{p}help economy` — Economía 💰\n\
                 ➭ `{p}help gacha` — Gacha/Waifus 🎌\n\
                 ➭ `{p}help games` — Juegos 🎮\n\
                 ➭ `{p}help ai` — Inteligencia Artificial 🤖\n\
                 ➭ `{p}help tools` — Herramientas 🛠️\n\
                 ➭ `{p}help search` — Búsqueda 🔍\n\
                 ➭ `{p}help downloads` — Descargas 📥\n\
                 ➭ `{p}help fun` — Diversión 🎉\n\
                 ➭ `{p}help nsfw` — NSFW 🔞\n\n\
                 ⟡ Prefijos: `#` `!` `/` `.` `:`\n\
                 ⟡ Total comandos: *159+*\n\
                 ⟡ Personajes gacha: *{}*\n\n\
                 _✿ Alya Kujou Bot - Rust Edition_",
                p = ctx.prefix,
                ctx.gacha.get_total().await
            )
        } else {
            format!("ꕢ Usa `{p}help` para ver todas las categorías.", p = ctx.prefix)
        };

        ctx.reply_styled(&msg).await
    }
}
