use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String, // Using type_ to avoid Rust keyword
    pub title: String,
    pub message: Option<String>,
    pub read: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NotificationSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String,
    pub enabled: Option<bool>,
    pub frequency: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Enums for type safety
#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Repository,
    Organization,
    Security,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Realtime,
    Daily,
    Weekly,
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationType::Repository => "repository",
            NotificationType::Organization => "organization",
            NotificationType::Security => "security",
            NotificationType::System => "system",
        }
    }
}

impl NotificationFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationFrequency::Realtime => "realtime",
            NotificationFrequency::Daily => "daily",
            NotificationFrequency::Weekly => "weekly",
        }
    }
}
