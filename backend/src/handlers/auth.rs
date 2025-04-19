use crate::utils::config::Config;
use actix_web::{web, HttpResponse};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct OAuthRequest {
    pub code: String,
    pub state: String,
}

pub async fn login() -> HttpResponse {
    // Load OAuth config and generate state
    let cfg = Config::from_env();
    let state = Uuid::new_v4().to_string();
    // Build GitHub authorization URL
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&state={}&scope=repo,user",
        cfg.github_client_id,
        cfg.github_redirect_url,
        state
    );
    HttpResponse::Found()
        .insert_header(("Location", url))
        .finish()
}

pub async fn callback(query: web::Query<OAuthRequest>) -> HttpResponse {
    let query = query.into_inner();
    let cfg = Config::from_env();
    // Configure OAuth2 client
    let client = BasicClient::new(
        ClientId::new(cfg.github_client_id.clone()),
        Some(ClientSecret::new(cfg.github_client_secret.clone())),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(cfg.github_redirect_url.clone()).unwrap());
    // Exchange code for token
    match client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
    {
        Ok(token) => {
            // Return the access token in JSON (TODO: issue JWT and set cookie)
            HttpResponse::Ok().json(json!({"access_token": token.access_token().secret()}))
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("OAuth exchange error: {}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn login_redirect_should_redirect_to_github() {
        // Set env vars for test
        std::env::set_var("GITHUB_CLIENT_ID", "testid");
        std::env::set_var("GITHUB_REDIRECT_URL", "http://localhost/callback");
        // Initialize app with login route
        let app = test::init_service(App::new().route("/auth/login", web::get().to(login))).await;
        // Send request
        let req = test::TestRequest::get().uri("/auth/login").to_request();
        let resp = test::call_service(&app, req).await;
        // Assert redirect
        assert_eq!(resp.status(), StatusCode::FOUND);
        let loc = resp.headers().get("Location").unwrap().to_str().unwrap();
        assert!(loc.contains("client_id=testid"));
        assert!(loc.contains("redirect_uri=http://localhost/callback"));
        assert!(loc.contains("state="));
    }
}
