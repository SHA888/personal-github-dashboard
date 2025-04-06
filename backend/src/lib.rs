pub mod analytics;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

use sqlx::PgPool;

pub struct AppState {
    pub analytics: analytics::Analytics,
    pub pool: PgPool,
}
