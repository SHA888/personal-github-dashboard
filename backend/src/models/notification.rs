use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "type")]
    pub type_: NotificationType,
    pub title: String,
    pub message: Option<String>,
    pub read: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    Repository,
    Organization,
    Security,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "type")]
    pub type_: NotificationType,
    pub enabled: bool,
    pub frequency: NotificationFrequency,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, Default)]
#[sqlx(type_name = "notification_frequency", rename_all = "snake_case")]
pub enum NotificationFrequency {
    Realtime,
    #[default]
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    #[serde(rename = "type")]
    pub type_: NotificationType,
    pub title: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationSettingRequest {
    #[serde(rename = "type")]
    pub type_: NotificationType,
    pub enabled: bool,
    pub frequency: NotificationFrequency,
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

impl std::str::FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "organization" => Ok(NotificationType::Organization),
            "repository" => Ok(NotificationType::Repository),
            _ => Err(format!("Invalid notification type: {}", s)),
        }
    }
}

impl From<String> for NotificationType {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(NotificationType::Repository)
    }
}

impl NotificationFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationFrequency::Realtime => "realtime",
            NotificationFrequency::Daily => "daily",
            NotificationFrequency::Weekly => "weekly",
            NotificationFrequency::Monthly => "monthly",
        }
    }
}

impl std::str::FromStr for NotificationFrequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "realtime" => Ok(NotificationFrequency::Realtime),
            "daily" => Ok(NotificationFrequency::Daily),
            "weekly" => Ok(NotificationFrequency::Weekly),
            _ => Err(format!("Invalid notification frequency: {}", s)),
        }
    }
}

impl From<String> for NotificationFrequency {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(NotificationFrequency::Realtime)
    }
}
