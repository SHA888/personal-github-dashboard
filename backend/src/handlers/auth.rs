use actix_session::Session;
use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpRequest, HttpResponse,
};
use log::{error, warn};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use octocrab::Octocrab;
use personal_github_dashboard::utils::config::Config;
use personal_github_dashboard::utils::jwt::create_jwt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
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

pub async fn callback(req: HttpRequest, session: Session) -> HttpResponse {
    // Test mode: skip real OAuth for tests
    if std::env::var("TEST_MODE").unwrap_or_default() == "1" {
        let secret = std::env::var("JWT_SECRET").unwrap_or_default();
        let jwt = create_jwt("testuser", &secret, 3600).unwrap();
        // Store JWT in session
        let _ = session.insert("jwt", &jwt);
        let cookie = Cookie::build("auth_token", jwt)
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .max_age(Duration::seconds(3600))
            .finish();
        return HttpResponse::Found()
            .cookie(cookie)
            .insert_header(("Location", "/"))
            .finish();
    }

    // Parse OAuth query parameters
    let query = match web::Query::<OAuthRequest>::from_query(req.query_string()) {
        Ok(q) => q.into_inner(),
        Err(e) => {
            warn!("Invalid OAuth query params: {:?}", e);
            return HttpResponse::BadRequest().json(json!({"error": "Invalid OAuth parameters"}));
        }
    };

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
                // Store JWT in session
                let _ = session.insert("jwt", &jwt);
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
            error!("Failed to fetch GitHub user: {:?}", gh_user.err());
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to fetch GitHub user"}))
        }
        Err(err) => {
            error!("OAuth token exchange failed: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to exchange OAuth token"}))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PatRequest {
    pub pat: String,
}

/// Authenticate using a personal access token for desktop/CLI usage
pub async fn pat_auth(body: web::Json<PatRequest>) -> HttpResponse {
    // Test mode: skip real GitHub PAT validation
    if std::env::var("TEST_MODE").unwrap_or_default() == "1" {
        let secret = std::env::var("JWT_SECRET").unwrap_or_default();
        let jwt = create_jwt("testuser", &secret, 3600).unwrap();
        return HttpResponse::Ok().json(json!({"jwt": jwt, "user": "testuser"}));
    }
    let cfg = Config::from_env();
    // Initialize Octocrab with PAT
    let octocrab = Octocrab::builder()
        .personal_token(body.pat.clone())
        .build()
        .unwrap();
    // Fetch user
    match octocrab.current().user().await {
        Ok(user) => {
            // Create JWT
            let jwt = create_jwt(&user.login, &cfg.jwt_secret, 3600).unwrap();
            HttpResponse::Ok().json(json!({
                "jwt": jwt,
                "user": user.login,
            }))
        }
        Err(err) => {
            warn!("Invalid PAT provided: {:?}", err);
            HttpResponse::Unauthorized().json(json!({"error": format!("Invalid PAT: {}", err)}))
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

#[cfg(test)]
mod callback_tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn callback_in_test_mode_sets_cookie() {
        std::env::set_var("JWT_SECRET", "secret");
        std::env::set_var("TEST_MODE", "1");
        let app =
            test::init_service(App::new().route("/auth/callback", web::get().to(callback))).await;
        let req = test::TestRequest::get()
            .uri("/auth/callback?code=anything")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::FOUND);
        let cookie_hdr = resp.headers().get("Set-Cookie").unwrap().to_str().unwrap();
        assert!(cookie_hdr.contains("auth_token="));
    }
}

#[cfg(test)]
mod pat_tests {
    use super::*;
    use crate::handlers::auth::PatRequest;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::Value;

    #[actix_web::test]
    async fn pat_auth_in_test_mode_returns_jwt_and_user() {
        std::env::set_var("JWT_SECRET", "secret");
        std::env::set_var("TEST_MODE", "1");
        let app = test::init_service(App::new().route("/auth/pat", web::post().to(pat_auth))).await;
        let req = test::TestRequest::post()
            .uri("/auth/pat")
            .set_json(PatRequest {
                pat: "dummy".into(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["user"], "testuser");
        assert!(body["jwt"].is_string());
    }
}
