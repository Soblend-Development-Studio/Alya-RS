use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserEconomy {
    #[serde(default)]
    pub coins: i64,
    #[serde(default)]
    pub bank: i64,
    #[serde(default)]
    pub last_daily: i64,
    #[serde(default)]
    pub last_work: i64,
    #[serde(default)]
    pub last_crime: i64,
    #[serde(default)]
    pub last_slut: i64,
    #[serde(default)]
    pub daily_streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GachaCharacter {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub value: i64,
    #[serde(default)]
    pub img: Vec<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserGacha {
    #[serde(default)]
    pub characters: Vec<GachaCharacter>,
    #[serde(default)]
    pub last_claim: i64,
    #[serde(default)]
    pub last_roll: i64,
    #[serde(default)]
    pub rolled: Option<String>,
    #[serde(default)]
    pub last_vote: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStats {
    #[serde(default)]
    pub messages: i64,
    #[serde(default)]
    pub commands: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserLevel {
    #[serde(default)]
    pub xp: i64,
    #[serde(default)]
    pub lvl: i32,
    #[serde(default)]
    pub last_xp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub birthday: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryItem {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub quantity: i32,
    #[serde(default)]
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: String,
    #[serde(default)]
    pub economy: UserEconomy,
    #[serde(default)]
    pub gacha: UserGacha,
    #[serde(default)]
    pub stats: UserStats,
    #[serde(default)]
    pub level: UserLevel,
    #[serde(default)]
    pub inventory: Vec<InventoryItem>,
    #[serde(default = "default_created_at")]
    pub created_at: i64,
    #[serde(default)]
    pub monedas: i64,
    #[serde(default)]
    pub antirobo: i32,
    #[serde(default)]
    pub desbloqueo: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub profile: Option<UserProfile>,
}

fn default_created_at() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

impl User {
    pub fn new(id: String) -> Self {
        Self {
            _id: None,
            id,
            economy: UserEconomy::default(),
            gacha: UserGacha::default(),
            stats: UserStats::default(),
            level: UserLevel { xp: 0, lvl: 1, last_xp: 0 },
            inventory: vec![],
            created_at: default_created_at(),
            monedas: 0,
            antirobo: 0,
            desbloqueo: 0,
            name: None,
            profile: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupSettings {
    #[serde(default)]
    pub welcome: bool,
    #[serde(default)]
    pub goodbye: bool,
    #[serde(default)]
    pub antilink: bool,
    #[serde(default = "default_economy")]
    pub economy: bool,
    #[serde(default)]
    pub nsfw: bool,
    #[serde(default)]
    pub alerts: bool,
    #[serde(default = "default_currency")]
    pub currency_name: String,
}

fn default_economy() -> bool { true }
fn default_currency() -> String { "coins".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: String,
    #[serde(default)]
    pub settings: GroupSettings,
    #[serde(default)]
    pub alerts: Vec<serde_json::Value>,
    #[serde(default)]
    pub stats: GroupStats,
    #[serde(default)]
    pub primary_bot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupStats {
    #[serde(default)]
    pub messages: i64,
}

impl Group {
    pub fn new(id: String) -> Self {
        Self {
            _id: None,
            id,
            settings: GroupSettings {
                welcome: false,
                goodbye: false,
                antilink: false,
                economy: true,
                nsfw: false,
                alerts: false,
                currency_name: "coins".to_string(),
            },
            alerts: vec![],
            stats: GroupStats::default(),
            primary_bot: None,
        }
    }
}
