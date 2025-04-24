use crate::utils::cache::CacheableEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CacheableEntity for User {
    fn cache_key(&self) -> String {
        format!("user:{}", self.id)
    }
    fn cache_key_from_id(id: &uuid::Uuid) -> String {
        format!("user:{}", id)
    }
    fn cache_ttl() -> usize {
        crate::utils::cache::TTL_USER
    }
}
