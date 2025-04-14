// Middleware module for request/response processing

pub mod auth;
pub mod logging;
pub mod rate_limit;
pub mod error_handler;

// Re-export commonly used middleware
pub use auth::AuthMiddleware;
pub use logging::RequestLogger;
pub use rate_limit::RateLimiter;
pub use error_handler::ErrorHandler;
