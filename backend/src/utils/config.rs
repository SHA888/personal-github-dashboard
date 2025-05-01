use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub github_personal_access_token: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
    pub frontend_url: String,
}

impl Config {
    /// Loads configuration values from environment variables.
    /// Reads required configuration parameters from environment variables and constructs a `Config` instance. Panics if any required variable is missing, except for `FRONTEND_URL`, which defaults to `"http://localhost:3001"` if not set. The `GITHUB_REDIRECT_URL` must exactly match the callback URL registered in your GitHub OAuth app settings.
    /// Loads configuration values from environment variables, providing defaults for optional fields.
    /// Attempts to read environment variables for all configuration parameters, falling back to default values for `DATABASE_URL` and `FRONTEND_URL` if they are not set. Panics if any required environment variable is missing.
    /// # Panics
    /// Panics if any of the following environment variables are not set: `GITHUB_PERSONAL_ACCESS_TOKEN`, `REDIS_URL`, `JWT_SECRET`, `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, or `GITHUB_REDIRECT_URL`.
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        // Debug logging for environment variables
        #[cfg(test)]
        {
            println!("Environment Variables:");
            for (key, value) in std::env::vars() {
                if key.contains("REDIS") || key.contains("TEST_REDIS") {
                    println!("{}: {}", key, value);
                }
            }
        }
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/personal_github_dashboard_dev"
                    .to_string()
            }),
            github_personal_access_token: std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
                .expect("GITHUB_PERSONAL_ACCESS_TOKEN must be set"),
            redis_url: std::env::var(if cfg!(test) {
                "TEST_REDIS_URL"
            } else {
                "REDIS_URL"
            })
            .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "test_secret_for_ci".to_string()),
            github_client_id: std::env::var("GITHUB_CLIENT_ID").unwrap_or_else(|_| {
                if cfg!(test) {
                    "test_client_id".to_string()
                } else {
                    panic!("GITHUB_CLIENT_ID must be set")
                }
            }),
            github_client_secret: std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_else(|_| {
                if cfg!(test) {
                    "test_client_secret".to_string()
                } else {
                    panic!("GITHUB_CLIENT_SECRET must be set")
                }
            }),
            github_redirect_url: std::env::var("GITHUB_REDIRECT_URL").unwrap_or_else(|_| {
                if cfg!(test) {
                    "http://localhost:8080/callback".to_string()
                } else {
                    panic!("GITHUB_REDIRECT_URL must be set")
                }
            }),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        }
    }
}
