use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, Error, HttpResponse, Result,
};
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
            AppError::InternalServerError(message) => write!(f, "Internal Server Error: {}", message),
            AppError::DatabaseError(err) => write!(f, "Database Error: {}", err),
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
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
            },
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
            AppError::InternalServerError(_) | AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn handle_not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = ErrorResponse {
        error: "Not Found".to_string(),
        message: "The requested resource was not found".to_string(),
    };
    let response = HttpResponse::NotFound().json(error);
    let (req, _) = res.into_parts();
    let res = ServiceResponse::new(req, response.map_into_boxed_body());
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

fn handle_bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = ErrorResponse {
        error: "Bad Request".to_string(),
        message: "The request was invalid".to_string(),
    };
    let response = HttpResponse::BadRequest().json(error);
    let (req, _) = res.into_parts();
    let res = ServiceResponse::new(req, response.map_into_boxed_body());
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

fn handle_internal_server_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = ErrorResponse {
        error: "Internal Server Error".to_string(),
        message: "An internal server error occurred".to_string(),
    };
    let response = HttpResponse::InternalServerError().json(error);
    let (req, _) = res.into_parts();
    let res = ServiceResponse::new(req, response.map_into_boxed_body());
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

pub fn error_handler() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, handle_not_found)
        .handler(StatusCode::BAD_REQUEST, handle_bad_request)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, handle_internal_server_error)
}