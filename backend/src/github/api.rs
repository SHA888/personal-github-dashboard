use octocrab::models::orgs::Organization;
use octocrab::models::Author;
use octocrab::models::Repository;
use octocrab::{Error as OctocrabError, Octocrab, Page};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
        Self {
            client: Octocrab::builder()
                .personal_token(token)
                .build()
                .expect("Failed to create GitHub client"),
        }
    }

    pub async fn get_organization(&self, org_name: &str) -> Result<Organization, OctocrabError> {
        self.client.orgs(org_name).get().await
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
                .page(page)
                .per_page(100)
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
        let first_page: Page<UserOrgMembership> = self
            .client
            .get("/user/memberships/orgs", None::<&()>)
            .await?;

        let memberships = first_page.items;

        let organizations = memberships
            .into_iter()
            .map(|mem| mem.organization)
            .collect();
        Ok(organizations)
    }
}
