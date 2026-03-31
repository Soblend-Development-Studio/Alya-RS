use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::style_text;

pub struct InventoryCommand;

#[async_trait]
impl Command for InventoryCommand {
    fn triggers(&self) -> &[&str] { &["inventory", "inventario", "inv"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Muestra tu inventario" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let user = ctx.db.get_user(&ctx.sender).await?;

        if user.inventory.is_empty() {
            return ctx.reply_styled("ꕢ Tu inventario está vacío. Usa `#shop` para comprar ítems.").await;
        }

        let mut msg = "ꕣ *TU INVENTARIO*\n\n".to_string();
        for item in &user.inventory {
            msg.push_str(&format!("⟡ **{}** x{}\n", item.name, item.quantity));
        }

        ctx.reply_styled(&msg).await
    }
}
