// Handlers module for request processing logic

pub mod auth;
pub mod metrics;
pub mod notifications;
pub mod repositories;

// Re-export commonly used handlers
pub use auth::{github_callback, github_login, logout};
pub use metrics::{repository_metrics, user_metrics};
pub use notifications::{get_notification, list_notifications, mark_read};
pub use repositories::{get_repository, list_repositories, sync_repositories};
