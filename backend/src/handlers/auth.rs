use crate::utils::config::Config;
use crate::utils::jwt::create_jwt;
use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpResponse,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use octocrab::Octocrab;
use serde::Deserialize;
use time::Duration;
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
            // Fetch GitHub user info
            let octocrab = Octocrab::builder()
                .personal_token(token.access_token().secret().to_string())
                .build()
                .unwrap();
            let gh_user = octocrab.current().user().await;
            if let Ok(user) = gh_user {
                // Create JWT for user
                let jwt = create_jwt(&user.login, &Config::from_env().jwt_secret, 3600).unwrap();
                // Set cookie
                let cookie = Cookie::build("auth_token", jwt)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .path("/")
                    .max_age(Duration::seconds(3600))
                    .finish();
                // Redirect to home with cookie
                return HttpResponse::Found()
                    .cookie(cookie)
                    .insert_header(("Location", "/"))
                    .finish();
            }
            HttpResponse::InternalServerError().body("Failed to fetch GitHub user")
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

    #[actix_web::test]
    #[ignore]
    async fn callback_returns_internal_server_error_for_invalid_code() {
        // Set env vars for test
        std::env::set_var("GITHUB_CLIENT_ID", "testid");
        std::env::set_var("GITHUB_CLIENT_SECRET", "testsecret");
        std::env::set_var("GITHUB_REDIRECT_URL", "http://localhost/callback");
        // Initialize app with callback route
        let app =
            test::init_service(App::new().route("/auth/callback", web::get().to(callback))).await;
        // Send request with dummy code and state
        let req = test::TestRequest::get()
            .uri("/auth/callback?code=invalid&state=none")
            .to_request();
        let resp = test::call_service(&app, req).await;
        // Expect internal server error
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
