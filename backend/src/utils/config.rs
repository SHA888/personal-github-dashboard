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
    ///
    /// Reads required configuration parameters from environment variables and constructs a `Config` instance. Panics if any required variable is missing, except for `FRONTEND_URL`, which defaults to `"http://localhost:3001"` if not set. The `GITHUB_REDIRECT_URL` must exactly match the callback URL registered in your GitHub OAuth app settings.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::from_env();
    /// assert!(!config.database_url.is_empty());
    /// ```
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/personal_github_dashboard_dev"
                    .to_string()
            }),
            github_personal_access_token: std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
                .expect("GITHUB_PERSONAL_ACCESS_TOKEN must be set"),
            redis_url: std::env::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            github_client_id: std::env::var("GITHUB_CLIENT_ID")
                .expect("GITHUB_CLIENT_ID must be set"),
            github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET must be set"),
            // Only use GITHUB_REDIRECT_URL for clarity and consistency
            // This must match the callback URL registered in your GitHub OAuth app settings exactly (including port and path)
            github_redirect_url: std::env::var("GITHUB_REDIRECT_URL")
                .expect("GITHUB_REDIRECT_URL must be set"),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        }
    }
}
