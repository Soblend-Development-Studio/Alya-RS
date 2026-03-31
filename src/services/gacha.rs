use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Character {
    #[serde(default)]
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
    pub owner: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
}

pub struct GachaInner {
    pub characters: Vec<Character>,
    pub id_index: HashMap<String, usize>,
    pub name_index: HashMap<String, usize>,
    pub source_index: HashMap<String, Vec<usize>>,
    pub gender_index: HashMap<String, Vec<usize>>,
}

#[derive(Clone)]
pub struct GachaService {
    inner: Arc<RwLock<GachaInner>>,
}

impl GachaService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(GachaInner {
                characters: vec![],
                id_index: HashMap::new(),
                name_index: HashMap::new(),
                source_index: HashMap::new(),
                gender_index: HashMap::new(),
            })),
        }
    }

    pub async fn load(&self, data_path: &str) -> Result<()> {
        info!("Cargando personajes desde {}...", data_path);
        let content = tokio::fs::read_to_string(data_path).await?;
        let characters: Vec<Character> = serde_json::from_str(&content)?;
        let count = characters.len();

        let mut inner = self.inner.write().await;
        inner.characters = characters;
        inner.id_index.clear();
        inner.name_index.clear();
        inner.source_index.clear();
        inner.gender_index.clear();

        for (i, c) in inner.characters.iter().enumerate() {
            if !c.id.is_empty() {
                inner.id_index.insert(c.id.clone(), i);
            }
            if !c.name.is_empty() {
                inner.name_index.insert(c.name.to_lowercase(), i);
            }
            if let Some(src) = &c.source {
                inner.source_index.entry(src.to_lowercase()).or_default().push(i);
            }
            if let Some(g) = &c.gender {
                inner.gender_index.entry(g.to_lowercase()).or_default().push(i);
            }
        }

        info!("{} personajes cargados exitosamente", count);
        Ok(())
    }

    pub async fn get_random_character(&self) -> Option<Character> {
        use rand::Rng;
        let inner = self.inner.read().await;
        if inner.characters.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..inner.characters.len());
        Some(inner.characters[idx].clone())
    }

    pub async fn get_by_id(&self, id: &str) -> Option<Character> {
        let inner = self.inner.read().await;
        let idx = inner.id_index.get(id)?;
        inner.characters.get(*idx).cloned()
    }

    pub async fn get_by_name(&self, name: &str) -> Option<Character> {
        let inner = self.inner.read().await;
        let idx = inner.name_index.get(&name.to_lowercase())?;
        inner.characters.get(*idx).cloned()
    }

    pub async fn get_by_source(&self, source: &str) -> Vec<Character> {
        let inner = self.inner.read().await;
        let indices = inner.source_index.get(&source.to_lowercase()).cloned().unwrap_or_default();
        indices.iter().filter_map(|&i| inner.characters.get(i).cloned()).collect()
    }

    pub async fn get_total(&self) -> usize {
        self.inner.read().await.characters.len()
    }

    pub async fn search(&self, query: &str) -> Vec<Character> {
        let inner = self.inner.read().await;
        let q = query.to_lowercase();
        inner.characters
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.source.as_deref().unwrap_or("").to_lowercase().contains(&q)
            })
            .take(10)
            .cloned()
            .collect()
    }
}

impl Default for GachaService {
    fn default() -> Self {
        Self::new()
    }
}
