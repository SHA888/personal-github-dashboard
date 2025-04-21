use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repo_id: Option<Uuid>,
    pub r#type: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
