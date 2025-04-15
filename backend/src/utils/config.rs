use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub github_personal_access_token: String,
    pub redis_url: String,
    pub port: u16,
    // GitHub OAuth Configuration
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_callback_url: String,
    // JWT Configuration
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    // Frontend Configuration
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            github_personal_access_token: env::var("GITHUB_PERSONAL_ACCESS_TOKEN")?,
            redis_url: env::var("REDIS_URL")?,
            port: env::var("PORT")?.parse().unwrap_or(8080),
            // GitHub OAuth Configuration
            github_client_id: env::var("GITHUB_CLIENT_ID")?,
            github_client_secret: env::var("GITHUB_CLIENT_SECRET")?,
            github_callback_url: env::var("GITHUB_CALLBACK_URL")?,
            // JWT Configuration
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expires_in: env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string()),
            // Frontend Configuration
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }

    // Helper method to validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.github_client_id.is_empty() {
            return Err("GITHUB_CLIENT_ID is not set".to_string());
        }
        if self.github_client_secret.is_empty() {
            return Err("GITHUB_CLIENT_SECRET is not set".to_string());
        }
        if self.github_callback_url.is_empty() {
            return Err("GITHUB_CALLBACK_URL is not set".to_string());
        }
        if self.jwt_secret.is_empty()
            || self.jwt_secret == "generate_a_strong_random_secret_replace_this"
        {
            return Err("JWT_SECRET is not properly set".to_string());
        }
        if self.frontend_url.is_empty() {
            return Err("FRONTEND_URL is not properly set".to_string());
        }
        Ok(())
    }
}
