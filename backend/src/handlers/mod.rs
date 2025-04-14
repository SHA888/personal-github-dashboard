// Handlers module for request processing logic

pub mod auth;
pub mod health;
pub mod metrics;
pub mod notifications;
pub mod organizations;
pub mod repositories;
pub mod users;

// Re-export commonly used handlers
pub use auth::{github_callback, github_login};
pub use health::health_check;
pub use metrics::{repository_metrics, user_metrics};
pub use notifications::{get_notification, list_notifications, mark_read};
pub use organizations::{
    get_organization, list_organizations, sync_my_organizations, sync_organization_by_name,
};
pub use repositories::{get_repository, list_repositories, sync_repositories};
pub use users::{get_user, list_users};
