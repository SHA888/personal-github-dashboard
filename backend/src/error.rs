use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal server error: {}", _0)]
    InternalError(String),
    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Not found: {}", _0)]
    NotFound(String),
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError(msg) => HttpResponse::InternalServerError().body(msg.clone()),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().body(msg.clone()),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
            AppError::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
        }
    }
}
