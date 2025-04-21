use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
<<<<<<< HEAD
=======
    #[allow(dead_code)]
    pub github_personal_access_token: String,
>>>>>>> d53f3e0 (Fix whitespace via pre-commit hook. All lints and formatting clean.)
    pub redis_url: String,
    pub jwt_secret: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Config {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: std::env::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            github_client_id: std::env::var("GITHUB_CLIENT_ID")
                .expect("GITHUB_CLIENT_ID must be set"),
            github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET must be set"),
            github_redirect_url: std::env::var("GITHUB_CALLBACK_URL")
                .expect("GITHUB_CALLBACK_URL must be set"),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}
