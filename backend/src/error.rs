use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalError(String),

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "GitHub API Error: {}", _0)]
    GitHubError(String),

    #[display(fmt = "Rate Limit Exceeded: {}", _0)]
    RateLimitExceeded(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": msg
            })),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": msg
            })),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": msg
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": msg
            })),
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Database Error",
                "message": msg
            })),
            AppError::GitHubError(msg) => HttpResponse::BadGateway().json(json!({
                "error": "GitHub API Error",
                "message": msg
            })),
            AppError::RateLimitExceeded(msg) => HttpResponse::TooManyRequests().json(json!({
                "error": "Rate Limit Exceeded",
                "message": msg
            })),
        }
    }
}

// Add the From implementation for sqlx::Error
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        // You might want more sophisticated mapping here based on the sqlx error type
        AppError::DatabaseError(err.to_string())
    }
}

impl From<octocrab::Error> for AppError {
    fn from(err: octocrab::Error) -> Self {
        match err {
            octocrab::Error::GitHub { source, .. } => {
                let message = source.message;
                if source.documentation_url.is_some() && message.contains("rate limit") {
                    AppError::RateLimitExceeded(message)
                } else if message.contains("Not Found") {
                    AppError::NotFound(message)
                } else if message.contains("Unauthorized") || message.contains("Bad credentials") {
                    AppError::Unauthorized(message)
                } else {
                    AppError::GitHubError(message)
                }
            }
            _ => AppError::InternalError(format!("GitHub API error: {}", err)),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::InternalError(format!("HTTP request error: {}", err))
    }
}

#[allow(dead_code)]
fn json_error(error: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": error,
        "message": message
    })
}
