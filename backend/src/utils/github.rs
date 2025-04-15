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
            .get(format!("https://api.github.com{}", path))
            .header("User-Agent", "Personal-GitHub-Dashboard-Rust")
            .header("Accept", "application/vnd.github.v3+json")
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<T>()
            .await
    }
}
