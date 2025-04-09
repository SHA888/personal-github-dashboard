use octocrab::models::orgs::Organization;
use octocrab::models::Author;
use octocrab::models::Repository;
use octocrab::{Error as OctocrabError, Octocrab};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UserOrgMembership {
    organization: Organization,
    #[serde(flatten)]
    _extra: std::collections::HashMap<String, serde_json::Value>,
}

pub struct GitHubAPIService {
    client: Octocrab,
}

impl GitHubAPIService {
    pub fn new(token: String) -> Self {
        let client = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to create Octocrab client");
        Self { client }
    }

    pub async fn get_organization(&self, org_name: &str) -> Result<Organization, OctocrabError> {
        let org = self.client.orgs(org_name).get().await?;
        Ok(org)
    }

    #[allow(dead_code)]
    pub async fn get_organization_repos(
        &self,
        org_name: &str,
    ) -> Result<Vec<Repository>, OctocrabError> {
        let mut repos = Vec::new();
        let mut page = 1u32;
        loop {
            let response = self
                .client
                .orgs(org_name)
                .list_repos()
                .per_page(100)
                .page(page)
                .send()
                .await?;

            let mut page_repos = response.items;
            if page_repos.is_empty() {
                break;
            }
            repos.append(&mut page_repos);
            page += 1;
        }
        Ok(repos)
    }

    pub async fn get_authenticated_user(&self) -> Result<Author, OctocrabError> {
        self.client.current().user().await
    }

    pub async fn list_my_organizations(&self) -> Result<Vec<Organization>, OctocrabError> {
        let first_page: octocrab::Page<Organization> =
            self.client.get("/user/orgs", None::<&()>).await?;

        Ok(first_page.items)
    }
}
