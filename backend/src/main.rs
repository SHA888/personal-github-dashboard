use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use personal_github_dashboard::{
    api::configure_organizations_routes, github::GitHubService, AppState,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Get environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize GitHub service
    let github_service = Arc::new(GitHubService::new(github_token, pool.clone()));

    // Create app state
    let app_state = web::Data::new(AppState {
        pool: pool.clone(),
        github: github_service,
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(app_state.clone())
            .service(
                web::scope("/api").configure(|cfg| configure_organizations_routes(cfg, &app_state)),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[allow(dead_code)]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}
