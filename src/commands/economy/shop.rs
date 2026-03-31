use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text};

pub struct ShopCommand;

#[async_trait]
impl Command for ShopCommand {
    fn triggers(&self) -> &[&str] { &["shop", "tienda"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Muestra la tienda y permite comprar ítems" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let items = ctx.shop.get_stock().await;

        if ctx.args.is_empty() {
            // Show shop
            let mut msg = "ꕣ *TIENDA*\n\n".to_string();
            for item in &items {
                msg.push_str(&format!(
                    "{} *{}* — ¥{}\n  _{}_\n  ID: `{}`\n\n",
                    item.emoji, item.name,
                    format_number(item.price),
                    item.description,
                    item.id
                ));
            }
            msg.push_str("> Usa `#shop <id>` para comprar un ítem");
            return ctx.reply_styled(&msg).await;
        }

        let item_id = &ctx.args[0];
        let item = match ctx.shop.get_item(item_id).await {
            Some(i) => i,
            None => return ctx.reply_styled(&format!("ꕢ No existe el ítem `{}`.", item_id)).await,
        };

        let user = ctx.db.get_user(&ctx.sender).await?;
        if user.economy.coins < item.price {
            return ctx.reply_styled(&format!(
                "ꕢ No tienes suficientes monedas.\nNecesitas *¥{}* y tienes *¥{}*",
                format_number(item.price),
                format_number(user.economy.coins)
            )).await;
        }

        // Deduct coins and add item to inventory
        let new_coins = user.economy.coins - item.price;
        ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
            "economy.coins": new_coins,
            "$push": { "inventory": { "id": &item.id, "name": &item.name, "quantity": 1, "type": &item.item_type } }
        }).await?;

        ctx.reply_styled(&format!(
            "ꕣ *COMPRA EXITOSA*\n\n\
             {} *{}*\n\
             ⟡ Precio: *¥{}*\n\
             ⟡ Saldo restante: *¥{}*",
            item.emoji, item.name,
            format_number(item.price),
            format_number(new_coins)
        )).await
    }
}
