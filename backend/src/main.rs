use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Starting backend on {}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(health)
    })
        .bind(bind_addr)?
        .run()
        .await
}
