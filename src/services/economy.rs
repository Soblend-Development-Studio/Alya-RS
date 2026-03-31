use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub item_type: String,
    pub emoji: String,
}

pub struct ShopService {
    items: Arc<RwLock<Vec<ShopItem>>>,
    stock: Arc<RwLock<Vec<ShopItem>>>,
}

impl ShopService {
    pub fn new() -> Self {
        let service = Self {
            items: Arc::new(RwLock::new(Self::default_items())),
            stock: Arc::new(RwLock::new(vec![])),
        };
        service
    }

    fn default_items() -> Vec<ShopItem> {
        vec![
            ShopItem { id: "potion_small".into(), name: "Poción Pequeña".into(), description: "Restaura 100 monedas".into(), price: 500, item_type: "potion".into(), emoji: "🧪".into() },
            ShopItem { id: "potion_medium".into(), name: "Poción Mediana".into(), description: "Restaura 500 monedas".into(), price: 2000, item_type: "potion".into(), emoji: "⚗️".into() },
            ShopItem { id: "potion_large".into(), name: "Poción Grande".into(), description: "Restaura 2000 monedas".into(), price: 7500, item_type: "potion".into(), emoji: "🔮".into() },
            ShopItem { id: "shield".into(), name: "Escudo Anti-Robo".into(), description: "Protege tus monedas de robos por 24h".into(), price: 5000, item_type: "protection".into(), emoji: "🛡️".into() },
            ShopItem { id: "luck_charm".into(), name: "Amuleto de Suerte".into(), description: "Aumenta las ganancias 2x por 1h".into(), price: 15000, item_type: "booster".into(), emoji: "🍀".into() },
            ShopItem { id: "fishing_rod".into(), name: "Caña de Pescar".into(), description: "Necesaria para usar el comando pesca".into(), price: 3000, item_type: "tool".into(), emoji: "🎣".into() },
            ShopItem { id: "gacha_token".into(), name: "Token Gacha".into(), description: "Permite un roll extra inmediato".into(), price: 25000, item_type: "special".into(), emoji: "🎫".into() },
            ShopItem { id: "exp_boost".into(), name: "Boost de XP".into(), description: "Duplica la XP ganada por 2h".into(), price: 10000, item_type: "booster".into(), emoji: "⚡".into() },
            ShopItem { id: "mystery_box".into(), name: "Caja Misteriosa".into(), description: "Contiene un regalo aleatorio".into(), price: 8000, item_type: "special".into(), emoji: "🎁".into() },
            ShopItem { id: "premium_pass".into(), name: "Pase Premium".into(), description: "Acceso a contenido exclusivo por 7 días".into(), price: 50000, item_type: "premium".into(), emoji: "💎".into() },
        ]
    }

    pub async fn get_stock(&self) -> Vec<ShopItem> {
        self.items.read().await.clone()
    }

    pub async fn get_item(&self, item_id: &str) -> Option<ShopItem> {
        self.items.read().await.iter().find(|i| i.id == item_id).cloned()
    }

    pub async fn rotate_stock(&self) {
        let all = self.items.read().await.clone();
        let mut rng = rand::thread_rng();
        let count = 5.min(all.len());
        let mut indices: Vec<usize> = (0..all.len()).collect();
        // Simple shuffle
        for i in (1..indices.len()).rev() {
            let j = rng.gen_range(0..=i);
            indices.swap(i, j);
        }
        let stock: Vec<ShopItem> = indices[..count].iter().map(|&i| all[i].clone()).collect();
        *self.stock.write().await = stock;
    }
}

impl Default for ShopService {
    fn default() -> Self {
        Self::new()
    }
}
