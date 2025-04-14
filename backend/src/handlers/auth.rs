use actix_web::{web, HttpResponse};
use crate::error::AppError;
use crate::utils::config::Config;
use crate::db::{DbPool, User}; // Assuming a User model exists in db module
use reqwest::Client;
use serde::Deserialize;

// --- Structs for GitHub API Responses ---
#[derive(Deserialize, Debug)]
struct GitHubTokenResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUserResponse {
    id: i64,          // Use i64 for GitHub IDs
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    html_url: Option<String>,
}
// ---------------------------------------

// Handler to initiate the GitHub OAuth login flow
pub async fn github_login(
    config: web::Data<Config>
) -> Result<HttpResponse, AppError> {
    // TODO: Add CSRF protection using the state parameter
    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user,user:email", // Added user:email scope
        config.github_client_id,
        config.github_callback_url
    );
    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}

// Handler for the GitHub OAuth callback
pub async fn github_callback(
    query: web::Query<CallbackQuery>,
    config: web::Data<Config>,
    pool: web::Data<DbPool>,
    http_client: web::Data<Client>, // Inject Reqwest client
) -> Result<HttpResponse, AppError> {
    let code = &query.code;
    // TODO: Verify the state parameter (if used)

    // --- 1. Exchange code for access token ---
    let token_response = request_github_token(code, &config, &http_client).await?;

    // --- 2. Fetch user info using access token ---
    let github_user = get_github_user_info(&token_response.access_token, &http_client).await?;

    // --- 3. Upsert user in database ---
    let user = upsert_user(&pool, &github_user).await?;

    // --- 4. Generate JWT (Subtask 2.4) ---
    // TODO: Implement JWT generation using the 'user' object (e.g., user.id)
    let jwt_token = "temporary_jwt_placeholder"; // Placeholder

    // --- 5. Set JWT in secure cookie (Subtask 2.6) ---
    // TODO: Implement secure cookie setting
    let cookie = actix_web::cookie::Cookie::build("auth_token", jwt_token)
        .path("/")
        .http_only(true)
        // .secure(true) // Uncomment in production with HTTPS
        .finish();

    // --- 6. Redirect back to frontend ---
    let frontend_dashboard_url = "/"; // Or get from config?
    Ok(HttpResponse::Found()
        .cookie(cookie)
        .append_header(("Location", frontend_dashboard_url))
        .finish())
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    code: String,
    // state: Option<String>, // Optional: Add state for CSRF protection
}

// --- Helper Functions ---

async fn request_github_token(
    code: &str,
    config: &Config,
    client: &Client,
) -> Result<GitHubTokenResponse, AppError> {
    let params = [
        ("client_id", &config.github_client_id),
        ("client_secret", &config.github_client_secret),
        ("code", &code.to_string()),
        ("redirect_uri", &config.github_callback_url),
    ];

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed request to GitHub token endpoint: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::InternalError(format!(
            "GitHub token exchange failed: {}",
            error_text
        )));
    }

    response
        .json::<GitHubTokenResponse>()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse GitHub token response: {}", e)))
}

async fn get_github_user_info(
    access_token: &str,
    client: &Client,
) -> Result<GitHubUserResponse, AppError> {
    client
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .header("User-Agent", "Personal-GitHub-Dashboard-Rust") // GitHub requires a User-Agent
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed request to GitHub user endpoint: {}", e)))?
        .json::<GitHubUserResponse>()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse GitHub user response: {}", e)))
}

async fn upsert_user(
    pool: &DbPool,
    github_user: &GitHubUserResponse,
) -> Result<User, AppError> {
    // Use INSERT ... ON CONFLICT to handle existing users
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (github_id, login, name, email, avatar_url, html_url)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (github_id) DO UPDATE SET
            login = EXCLUDED.login,
            name = EXCLUDED.name,
            email = EXCLUDED.email,
            avatar_url = EXCLUDED.avatar_url,
            html_url = EXCLUDED.html_url,
            last_login_at = NOW()
        RETURNING id, github_id, login, name, email, avatar_url, html_url, created_at, updated_at, last_login_at
        "#,
        github_user.id,
        github_user.login,
        github_user.name,
        github_user.email,
        github_user.avatar_url,
        github_user.html_url
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to upsert user: {}", e)))?;

    Ok(user)
}
// -----------------------

pub async fn logout() -> HttpResponse {
    // TODO: Implement logout
    HttpResponse::NotImplemented().finish()
}
