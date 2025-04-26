use crate::utils::cache::CacheableEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Option<Uuid>,
    pub repo_id: Option<Uuid>,
    pub r#type: String,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CacheableEntity for Activity {
    fn cache_key(&self) -> String {
        format!("activity:{}", self.id)
    }
    fn cache_key_from_id(id: &uuid::Uuid) -> String {
        format!("activity:{}", id)
    }
    fn cache_ttl() -> usize {
        crate::utils::cache::TTL_ACTIVITY
    }
}
