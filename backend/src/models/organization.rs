use crate::utils::cache::CacheableEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CacheableEntity for Organization {
    fn cache_key(&self) -> String {
        format!("org:{}", self.id)
    }
    fn cache_key_from_id(id: &uuid::Uuid) -> String {
        format!("org:{}", id)
    }
    fn cache_ttl() -> usize {
        crate::utils::cache::TTL_REPO
    }
}
