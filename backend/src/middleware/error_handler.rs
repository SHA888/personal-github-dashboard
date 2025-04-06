use actix_web::{error, http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: status_code.to_string(),
            message: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

pub fn error_handler() -> actix_web::middleware::ErrorHandlers {
    actix_web::middleware::ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
}

fn not_found<B>(res: error::ErrorResponse<B>) -> error::ErrorResponse<B> {
    let error_response = ErrorResponse {
        error: "Not Found".to_string(),
        message: "The requested resource was not found".to_string(),
    };

    res.into_builder()
        .json(error_response)
}

fn bad_request<B>(res: error::ErrorResponse<B>) -> error::ErrorResponse<B> {
    let error_response = ErrorResponse {
        error: "Bad Request".to_string(),
        message: "The request was invalid".to_string(),
    };

    res.into_builder()
        .json(error_response)
}

fn internal_server_error<B>(res: error::ErrorResponse<B>) -> error::ErrorResponse<B> {
    let error_response = ErrorResponse {
        error: "Internal Server Error".to_string(),
        message: "An internal server error occurred".to_string(),
    };

    res.into_builder()
        .json(error_response)
} 