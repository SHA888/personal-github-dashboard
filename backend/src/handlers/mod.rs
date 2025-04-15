// Handlers module for request processing logic

#[allow(unused_imports)]
#[allow(dead_code)]
pub mod auth;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod github;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod health;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod metrics;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod notifications;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod organizations;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod repositories;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod users;

// Re-export only what's being used
#[allow(unused_imports)]
pub use auth::{/* test_auth, */ get_current_user, github_callback, github_login, logout};
// pub use auth::test_auth; // Keep if needed for testing later
#[allow(unused_imports)]
pub use health::health_check;
#[allow(unused_imports)]
pub use metrics::{repository_metrics, user_metrics};
#[allow(unused_imports)]
pub use notifications::{get_notification, list_notifications, mark_read};
#[allow(unused_imports)]
pub use organizations::{
    get_organization, list_organizations, sync_my_organizations, sync_organization_by_name,
};
#[allow(unused_imports)]
pub use repositories::{get_repository, list_repositories, sync_repositories};
#[allow(unused_imports)]
pub use users::{get_user, list_users};
