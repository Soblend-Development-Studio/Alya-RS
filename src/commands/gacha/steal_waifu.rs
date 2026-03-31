use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_random_int};
use crate::utils::helpers::extract_mentions;

pub struct StealWaifuCommand;

#[async_trait]
impl Command for StealWaifuCommand {
    fn triggers(&self) -> &[&str] { &["robar", "robarwaifu"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Intenta robar un personaje de otro usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() {
            return ctx.reply_styled("ꕢ Uso: `#robar @usuario <nombre o ID del personaje>`").await;
        }

        let target = &mentions[0];
        if target == &ctx.sender {
            return ctx.reply_styled("ꕢ No puedes robarte a ti mismo.").await;
        }

        let target_user = ctx.db.get_user(target).await?;

        if target_user.antirobo > 0 {
            ctx.db.update_user(target, mongodb::bson::doc! {
                "antirobo": target_user.antirobo - 1
            }).await?;
            return ctx.reply_styled(&format!(
                "🛡️ @{} tiene escudo anti-robo! Tu intento fue bloqueado.",
                target.split('@').next().unwrap_or("")
            )).await;
        }

        if target_user.gacha.characters.is_empty() {
            return ctx.reply_styled("ꕢ Esa persona no tiene personajes para robar.").await;
        }

        let success = get_random_int(1, 100) <= 30;

        if success {
            let char_idx = get_random_int(0, target_user.gacha.characters.len() as i64 - 1) as usize;
            let character = target_user.gacha.characters[char_idx].clone();
            let mut target_chars = target_user.gacha.characters.clone();
            target_chars.remove(char_idx);

            let chars_bson: Vec<bson::Document> = target_chars.iter().map(|c| bson::doc! {
                "id": &c.id, "name": &c.name, "source": c.source.as_deref().unwrap_or(""), "value": c.value
            }).collect();

            ctx.db.update_user(target, mongodb::bson::doc! {
                "gacha.characters": chars_bson
            }).await?;

            let char_doc = bson::doc! {
                "id": &character.id, "name": &character.name,
                "source": character.source.as_deref().unwrap_or(""), "value": character.value
            };
            let user = ctx.db.get_user(&ctx.sender).await?;
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "$push": { "gacha.characters": char_doc }
            }).await?;

            ctx.reply_styled(&format!(
                "ꕣ *¡ROBO EXITOSO!*\n\n\
                 ♛ Robaste *{}* de @{}!",
                character.name,
                target.split('@').next().unwrap_or("")
            )).await
        } else {
            ctx.reply_styled(&format!(
                "ꕢ *ROBO FALLIDO*\n\n\
                 @{} se dio cuenta y escapaste con las manos vacías.",
                target.split('@').next().unwrap_or("")
            )).await
        }
    }
}
