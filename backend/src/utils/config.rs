use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_callback_url: String,
    pub github_personal_access_token: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub server_host: String,
    pub server_port: u16,
}

#[allow(dead_code)]
impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            github_client_id: env::var("GITHUB_CLIENT_ID")?,
            github_client_secret: env::var("GITHUB_CLIENT_SECRET")?,
            github_callback_url: env::var("GITHUB_CALLBACK_URL")?,
            github_personal_access_token: env::var("GITHUB_PERSONAL_ACCESS_TOKEN")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expires_in: env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
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
        if self.jwt_secret.is_empty() {
            return Err("JWT_SECRET is not set".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_from_env() {
        // Set test environment variables
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("REDIS_URL", "redis://localhost:6379");
        env::set_var("GITHUB_CLIENT_ID", "test_client_id");
        env::set_var("GITHUB_CLIENT_SECRET", "test_client_secret");
        env::set_var("GITHUB_CALLBACK_URL", "http://localhost:3000/callback");
        env::set_var("JWT_SECRET", "test_secret");
        env::set_var("PORT", "9000");

        let config = Config::from_env().unwrap();

        assert_eq!(config.database_url, "postgres://test:test@localhost/test");
        assert_eq!(config.redis_url, "redis://localhost:6379");
        assert_eq!(config.github_client_id, "test_client_id");
        assert_eq!(config.github_client_secret, "test_client_secret");
        assert_eq!(config.github_callback_url, "http://localhost:3000/callback");
        assert_eq!(config.jwt_secret, "test_secret");
        assert_eq!(config.server_host, "localhost");
        assert_eq!(config.server_port, 9000);
    }

    #[test]
    fn test_config_default_values() {
        env::remove_var("PORT");
        env::remove_var("SERVER_HOST");

        let config = Config::from_env().unwrap();

        assert_eq!(config.server_host, "localhost");
        assert_eq!(config.server_port, 8080);
    }
}
