use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest;
use std::env;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    access_token: String,
    token_type: String,
}

pub fn configure_auth_routes() -> actix_web::Scope {
    web::scope("/auth")
        .route("/github", web::post().to(handle_github_auth))
}

async fn handle_github_auth(
    body: web::Json<AuthRequest>,
) -> impl Responder {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");
    
    let token_url = "https://github.com/login/oauth/access_token";
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", body.code.clone()),
    ];
    
    let client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            if let Ok(token_data) = resp.json::<AuthResponse>().await {
                HttpResponse::Ok().json(token_data)
            } else {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Failed to parse GitHub response"
                }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to authenticate with GitHub",
            "details": e.to_string()
        }))
    }
} 