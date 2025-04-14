// Handlers module for request processing logic

pub mod auth;
pub mod repositories;
pub mod notifications;
pub mod metrics;

// Re-export commonly used handlers
pub use auth::{github_login, github_callback, logout};
pub use repositories::{list_repositories, get_repository, sync_repositories};
pub use notifications::{list_notifications, get_notification, mark_read};
pub use metrics::{repository_metrics, user_metrics};
