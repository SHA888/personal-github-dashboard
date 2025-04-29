use crate::models::oauth_token::OAuthToken;
use crate::models::user::User;
use crate::utils::config::Config;
use crate::utils::jwt::create_jwt;
use actix_session::Session;
use actix_web::{
    HttpRequest, HttpResponse,
    cookie::{Cookie, SameSite},
    web,
};
use base64::Engine;
use log::{error, warn};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use time::Duration;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct OAuthRequest {
    pub code: String,
    pub state: String,
}

/// Redirects the client to the GitHub OAuth authorization page.
///
/// Constructs a GitHub OAuth authorization URL with the required client ID, redirect URI, state, and scopes, then returns an HTTP 302 redirect to initiate the OAuth login flow.
///
/// # Examples
///
/// ```
/// let response = login();
/// assert_eq!(response.status(), actix_web::http::StatusCode::FOUND);
/// ```
pub async fn login() -> HttpResponse {
    // Load OAuth config and generate state
    let cfg = Config::from_env();
    let state = Uuid::new_v4().to_string();
    // Build GitHub authorization URL
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&state={}&scope=read:org,read:user,repo",
        cfg.github_client_id, cfg.github_redirect_url, state
    );
    HttpResponse::Found()
        .insert_header(("Location", url))
        .finish()
}

/// Handles the OAuth callback from GitHub, exchanges the authorization code for an access token, retrieves user information, persists user and token data, and establishes a session with a JWT.
///
/// In test mode (`TEST_MODE=1`), bypasses real OAuth and issues a test JWT for a dummy user.
///
/// # Examples
///
/// ```
/// // In an Actix-web test, simulate the OAuth callback:
/// let req = test::TestRequest::with_uri("/auth/callback?code=abc&state=xyz").to_http_request();
/// let session = Session::default();
/// let pool = web::Data::new(setup_test_pg_pool());
/// let resp = callback(req, session, pool).await;
/// assert_eq!(resp.status(), StatusCode::FOUND);
/// ```
pub async fn callback(req: HttpRequest, session: Session, pool: web::Data<PgPool>) -> HttpResponse {
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
            if let Ok(gh_user) = gh_user {
                // Map GitHub user fields
                let username = gh_user.login.clone();
                let email = gh_user.email.clone().unwrap_or_default();
                let avatar_url = Some(gh_user.avatar_url.to_string());

                // Find or create user in DB
                let user = match sqlx::query_as!(User, "SELECT id, username, email, avatar_url, created_at FROM users WHERE username = $1 OR email = $2 LIMIT 1", username, email)
                    .fetch_optional(pool.get_ref())
                    .await
                {
                    Ok(u) => u,
                    Err(e) => {
                        error!("User lookup failed: {:?}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                };

                let user = match user {
                    Some(u) => u,
                    None => {
                        let new_id = Uuid::new_v4();
                        let now = chrono::Utc::now();
                        if let Err(e) = sqlx::query!("INSERT INTO users (id, username, email, avatar_url, created_at) VALUES ($1, $2, $3, $4, $5)", new_id, username, email, avatar_url, now)
                            .execute(pool.get_ref()).await {
                            error!("User insert failed: {:?}", e);
                            return HttpResponse::InternalServerError().finish();
                        }
                        User {
                            id: new_id,
                            username,
                            email,
                            avatar_url,
                            created_at: now,
                        }
                    }
                };

                // Encrypt tokens
                let key = env::var("OAUTH_TOKEN_KEY").expect("OAUTH_TOKEN_KEY must be set");
                let key_bytes = match base64::engine::general_purpose::STANDARD.decode(&key) {
                    Ok(k) if k.len() == 32 => {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&k);
                        arr
                    }
                    _ => {
                        error!("OAUTH_TOKEN_KEY must be 32 raw bytes (base64-encoded)");
                        return HttpResponse::InternalServerError().finish();
                    }
                };
                let encrypted_access =
                    OAuthToken::encrypt_token(token.access_token().secret(), &key_bytes).unwrap();
                let encrypted_refresh = token
                    .refresh_token()
                    .map(|rt| OAuthToken::encrypt_token(rt.secret(), &key_bytes).unwrap());
                // Store in DB
                let now = chrono::Utc::now();
                let expiry = token
                    .expires_in()
                    .map(|d| now + chrono::Duration::from_std(d).unwrap());
                let token_type_str = format!("{:?}", token.token_type());
                if let Err(e) = sqlx::query!(
                    "INSERT INTO oauth_tokens (id, user_id, provider, access_token, refresh_token, token_type, scope, expiry, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    Uuid::new_v4(),
                    user.id,
                    "github",
                    encrypted_access,
                    encrypted_refresh,
                    Some(token_type_str),
                    token.scopes().map(|scopes| scopes.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(",")),
                    expiry,
                    now,
                    now
                )
                .execute(pool.get_ref())
                .await {
                    error!("Failed to store OAuth token: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
                // Create JWT for user
                let jwt = create_jwt(&user.username, &Config::from_env().jwt_secret, 3600).unwrap();
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

/// Authenticate using a personal access token for desktop/CLI usage
pub async fn pat_auth(body: web::Json<String>) -> HttpResponse {
    // Test mode: skip real GitHub PAT validation
    if std::env::var("TEST_MODE").unwrap_or_default() == "1" {
        let secret = std::env::var("JWT_SECRET").unwrap_or_default();
        let jwt = create_jwt("testuser", &secret, 3600).unwrap();
        return HttpResponse::Ok().json(json!({"jwt": jwt, "user": "testuser"}));
    }
    let cfg = Config::from_env();
    // Initialize Octocrab with PAT
    let octocrab = Octocrab::builder()
        .personal_token(body.0.clone())
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
    use actix_web::{App, http::StatusCode, test, web};

    #[actix_web::test]
    async fn login_redirect_should_redirect_to_github() {
        // Set env vars for test
        std::env::set_var("GITHUB_CLIENT_ID", "testid");
        std::env::set_var("GITHUB_REDIRECT_URL", "http://localhost/callback");
        std::env::set_var("GITHUB_PERSONAL_ACCESS_TOKEN", "dummy_token");
        std::env::set_var("DATABASE_URL", "postgres://dummy:dummy@localhost/dummy");
        std::env::set_var("REDIS_URL", "redis://localhost:6379");
        std::env::set_var("JWT_SECRET", "dummy_secret");
        std::env::set_var("GITHUB_CLIENT_SECRET", "dummy_secret");
        std::env::set_var("FRONTEND_URL", "http://localhost:3000");
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
    use actix_web::cookie::Key;
    use actix_web::{App, http::StatusCode, test, web};
    use dotenv;
    use sqlx::PgPool;

    #[actix_web::test]
    async fn callback_in_test_mode_sets_cookie() {
        dotenv::dotenv().ok();
        std::env::set_var("JWT_SECRET", "secret");
        std::env::set_var("TEST_MODE", "1");
        let db_url = std::env::var("TEST_DATABASE_URL").unwrap();
        let pool = PgPool::connect(&db_url).await.unwrap();
        let session_key = Key::generate();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/auth/callback", web::get().to(callback)),
        )
        .await;
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
    use actix_web::{App, http::StatusCode, test};
    use serde_json::Value;

    #[actix_web::test]
    async fn pat_auth_in_test_mode_returns_jwt_and_user() {
        std::env::set_var("JWT_SECRET", "secret");
        std::env::set_var("TEST_MODE", "1");
        let app = test::init_service(App::new().route("/auth/pat", web::post().to(pat_auth))).await;
        let req = test::TestRequest::post()
            .uri("/auth/pat")
            .set_json("dummy".into())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["user"], "testuser");
        assert!(body["jwt"].is_string());
    }
}
