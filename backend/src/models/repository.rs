use crate::utils::cache::CacheableEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Repository {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

impl CacheableEntity for Repository {
    fn cache_key(&self) -> String {
        format!("repo:{}", self.id)
    }
    fn cache_key_from_id(id: &uuid::Uuid) -> String {
        format!("repo:{}", id)
    }
    fn cache_ttl() -> usize {
        crate::utils::cache::TTL_REPO
    }
}
