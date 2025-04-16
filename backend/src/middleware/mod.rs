// Middleware module for request/response processing

pub mod auth;
pub mod error_handler;
pub mod logging;
pub mod rate_limit;

// Re-export only necessary middleware components
// #[allow(unused_imports)] pub use auth::AuthMiddleware; // Commented out - will be used
#[allow(unused_imports)]
pub use error_handler::ErrorHandler;
#[allow(unused_imports)]
pub use logging::RequestLogger;
// #[allow(unused_imports)] pub use rate_limit::RateLimiter; // Commented out - potentially needed later
