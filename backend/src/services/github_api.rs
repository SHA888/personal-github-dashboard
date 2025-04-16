use octocrab::models::{orgs::MembershipInvitation, orgs::Organization, Author, Repository};
use octocrab::Octocrab;
use std::sync::Arc;

#[derive(Clone)]
pub struct GitHubService {
    client: Arc<Octocrab>,
}

impl GitHubService {
    pub fn new(token: String) -> Self {
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to build Octocrab client");

        Self {
            client: Arc::new(octocrab),
        }
    }

    pub async fn get_organization(&self, org_name: &str) -> Result<Organization, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        self.client.orgs(org_name).get().await
    }

    #[allow(dead_code)]
    pub async fn get_organization_repos(
        &self,
        org_name: &str,
    ) -> Result<Vec<Repository>, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        let pages = self
            .client
            .orgs(org_name)
            .list_repos()
            .per_page(100)
            .send()
            .await?;

        self.client.all_pages(pages).await
    }

    pub async fn get_authenticated_user(&self) -> Result<Author, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        self.client.current().user().await
    }

    pub async fn list_my_organizations(&self) -> Result<Vec<Organization>, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        let pages = self
            .client
            .current()
            .list_org_memberships_for_authenticated_user()
            .per_page(100)
            .send()
            .await?;

        let memberships: Vec<MembershipInvitation> = self.client.all_pages(pages).await?;

        // Extract organization details from each membership
        let organizations = memberships
            .into_iter()
            .map(|membership| membership.organization)
            .collect();

        Ok(organizations)
    }

    pub async fn list_my_repositories(&self) -> Result<Vec<Repository>, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        let pages = self
            .client
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .send()
            .await?;

        self.client.all_pages(pages).await
    }

    #[allow(dead_code)]
    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<octocrab::models::Author, octocrab::Error> {
        self.client
            .get(format!("/users/{}", username), None::<&()>)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_repository_details(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Repository, octocrab::Error> {
        // TODO: Implement rate limiting logic here
        self.client.repos(owner, repo).get().await
    }
}
