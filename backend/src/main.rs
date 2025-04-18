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
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};
use db::DbPool;
use redis::Client as RedisClient;
use services::github_api::GitHubService;
use utils::config::Config;

// Define AppState
pub struct AppState {
    pub pool: DbPool,
    pub redis: RedisClient,
    pub github: GitHubService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Create database pool
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Create Redis client
    let redis_client =
        redis::Client::open(config.redis_url.as_str()).expect("Failed to create Redis client");

    // Create GitHub service
    let github_service = services::github_api::GitHubService::new(
        config.github_personal_access_token.clone(),
        config.redis_url.clone(),
    )
    .await
    .expect("Failed to create GitHub service");

    // Create app state
    let app_state = web::Data::new(AppState {
        pool: pool.clone(),
        redis: redis_client,
        github: github_service,
    });

    // Create app config
    let app_config = config.clone(); // Clone config for use in the closure

    // Start HTTP server
    let bind_addr = format!("{}:{}", config.server_host, config.server_port);
    log::info!("Starting server at http://{}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // TODO: Configure CORS properly
            .app_data(app_state.clone())
            .app_data(web::Data::new(app_config.clone()))
            .configure(routes::configure_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
