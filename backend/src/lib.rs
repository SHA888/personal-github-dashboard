use sqlx::PgPool;
use std::sync::Arc;
use crate::analytics::Analytics;

pub mod models;
pub mod routes;
pub mod analytics;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub analytics: Arc<Analytics>,
} 