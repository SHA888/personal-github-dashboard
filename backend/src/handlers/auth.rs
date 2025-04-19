use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct OAuthRequest {
    pub code: String,
    pub state: String,
}

pub async fn login() -> HttpResponse {
    // TODO: Redirect to GitHub OAuth authorization URL
    HttpResponse::NotImplemented().finish()
}

pub async fn callback(_query: web::Query<OAuthRequest>) -> HttpResponse {
    // TODO: Handle GitHub OAuth callback (exchange code for access token)
    HttpResponse::NotImplemented().finish()
}
