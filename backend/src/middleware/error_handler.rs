use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    DatabaseError(sqlx::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(message) => write!(f, "Not Found: {}", message),
            AppError::BadRequest(message) => write!(f, "Bad Request: {}", message),
            AppError::InternalServerError(message) => {
                write!(f, "Internal Server Error: {}", message)
            }
            AppError::DatabaseError(err) => write!(f, "Database Error: {}", err),
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(message) => HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: message.clone(),
            }),
            AppError::BadRequest(message) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: message.clone(),
            }),
            AppError::InternalServerError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                    message: message.clone(),
                })
            }
            AppError::DatabaseError(err) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database Error".to_string(),
                    message: err.to_string(),
                })
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) | AppError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
