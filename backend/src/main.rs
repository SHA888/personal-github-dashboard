use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{App, HttpServer};
use cookie::Key;
use sqlx::PgPool;

mod handlers;
mod routes;
mod utils;
use crate::utils::config::Config;

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

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
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
