use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

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

fn json_error(error: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": error,
        "message": message
    })
}
