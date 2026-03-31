use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;
use crate::utils::helpers::extract_mentions;

pub struct DarCommand;

#[async_trait]
impl Command for DarCommand {
    fn triggers(&self) -> &[&str] { &["dar", "regalar"] }
    fn category(&self) -> &str { "gacha" }
    fn help(&self) -> &str { "Regala un personaje a otro usuario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        // Same as give command, re-uses logic
        let mentions = extract_mentions(&ctx.body);
        if mentions.is_empty() || ctx.args.len() < 2 {
            return ctx.reply_styled("ꕢ Uso: `#dar @usuario <nombre o ID>`").await;
        }

        let target = &mentions[0];
        if target == &ctx.sender {
            return ctx.reply_styled("ꕢ No puedes regalarte un personaje a ti mismo.").await;
        }

        let char_query = ctx.args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
        let user = ctx.db.get_user(&ctx.sender).await?;

        let char_idx = user.gacha.characters.iter().position(|c| {
            c.id == char_query || c.name.to_lowercase() == char_query.to_lowercase()
        });

        let char_idx = match char_idx {
            Some(i) => i,
            None => return ctx.reply_styled(&format!("ꕢ No tienes el personaje *{}*.", char_query)).await,
        };

        let character = user.gacha.characters[char_idx].clone();
        let mut chars = user.gacha.characters.clone();
        chars.remove(char_idx);

        let chars_bson: Vec<bson::Document> = chars.iter().map(|c| bson::doc! {
            "id": &c.id, "name": &c.name, "source": c.source.as_deref().unwrap_or(""), "value": c.value
        }).collect();

        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "gacha.characters": chars_bson
        }).await?;

        let char_doc = bson::doc! {
            "id": &character.id, "name": &character.name,
            "source": character.source.as_deref().unwrap_or(""), "value": character.value
        };
        ctx.db.update_user(target, mongodb::bson::doc! {
            "$push": { "gacha.characters": char_doc }
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *REGALO ENVIADO*\n\n\
             🎁 Le regalaste *{}* a @{}",
            character.name,
            target.split('@').next().unwrap_or("")
        )).await
    }
}
