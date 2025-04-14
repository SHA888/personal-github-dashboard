// Middleware module for request/response processing

pub mod auth;
pub mod error_handler;
pub mod logging;
pub mod rate_limit;

// Re-export commonly used middleware
pub use auth::AuthMiddleware;
pub use error_handler::ErrorHandler;
pub use logging::RequestLogger;
pub use rate_limit::RateLimiter;
