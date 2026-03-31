use async_trait::async_trait;
use anyhow::Result;
use crate::commands::{Command, CommandContext};
use crate::utils::format::{format_number, style_text, get_cooldown, format_time, get_random_int, get_random};

const COOLDOWN_MS: i64 = 30 * 60 * 1000;

const FISH_TYPES: &[(&str, i64)] = &[
    ("🐟 Pez Pequeño", 500),
    ("🐠 Pez Tropical", 1500),
    ("🐡 Pez Globo", 2000),
    ("🦈 Tiburón Pequeño", 5000),
    ("🐙 Pulpo", 4000),
    ("🦑 Calamar", 3500),
    ("🦐 Camarones", 1000),
    ("🦞 Langosta", 8000),
    ("🦀 Cangrejo", 3000),
    ("🐬 Delfín (¡lo dejaste ir!)", 0),
    ("👟 Zapato Viejo", 50),
    ("💎 Gema Submarina", 25000),
    ("⚓ Ancla Oxidada", 100),
];

pub struct FishCommand;

#[async_trait]
impl Command for FishCommand {
    fn triggers(&self) -> &[&str] { &["fish", "pescar", "pesca"] }
    fn category(&self) -> &str { "economy" }
    fn help(&self) -> &str { "Ve a pescar para ganar monedas (cooldown: 30min)" }

    async fn execute(&self, ctx: &CommandContext) -> Result<()> {
        let currency = ctx.get_currency_name().await;
        let cache_key = format!("fish_{}", ctx.sender);

        if ctx.cache.has(&cache_key) {
            return ctx.reply_styled("ꕢ Ya pescaste recientemente. Espera 30 minutos.").await;
        }

        // Check fishing rod in inventory
        let user = ctx.db.get_user(&ctx.sender).await?;
        let has_rod = user.inventory.iter().any(|i| i.id == "fishing_rod");
        if !has_rod {
            return ctx.reply_styled(
                "ꕢ Necesitas una *Caña de Pescar* para pescar.\nCómprala en `#shop`."
            ).await;
        }

        ctx.cache.set(&cache_key, "1", 1800);

        let idx = get_random_int(0, FISH_TYPES.len() as i64 - 1) as usize;
        let (fish_name, value) = FISH_TYPES[idx];

        let new_coins = user.economy.coins + value;
        if value > 0 {
            ctx.db.update_user(&ctx.sender, mongodb::bson::doc! {
                "economy.coins": new_coins
            }).await?;
        }

        let result = if value > 0 {
            format!("¡Atrapaste {}!\n⟡ Ganaste: *¥{}* {}", fish_name, format_number(value), currency)
        } else if fish_name.contains("Delfín") {
            format!("Atrapaste {}\n_Fue tan bonito que lo dejaste ir_ 🥺", fish_name)
        } else {
            format!("Atrapaste {}\n⟡ Ganaste: *¥{}* {}", fish_name, format_number(value), currency)
        };

        ctx.reply_styled(&format!("🎣 *PESCA*\n\n{}", result)).await
    }
}
