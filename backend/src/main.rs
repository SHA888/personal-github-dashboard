use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{App, HttpServer};
use cookie::Key;
use sqlx::PgPool;
// removed unused import

mod handlers;
// mod error; // moved to lib.rs
mod routes;
// mod utils; // should be accessed via crate path
use personal_github_dashboard::utils::config::Config;

// Rate limiter removed for now

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
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Redis session store
    let redis_store = RedisSessionStore::new(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");

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
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(redis_store.clone(), Key::generate()))
            // Pass database pool to routes
            .app_data(actix_web::web::Data::new(pool.clone()))
            // Configure routes
            .configure(routes::init_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
