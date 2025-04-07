use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i32,
    pub owner: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: i32,
    pub repository_id: i32,
    pub sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub created_at: DateTime<Utc>,
}
