use std::sync::Arc;
use anyhow::Result;
use tracing::error;

use crate::db::DatabaseService;

pub struct LevelResult {
    pub leveled_up: bool,
    pub current_level: i32,
    pub current_xp: i64,
    pub xp_needed: i64,
}

pub struct LevelService {
    db: Arc<DatabaseService>,
}

impl LevelService {
    pub fn new(db: Arc<DatabaseService>) -> Self {
        Self { db }
    }

    pub fn xp_for_level(level: i32) -> i64 {
        let base_xp = 100_i64;
        (base_xp as f64 * (level as f64).powf(1.5)) as i64
    }

    pub async fn add_xp(&self, user_id: &str, amount: i64) -> Result<LevelResult> {
        let user = self.db.get_user(user_id).await?;
        let mut xp = user.level.xp + amount;
        let mut lvl = user.level.lvl;
        let mut leveled_up = false;

        let xp_needed = Self::xp_for_level(lvl);
        if xp >= xp_needed {
            xp -= xp_needed;
            lvl += 1;
            leveled_up = true;
        }

        let now = chrono::Utc::now().timestamp_millis();
        self.db.update_user(user_id, mongodb::bson::doc! {
            "level.xp": xp,
            "level.lvl": lvl,
            "level.lastXp": now
        }).await?;

        Ok(LevelResult {
            leveled_up,
            current_level: lvl,
            current_xp: xp,
            xp_needed: Self::xp_for_level(lvl),
        })
    }
}
