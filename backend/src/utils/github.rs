use reqwest::Client;
use serde::de::DeserializeOwned;

pub struct GithubClient {
    client: Client,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        GithubClient {
            client: Client::new(),
            token,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, reqwest::Error> {
        self.client
            .get(&format!("https://api.github.com{}", path))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "personal-github-dashboard")
            .send()
            .await?
            .json()
            .await
    }
}
