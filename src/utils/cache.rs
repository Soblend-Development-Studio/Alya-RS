use std::time::{Duration, Instant};
use dashmap::DashMap;
use std::sync::Arc;

struct Entry {
    value: String,
    expires_at: Instant,
}

#[derive(Clone)]
pub struct CacheManager {
    store: Arc<DashMap<String, Entry>>,
}

impl CacheManager {
    pub fn new() -> Self {
        let manager = Self {
            store: Arc::new(DashMap::new()),
        };
        // Spawn cleanup task
        let store = manager.store.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let now = Instant::now();
                store.retain(|_, v| v.expires_at > now);
            }
        });
        manager
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let entry = self.store.get(key)?;
        if entry.expires_at > Instant::now() {
            Some(entry.value.clone())
        } else {
            drop(entry);
            self.store.remove(key);
            None
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn set(&self, key: &str, value: &str, ttl_secs: u64) {
        self.store.insert(key.to_string(), Entry {
            value: value.to_string(),
            expires_at: Instant::now() + Duration::from_secs(ttl_secs),
        });
    }

    pub fn delete(&self, key: &str) {
        self.store.remove(key);
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
