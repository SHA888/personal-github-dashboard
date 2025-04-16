use std::error::Error;
use std::sync::Arc;

use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use octocrab::models::orgs::Organization;
use octocrab::models::Repository;

#[derive(Clone)]
pub struct GitHubService {
    octocrab: octocrab::Octocrab,
    rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl GitHubService {
    pub async fn new(github_token: String, _redis_url: String) -> Result<Self, Box<dyn Error>> {
        let octocrab = octocrab::OctocrabBuilder::new()
            .personal_token(github_token)
            .build()?;

        // Configure rate limiter for GitHub's default rate limit (5000 requests per hour)
        let quota = Quota::per_hour(nonzero!(5000u32));
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        Ok(Self {
            octocrab,
            rate_limiter,
        })
    }

    async fn wait_for_rate_limit(&self) {
        self.rate_limiter.until_ready().await;
    }

    pub async fn get_organization(&self, org_name: &str) -> Result<Organization, octocrab::Error> {
        self.wait_for_rate_limit().await;
        self.octocrab.orgs(org_name).get().await
    }

    #[allow(dead_code)]
    pub async fn get_organization_repos(
        &self,
        org_name: &str,
    ) -> Result<Vec<Repository>, octocrab::Error> {
        self.wait_for_rate_limit().await;
        let mut repos = Vec::new();
        let mut page = self
            .octocrab
            .orgs(org_name)
            .list_repos()
            .per_page(100)
            .send()
            .await?;

        loop {
            repos.append(&mut page.items);

            if let Some(next) = page.next {
                self.wait_for_rate_limit().await;
                page = self
                    .octocrab
                    .get_page(&Some(next))
                    .await?
                    .unwrap_or_else(|| {
                        panic!("Failed to get next page of repositories");
                    });
            } else {
                break;
            }
        }

        Ok(repos)
    }

    pub async fn get_authenticated_user(
        &self,
    ) -> Result<octocrab::models::Author, octocrab::Error> {
        self.wait_for_rate_limit().await;
        self.octocrab.current().user().await
    }

    pub async fn list_my_organizations(&self) -> Result<Vec<Organization>, octocrab::Error> {
        self.wait_for_rate_limit().await;
        let mut orgs = Vec::new();
        let mut page = self
            .octocrab
            .current()
            .list_org_memberships_for_authenticated_user()
            .per_page(100)
            .send()
            .await?;

        loop {
            for membership in &page.items {
                self.wait_for_rate_limit().await;
                if let Ok(org_details) = self.get_organization(&membership.organization.login).await
                {
                    orgs.push(org_details);
                }
            }

            if let Some(next) = page.next {
                self.wait_for_rate_limit().await;
                page = self
                    .octocrab
                    .get_page(&Some(next))
                    .await?
                    .unwrap_or_else(|| {
                        panic!("Failed to get next page of organization memberships");
                    });
            } else {
                break;
            }
        }

        Ok(orgs)
    }

    pub async fn list_my_repositories(&self) -> Result<Vec<Repository>, octocrab::Error> {
        self.wait_for_rate_limit().await;
        let mut repos = Vec::new();
        let mut page = self
            .octocrab
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .send()
            .await?;

        loop {
            repos.append(&mut page.items);

            if let Some(next) = page.next {
                self.wait_for_rate_limit().await;
                page = self
                    .octocrab
                    .get_page(&Some(next))
                    .await?
                    .unwrap_or_else(|| {
                        panic!("Failed to get next page of repositories");
                    });
            } else {
                break;
            }
        }

        Ok(repos)
    }

    #[allow(dead_code)]
    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<octocrab::models::Author, octocrab::Error> {
        self.wait_for_rate_limit().await;
        self.octocrab
            .get(format!("/users/{}", username), None::<&()>)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_repository_details(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Repository, octocrab::Error> {
        self.wait_for_rate_limit().await;
        self.octocrab.repos(owner, repo).get().await
    }
}
