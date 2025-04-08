use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "Database error: {}", err),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(err) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: ErrorDetails {
                    code: "DATABASE_ERROR".to_string(),
                    message: err.to_string(),
                },
            }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                error: ErrorDetails {
                    code: "NOT_FOUND".to_string(),
                    message: msg.to_string(),
                },
            }),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                error: ErrorDetails {
                    code: "BAD_REQUEST".to_string(),
                    message: msg.to_string(),
                },
            }),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
} 