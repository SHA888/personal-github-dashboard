use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    DatabaseError(String),
    GitHubError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::GitHubError(msg) => write!(f, "GitHub error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                error: msg.to_string(),
            }),
            AppError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::GitHubError(msg) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: msg.to_string(),
            }),
            AppError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
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
