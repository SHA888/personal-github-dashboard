use log;
use octocrab::Octocrab;
use sqlx::PgPool;

pub struct GitHubService {
    client: Octocrab,
    pool: PgPool,
}

impl GitHubService {
    pub fn new(token: String, pool: PgPool) -> Self {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to create GitHub client");

        Self { client, pool }
    }

    pub async fn get_authenticated_user(&self) -> Result<String, Box<dyn std::error::Error>> {
        let user = self.client.current().user().await?;
        Ok(user.login)
    }

    pub async fn sync_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get repository info
        let repository = self.client.repos(owner, repo).get().await?;

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
            repository.language.map(|l| l.to_string()),
            repository.stargazers_count.unwrap_or(0) as i32,
            repository.forks_count.unwrap_or(0) as i32,
            repository.open_issues_count.unwrap_or(0) as i32,
            repository.private.unwrap_or(false)
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        // Get commits
        let commits = self.client.repos(owner, repo).list_commits().send().await?;

        // Store commits in database
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

        Ok(())
    }

    pub async fn fetch_user_repositories(
        &self,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100; // Maximum allowed by GitHub API

        // Get the authenticated user
        let username = self.get_authenticated_user().await?;
        log::info!("Fetching repositories for user: {}", username);

        loop {
            let repos = self
                .client
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
}
