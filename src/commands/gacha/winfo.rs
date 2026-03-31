use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};
use crate::utils::helpers::{get_stars, get_rarity_text};

pub struct WinfoCommand;

#[async_trait]
impl Command for WinfoCommand {
    fn triggers(&self) -> &[&str] { &["winfo", "charinfo", "personaje"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Muestra info de un personaje por nombre o ID" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.args.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#winfo <nombre o ID>`").await;
        }

        let query = ctx.args.join(" ");

        let character = if let Ok(id) = query.parse::<u64>() {
            ctx.gacha.get_by_id(&id.to_string()).await
        } else {
            ctx.gacha.get_by_name(&query).await
        };

        let character = match character {
            Some(c) => c,
            None => {
                let results = ctx.gacha.search(&query).await;
                if results.is_empty() {
                    return ctx.reply_styled(&format!("ꕢ No se encontró el personaje *{}*.", query)).await;
                }
                results.into_iter().next().unwrap()
            }
        };

        let stars = get_stars(character.value);
        let rarity = get_rarity_text(character.value);
        let owner = character.user.as_deref()
            .map(|u| format!("@{}", u.split('@').next().unwrap_or("")))
            .unwrap_or_else(|| "Nadie".to_string());

        let msg = format!(
            "ꕣ *INFO DE PERSONAJE*\n\n\
             ➭ ID: `{}`\n\
             ♛ Nombre: *{}*\n\
             🎌 Fuente: *{}*\n\
             ⚥ Género: *{}*\n\
             𖧧 Rareza: *{} {}*\n\
             💰 Valor: *¥{}*\n\
             👤 Dueño: *{}*",
            character.id,
            character.name,
            character.source.as_deref().unwrap_or("Desconocido"),
            character.gender.as_deref().unwrap_or("Desconocido"),
            stars, rarity,
            format_number(character.value),
            owner
        );

        if let Some(img_url) = character.img.first() {
            ctx.send_image(img_url, &style_text(&msg)).await?;
        } else {
            ctx.reply_styled(&msg).await?;
        }

        Ok(())
    }
}
