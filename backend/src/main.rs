mod api;
mod db;
mod error;
mod github;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use utils::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Load and validate configuration
    let config = Config::from_env().expect("Failed to load configuration from environment");

    if let Err(err) = config.validate() {
        panic!("Configuration error: {}", err);
    }

    // Initialize database connection
    let db_pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    // Get port from configuration
    let bind_addr = format!("127.0.0.1:{}", config.port);

    println!("Starting server on {}", bind_addr);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let app_config = config.clone(); // Clone config for use in the closure

        App::new()
            .wrap(cors)
            .wrap(middleware::logging::RequestLogger::default())
            .wrap(middleware::error_handler::ErrorHandler::default())
            .configure(routes::configure)
            .app_data(db_pool.clone())
            .app_data(actix_web::web::Data::new(app_config)) // Add config as app data
    })
    .bind(&bind_addr)?
    .run()
    .await
}
