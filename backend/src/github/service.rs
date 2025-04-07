use crate::redis::RedisClient;
use log;
use octocrab::models::orgs::Organization;
use octocrab::Octocrab;
use sqlx::PgPool;
use std::sync::Arc;

pub struct GitHubService {
    octocrab: Arc<Octocrab>,
    pub pool: PgPool,
    #[allow(dead_code)]
    redis: Arc<RedisClient>,
}

impl GitHubService {
    pub fn new(token: String, pool: PgPool) -> Self {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to create GitHub client");

        Self {
            octocrab: Arc::new(client),
            pool,
            redis: Arc::new(RedisClient::new()),
        }
    }

    pub async fn get_authenticated_user(&self) -> Result<String, Box<dyn std::error::Error>> {
        let user = self.octocrab.current().user().await?;
        Ok(user.login)
    }

    pub async fn sync_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get repository info
        let repository = self.octocrab.repos(owner, repo).get().await?;

        // Insert or update repository
        let repository_id = sqlx::query!(
            r#"
            INSERT INTO repositories (id, owner, name, description, language, stars, forks, open_issues, is_private)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (owner, name) DO UPDATE
            SET updated_at = CURRENT_TIMESTAMP,
                description = EXCLUDED.description,
                language = EXCLUDED.language,
                stars = EXCLUDED.stars,
                forks = EXCLUDED.forks,
                open_issues = EXCLUDED.open_issues,
                is_private = EXCLUDED.is_private
            RETURNING id
            "#,
            repository.id.0 as i32,
            owner,
            repo,
            repository.description,
            repository.language.map(|v| v.to_string()),
            repository.stargazers_count.unwrap_or(0) as i32,
            repository.forks_count.unwrap_or(0) as i32,
            repository.open_issues_count.unwrap_or(0) as i32,
            repository.private.unwrap_or(false),
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        // Try to get commits, but don't fail if the repository is empty
        match self.octocrab.repos(owner, repo).list_commits().send().await {
            Ok(commits) => {
                for commit in commits.items {
                    sqlx::query!(
                        r#"
                        INSERT INTO commits (sha, repository_id, author_name, author_email, message, created_at)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        ON CONFLICT (repository_id, sha) DO UPDATE
                        SET author_name = EXCLUDED.author_name,
                            author_email = EXCLUDED.author_email,
                            message = EXCLUDED.message,
                            created_at = EXCLUDED.created_at
                        "#,
                        commit.sha,
                        repository_id,
                        commit.commit.author.as_ref().and_then(|a| Some(a.user.name.clone())),
                        commit.commit.author.as_ref().and_then(|a| Some(a.user.email.clone())),
                        commit.commit.message,
                        commit.commit.author.as_ref().and_then(|a| a.date)
                    )
                    .execute(&self.pool)
                    .await?;
                }
            }
            Err(e) => {
                if e.to_string().contains("Git Repository is empty") {
                    log::info!(
                        "Repository {}/{} is empty, skipping commit sync",
                        owner,
                        repo
                    );
                } else {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    pub async fn fetch_user_repositories(
        &self,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut all_repos = Vec::new();
        let mut page = 1u8; // octocrab expects u8 for pagination
        let per_page = 100;

        // Get the authenticated user
        let username = self.get_authenticated_user().await?;
        log::info!("Fetching repositories for user: {}", username);

        loop {
            let repos = self
                .octocrab
                .current()
                .list_repos_for_authenticated_user()
                .per_page(per_page)
                .page(page)
                .send()
                .await?;

            if repos.items.is_empty() {
                break;
            }

            for repo in repos.items {
                if let Some(owner) = repo.owner.map(|o| o.login) {
                    if owner == username {
                        let repo_name = repo.name.clone();
                        log::info!("Found repository: {}/{}", owner, repo_name);
                        all_repos.push((owner, repo_name));
                    }
                }
            }

            page += 1;
        }

        log::info!(
            "Found {} repositories for user {}",
            all_repos.len(),
            username
        );
        Ok(all_repos)
    }

    pub async fn sync_user_repositories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let repos = self.fetch_user_repositories().await?;

        for (owner, repo) in repos {
            if let Err(e) = self.sync_repository(&owner, &repo).await {
                log::error!("Failed to sync repository {}/{}: {}", owner, repo, e);
            }
        }

        Ok(())
    }

    pub async fn sync_organization(
        &self,
        org_login: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        // Get full organization details from GitHub
        let org = self
            .octocrab
            .get::<Organization, _, _>(
                &format!("https://api.github.com/orgs/{}", org_login),
                None::<&()>,
            )
            .await?;

        // Insert or update organization in database
        let org_id = sqlx::query!(
            r#"
            INSERT INTO organizations (github_id, name, description, avatar_url)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (github_id) DO UPDATE
            SET name = EXCLUDED.name,
                description = EXCLUDED.description,
                avatar_url = EXCLUDED.avatar_url
            RETURNING id
            "#,
            org.id.0 as i64,
            org.login,
            org.description,
            org.avatar_url.to_string(),
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        log::info!(
            "Synced organization {} (ID: {}, GitHub ID: {})",
            org.login,
            org_id,
            org.id.0
        );

        Ok(org_id)
    }

    pub async fn fetch_user_organizations(
        &self,
    ) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
        let mut all_orgs: Vec<Organization> = Vec::new();
        let mut page = 1u8;
        let per_page = 100;

        // Get the authenticated user
        let username = self.get_authenticated_user().await?;
        log::info!("Fetching organizations for user: {}", username);

        // Get organizations where the user is a member
        loop {
            let url = format!(
                "https://api.github.com/user/orgs?per_page={}&page={}",
                per_page, page
            );
            log::info!("Fetching organizations from URL: {}", url);

            // Fetch the list of organizations the user is a member of
            let org_memberships = self
                .octocrab
                .get::<Vec<Organization>, _, _>(&url, None::<&()>)
                .await?;

            log::info!(
                "Received {} organization memberships on page {}",
                org_memberships.len(),
                page
            );

            if org_memberships.is_empty() {
                log::info!("No more organization memberships found, breaking loop");
                break;
            }

            for org_membership in org_memberships {
                log::info!(
                    "Fetching full details for organization: {}",
                    org_membership.login
                );
                // Get full organization details and sync to database
                let full_org = self
                    .octocrab
                    .get::<Organization, _, _>(
                        &format!("https://api.github.com/orgs/{}", org_membership.login),
                        None::<&()>,
                    )
                    .await?;

                // Sync organization to database
                self.sync_organization(&full_org.login).await?;

                log::info!(
                    "Found organization: {} ({})",
                    full_org.login,
                    full_org.name.clone().unwrap_or_default()
                );
                all_orgs.push(full_org);
            }

            page += 1;
        }

        log::info!("Found total of {} organizations", all_orgs.len());
        Ok(all_orgs)
    }

    pub async fn fetch_organization_repositories(
        &self,
        org: &str,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut all_repos = Vec::new();
        let mut page = 1u8; // octocrab expects u8 for pagination
        let per_page = 100;

        loop {
            let repos = self
                .octocrab
                .orgs(org)
                .list_repos()
                .per_page(per_page)
                .page(page)
                .send()
                .await?;

            if repos.items.is_empty() {
                break;
            }

            for repo in repos.items {
                all_repos.push((org.to_string(), repo.name));
            }

            page += 1;
        }

        log::info!(
            "Found {} repositories for organization {}",
            all_repos.len(),
            org
        );
        Ok(all_repos)
    }

    pub async fn list_all_repositories_and_organizations(
        &self,
    ) -> Result<(Vec<(String, String)>, Vec<String>), Box<dyn std::error::Error>> {
        // Get user repositories
        let mut all_repos = self.fetch_user_repositories().await?;

        // Get organizations (full details)
        let orgs: Vec<Organization> = self.fetch_user_organizations().await?;

        // Get repositories for each organization
        for org in &orgs {
            // Iterate over Vec<Organization>
            // Use the organization's login (String) to fetch its repositories
            let org_repos = self.fetch_organization_repositories(&org.login).await?;
            all_repos.extend(org_repos);
        }

        // Collect only the organization logins (Vec<String>) for the return tuple
        let org_logins: Vec<String> = orgs.iter().map(|org| org.login.clone()).collect();

        Ok((all_repos, org_logins))
    }
}
