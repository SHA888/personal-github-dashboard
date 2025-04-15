#[allow(unused_imports)]
#[allow(dead_code)]
pub mod connection;
#[allow(unused_imports)]
#[allow(dead_code)]
mod metrics;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod models;
#[allow(unused_imports)]
#[allow(dead_code)]
mod notifications;

#[allow(unused_imports)]
pub use connection::{create_pool, DbPool};
#[allow(unused_imports)]
pub use metrics::{get_repository_metrics, record_repository_metrics};
#[allow(unused_imports)]
pub use models::*;
#[allow(unused_imports)]
pub use notifications::{
    create_notification, get_notification_settings, get_notifications_by_user,
    mark_notification_read, update_notification_setting,
};

// Re-export models for convenience
// (Already done in models/mod.rs, but can be useful here too)
// pub use crate::models::{User, Repository, Notification, ...};
