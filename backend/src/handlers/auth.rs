use crate::{
    db::{DbPool, User},
    error::AppError,
    utils::{config::Config, jwt},
};
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpMessage, HttpResponse};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

// --- Structs for GitHub API Responses ---
#[derive(Deserialize, Debug)]
struct GitHubTokenResponse {
    access_token: String,
    scope: String,
    token_type: String,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GitHubUserResponse {
    id: i64, // Use i64 for GitHub IDs
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    html_url: Option<String>,
}
// ---------------------------------------

// Handler to initiate the GitHub OAuth login flow
#[get("/auth/github")]
pub async fn github_login(config: web::Data<Config>) -> Result<HttpResponse, AppError> {
    // Generate random state parameter for CSRF protection
    let mut rng = thread_rng();
    let state: [u8; 32] = rng.gen();
    let state = URL_SAFE_NO_PAD.encode(state);

    // Create state cookie
    let state_cookie = Cookie::build("oauth_state", &state)
        .path("/")
        .http_only(true)
        .secure(config.github_callback_url.starts_with("https"))
        .max_age(Duration::minutes(10))
        .finish();

    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user,user:email,repo&state={}",
        config.github_client_id,
        config.github_callback_url,
        state
    );

    Ok(HttpResponse::Found()
        .cookie(state_cookie)
        .append_header(("Location", auth_url))
        .finish())
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
    error: Option<String>,
    error_description: Option<String>,
}

// Handler for the GitHub OAuth callback
#[get("/auth/github/callback")]
async fn github_callback(
    query: web::Query<CallbackQuery>,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    // Check for errors in the callback query
    if let Some(error) = &query.error {
        let description = query
            .error_description
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string());
        return Err(AppError::Unauthorized(format!(
            "GitHub OAuth error: {} - {}",
            error, description
        )));
    }

    // Exchange the code for an access token
    let token = request_github_token(&query.code, &config).await?;

    // Get the user info from GitHub
    let user_info = get_github_user_info(&token).await?;

    // Create or update the user in our database
    let user = upsert_user(&pool, &user_info, &token).await?;

    // Create a JWT token for the user
    let jwt_token = jwt::create_token(user.id, config.jwt_secret.as_bytes())?;

    // Create a cookie that expires in 7 days
    let cookie = Cookie::build("auth_token", jwt_token)
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(7))
        .finish();

    // Return a response that sets the cookie and redirects to the dashboard
    Ok(HttpResponse::Found()
        .append_header(("Set-Cookie", cookie.to_string()))
        .append_header(("Location", "/dashboard"))
        .finish())
}

// --- Helper Functions ---

async fn request_github_token(code: &str, config: &Config) -> Result<String, AppError> {
    let client = reqwest::Client::new();

    let params = [
        ("client_id", config.github_client_id.as_str()),
        ("client_secret", config.github_client_secret.as_str()),
        ("code", code),
    ];

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await?;

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        error: Option<String>,
        error_description: Option<String>,
    }

    let token_response: TokenResponse = response.json().await?;

    if let Some(error) = token_response.error {
        let description = token_response
            .error_description
            .unwrap_or_else(|| "Unknown error".to_string());
        return Err(AppError::Unauthorized(format!(
            "GitHub token error: {} - {}",
            error, description
        )));
    }

    Ok(token_response.access_token)
}

async fn get_github_user_info(access_token: &str) -> Result<GitHubUserResponse, AppError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .header("User-Agent", "Personal-GitHub-Dashboard-Rust")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let response = handle_github_error(response).await?;
    response.json::<GitHubUserResponse>().await.map_err(|e| {
        AppError::InternalError(format!("Failed to parse GitHub user response: {}", e))
    })
}

async fn upsert_user(
    pool: &DbPool,
    github_user: &GitHubUserResponse,
    access_token: &str,
) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            id, github_id, login, name, email, avatar_url,
            access_token, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (github_id) DO UPDATE SET
            login = EXCLUDED.login,
            name = EXCLUDED.name,
            email = EXCLUDED.email,
            avatar_url = EXCLUDED.avatar_url,
            access_token = EXCLUDED.access_token,
            updated_at = EXCLUDED.updated_at
        RETURNING *
        "#,
        Uuid::new_v4(),
        github_user.id,
        github_user.login,
        github_user.name,
        github_user.email,
        github_user.avatar_url,
        access_token,
        OffsetDateTime::now_utc(),
        OffsetDateTime::now_utc()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

// Add a new function to handle GitHub API errors
async fn handle_github_error(response: reqwest::Response) -> Result<reqwest::Response, AppError> {
    match response.status() {
        status if status.is_success() => Ok(response),
        reqwest::StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized(
            "Invalid GitHub credentials".to_string(),
        )),
        reqwest::StatusCode::FORBIDDEN => Err(AppError::Unauthorized(
            "Access forbidden by GitHub".to_string(),
        )),
        reqwest::StatusCode::NOT_FOUND => {
            Err(AppError::NotFound("GitHub resource not found".to_string()))
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            // Get the reset time from headers if available
            if let Some(reset) = response.headers().get("x-ratelimit-reset") {
                if let Ok(reset_str) = reset.to_str() {
                    return Err(AppError::RateLimitExceeded(format!(
                        "GitHub rate limit exceeded. Resets at {}",
                        reset_str
                    )));
                }
            }
            Err(AppError::RateLimitExceeded(
                "GitHub rate limit exceeded".to_string(),
            ))
        }
        status => {
            let error_msg = format!("GitHub API error: {}", status);
            Err(AppError::GitHubError(error_msg))
        }
    }
}

#[post("/auth/logout")]
pub async fn logout() -> Result<HttpResponse, AppError> {
    let cookie = Cookie::build("auth_token", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .append_header(("Set-Cookie", cookie.to_string()))
        .finish())
}

// Test endpoint that requires authentication
pub async fn test_auth(req: actix_web::HttpRequest) -> HttpResponse {
    // Get claims from request extensions (set by auth middleware)
    let extensions = req.extensions();
    let claims = extensions.get::<jwt::Claims>().unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "message": "You are authenticated!",
        "user_id": claims.sub.to_string()
    }))
}

// Get current user information
pub async fn get_current_user(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    let user_id = {
        let extensions = req.extensions();
        let claims = extensions
            .get::<jwt::Claims>()
            .ok_or_else(|| AppError::Unauthorized("Missing JWT claims".to_string()))?;
        claims.sub
    };

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, github_id, login, name, email, avatar_url, access_token, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
        _ => AppError::DatabaseError(format!("Failed to fetch user: {}", e)),
    })?;

    let token = jwt::create_token(user.id, config.jwt_secret.as_bytes())?;
    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(7))
        .finish();

    Ok(HttpResponse::Ok()
        .append_header(("Set-Cookie", cookie.to_string()))
        .json(user))
}
