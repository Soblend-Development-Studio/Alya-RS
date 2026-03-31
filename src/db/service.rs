use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::bson::{doc, Document};
use anyhow::{Result, Context};
use tracing::{info, warn, error};

use super::models::{User, Group};

const USER_CACHE_TTL: Duration = Duration::from_secs(300);
const GROUP_CACHE_TTL: Duration = Duration::from_secs(300);

struct CacheEntry<T> {
    value: T,
    inserted_at: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration) -> Self {
        Self { value, inserted_at: Instant::now(), ttl }
    }

    fn is_expired(&self) -> bool {
        self.inserted_at.elapsed() > self.ttl
    }
}

pub struct DatabaseService {
    users: Collection<User>,
    groups: Collection<Group>,
    user_cache: Arc<DashMap<String, CacheEntry<User>>>,
    group_cache: Arc<DashMap<String, CacheEntry<Group>>>,
}

impl DatabaseService {
    pub async fn new(mongodb_uri: &str) -> Result<Self> {
        info!("𖤐 Conectando a MongoDB...");

        let mut opts = ClientOptions::parse(mongodb_uri)
            .await
            .context("Error parsing MongoDB URI")?;

        opts.max_pool_size = Some(10);
        opts.min_pool_size = Some(2);
        opts.server_selection_timeout = Some(Duration::from_secs(5));
        opts.socket_timeout = Some(Duration::from_secs(45));

        let client = Client::with_options(opts)?;
        let db = client.database("alya_bot");

        let users = db.collection::<User>("users");
        let groups = db.collection::<Group>("groups");

        // Create indexes
        let _ = users.create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "id": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await;

        let _ = groups.create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "id": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await;

        info!("𖤐 Conectado a MongoDB exitosamente");

        Ok(Self {
            users,
            groups,
            user_cache: Arc::new(DashMap::new()),
            group_cache: Arc::new(DashMap::new()),
        })
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        // Check cache first
        if let Some(entry) = self.user_cache.get(user_id) {
            if !entry.is_expired() {
                return Ok(entry.value.clone());
            }
        }

        let user = match self.users.find_one(doc! { "id": user_id }, None).await? {
            Some(u) => u,
            None => {
                let new_user = User::new(user_id.to_string());
                self.users.insert_one(&new_user, None).await?;
                new_user
            }
        };

        self.user_cache.insert(
            user_id.to_string(),
            CacheEntry::new(user.clone(), USER_CACHE_TTL),
        );

        Ok(user)
    }

    pub async fn update_user(&self, user_id: &str, updates: Document) -> Result<()> {
        self.user_cache.remove(user_id);

        self.users
            .update_one(
                doc! { "id": user_id },
                doc! { "$set": updates },
                mongodb::options::UpdateOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(())
    }

    pub async fn get_group(&self, group_id: &str) -> Result<Group> {
        // Check cache
        if let Some(entry) = self.group_cache.get(group_id) {
            if !entry.is_expired() {
                return Ok(entry.value.clone());
            }
        }

        let group = match self.groups.find_one(doc! { "id": group_id }, None).await? {
            Some(g) => g,
            None => {
                let new_group = Group::new(group_id.to_string());
                self.groups.insert_one(&new_group, None).await?;
                new_group
            }
        };

        self.group_cache.insert(
            group_id.to_string(),
            CacheEntry::new(group.clone(), GROUP_CACHE_TTL),
        );

        Ok(group)
    }

    pub async fn update_group(&self, group_id: &str, updates: Document) -> Result<()> {
        self.group_cache.remove(group_id);

        self.groups
            .update_one(
                doc! { "id": group_id },
                doc! { "$set": updates },
                mongodb::options::UpdateOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(())
    }

    pub async fn get_leaderboard(&self, limit: i64) -> Result<Vec<User>> {
        use futures::StreamExt;
        let pipeline = vec![
            doc! { "$addFields": { "totalCoins": { "$add": ["$economy.coins", "$economy.bank"] } } },
            doc! { "$sort": { "totalCoins": -1 } },
            doc! { "$limit": limit },
        ];
        let mut cursor = self.users.aggregate(pipeline, None).await?;
        let mut results = vec![];
        while let Some(doc) = cursor.next().await {
            let doc = doc?;
            if let Ok(user) = bson::from_document::<User>(doc) {
                results.push(user);
            }
        }
        Ok(results)
    }

    pub async fn get_user_rank(&self, user_id: &str) -> Result<i64> {
        let user = self.get_user(user_id).await?;
        let total = user.economy.coins + user.economy.bank;
        let count = self.users.count_documents(
            doc! {
                "$expr": {
                    "$gt": [
                        { "$add": ["$economy.coins", "$economy.bank"] },
                        total
                    ]
                }
            },
            None,
        ).await?;
        Ok(count as i64 + 1)
    }

    pub async fn get_user_count(&self) -> Result<u64> {
        Ok(self.users.count_documents(None, None).await?)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.user_cache.remove(user_id);
        self.users.delete_one(doc! { "id": user_id }, None).await?;
        Ok(())
    }

    pub async fn delete_group(&self, group_id: &str) -> Result<()> {
        self.group_cache.remove(group_id);
        self.groups.delete_one(doc! { "id": group_id }, None).await?;
        Ok(())
    }

    pub async fn get_all_groups(&self) -> Result<Vec<Group>> {
        use futures::StreamExt;
        let mut cursor = self.groups.find(None, None).await?;
        let mut results = vec![];
        while let Some(group) = cursor.next().await {
            results.push(group?);
        }
        Ok(results)
    }

    pub fn invalidate_user_cache(&self, user_id: &str) {
        self.user_cache.remove(user_id);
    }

    pub fn invalidate_group_cache(&self, group_id: &str) {
        self.group_cache.remove(group_id);
    }

    pub async fn shutdown(&self) {
        info!("𖤐 Cerrando conexión a MongoDB...");
    }
}
