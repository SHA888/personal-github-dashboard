// mod api;
mod db;
mod error;
mod github;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
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
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create pool");

    // Initialize HTTP client
    let http_client = Client::builder()
        .cookie_store(true) // Enable cookie handling if needed later
        .build()
        .expect("Failed to create HTTP client");

    // Get Redis URL from configuration
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // Initialize GitHub service
    let github_service = services::github_api::GitHubService::new(
        config.github_personal_access_token.clone(),
        redis_url,
    )
    .await
    .expect("Failed to create GitHub service");

    // Get port from configuration
    let bind_addr = format!("127.0.0.1:{}", config.port);

    println!("Starting server on {}", bind_addr);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials(); // Enable credentials for auth cookies

        let app_config = config.clone(); // Clone config for use in the closure
        let db_pool_clone = db_pool.clone();
        let http_client_clone = http_client.clone();
        let github_service_clone = github_service.clone();

        App::new()
            .wrap(cors)
            .wrap(middleware::logging::RequestLogger)
            .wrap(middleware::error_handler::ErrorHandler)
            .wrap(middleware::auth::AuthMiddleware) // Add auth middleware
            .configure(routes::configure)
            .app_data(db_pool_clone)
            .app_data(actix_web::web::Data::new(app_config)) // Add config as app data
            .app_data(actix_web::web::Data::new(http_client_clone)) // Add Reqwest client
            .app_data(actix_web::web::Data::new(github_service_clone)) // Add GitHub service
    })
    .bind(&bind_addr)?
    .run()
    .await
}
