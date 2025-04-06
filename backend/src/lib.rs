pub mod analytics;
pub mod models;
pub mod routes;
pub mod services;

use sqlx::PgPool;
use std::sync::Arc;
use crate::analytics::Analytics;

pub struct AppState {
    pub analytics: analytics::Analytics,
    pub pool: PgPool,
} 