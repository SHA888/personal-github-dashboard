use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{App, HttpServer};
use cookie::Key;
use sqlx::PgPool;
// removed unused import

mod handlers;
mod routes;
mod utils;
use crate::utils::config::Config;

<<<<<<< HEAD
// Rate limiter removed for now
=======
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
use utils::{config::Config, redis::RedisClient};
>>>>>>> d53f3e0 (Fix whitespace via pre-commit hook. All lints and formatting clean.)

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    env_logger::init();
    let config = Config::from_env();

    // Database pool
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

<<<<<<< HEAD
    // Redis session store
    let redis_store = RedisSessionStore::new(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");
=======
    // Initialize Redis client
    let redis_client = RedisClient::new(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");

    // Initialize HTTP client
    let http_client = Client::builder()
        .cookie_store(true) // Enable cookie handling if needed later
        .build()
        .expect("Failed to create HTTP client");
>>>>>>> d53f3e0 (Fix whitespace via pre-commit hook. All lints and formatting clean.)

    // Server binding
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| {
        format!(
            "0.0.0.0:{}",
            std::env::var("PORT").unwrap_or_else(|_| "8080".into())
        )
    });

    // Rate limiter removed for now

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.frontend_url)
            .allow_any_method()
            .allow_any_header()
<<<<<<< HEAD
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(redis_store.clone(), Key::generate()))
            // Pass database pool to routes
            .app_data(actix_web::web::Data::new(pool.clone()))
            // Configure routes
            .configure(routes::init_routes)
=======
            .supports_credentials(); // Enable credentials for auth cookies

        let app_config = config.clone(); // Clone config for use in the closure
        let db_pool_clone = db_pool.clone();
        let http_client_clone = http_client.clone();
        let redis_client_clone = redis_client.clone();

        App::new()
            .wrap(cors)
            .wrap(middleware::logging::RequestLogger)
            .wrap(middleware::error_handler::ErrorHandler)
            .wrap(middleware::auth::AuthMiddleware) // Add auth middleware
            .configure(routes::configure)
            .app_data(db_pool_clone)
            .app_data(actix_web::web::Data::new(app_config)) // Add config as app data
            .app_data(actix_web::web::Data::new(http_client_clone)) // Add Reqwest client
            .app_data(actix_web::web::Data::new(redis_client_clone)) // Add Redis client
>>>>>>> d53f3e0 (Fix whitespace via pre-commit hook. All lints and formatting clean.)
    })
    .bind(bind_addr)?
    .run()
    .await
}
