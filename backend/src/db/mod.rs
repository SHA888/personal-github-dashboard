#[allow(unused_imports)]
#[allow(dead_code)]
pub mod connection;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod metrics;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod models;
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod notifications;

#[allow(unused_imports)]
pub use connection::{create_pool, DbPool};
#[allow(unused_imports)]
pub use metrics::*;
#[allow(unused_imports)]
pub use models::*;
#[allow(unused_imports)]
pub use notifications::*;

// Re-export models for convenience
// (Already done in models/mod.rs, but can be useful here too)
// pub use crate::models::{User, Repository, Notification, ...};
