use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::{App, HttpServer};
use cookie::Key;
// removed unused import
// use std::time::Duration;

mod db;
mod handlers;
// mod error; // moved to lib.rs
mod routes;
// mod utils; // should be accessed via crate path
use personal_github_dashboard::utils::config::Config;
use personal_github_dashboard::utils::redis::RedisClient;
use std::sync::Arc;

// Insert minimal AppError usage to test import
fn _test_app_error_import() {
    let _ = personal_github_dashboard::error::AppError::InternalError("test".to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    env_logger::init();
    let config = Config::from_env();

    // Database pool
    let max_pool_size = std::env::var("PG_POOL_MAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    let pool = db::create_pg_pool(&config.database_url, max_pool_size).await;

    // Redis session store
    let redis_store = RedisSessionStore::new(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");

    // Redis client for caching
    let redis_client = RedisClient::new(&config.redis_url)
        .await
        .expect("Failed to create Redis client");
    let redis_client_data = actix_web::web::Data::new(redis_client);

    // Server binding
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| {
        format!(
            "0.0.0.0:{}",
            std::env::var("PORT").unwrap_or_else(|_| "8081".into())
        )
    });

    // Rate limiter
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(10)
        .finish()
        .expect("Failed to build governor config");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin
                    .to_str()
                    .map(|s| s.starts_with("http://localhost"))
                    .unwrap_or(false)
            })
            .allowed_methods(["GET", "POST", "PUT", "DELETE"])
            .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            .wrap(SessionMiddleware::new(redis_store.clone(), Key::generate()))
            // Pass database pool to routes
            .app_data(actix_web::web::Data::new(pool.clone()))
            // Pass redis client to routes
            .app_data(redis_client_data.clone())
            // Configure routes
            .configure(routes::init_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
