use serde::{Deserialize, Serialize};

/// Represents the claims in a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration timestamp (seconds since epoch)
    pub iat: usize,         // Issued at timestamp (seconds since epoch)
    pub roles: Vec<String>, // User roles
}
