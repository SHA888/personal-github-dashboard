use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use chrono::{DateTime, Utc};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        // Initialize database schema
        Self::init_schema(&pool).await?;

        Ok(Self { pool })
    }

    async fn init_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                login VARCHAR(255) NOT NULL,
                name VARCHAR(255),
                avatar_url TEXT,
                bio TEXT,
                public_repos INTEGER,
                followers INTEGER,
                following INTEGER,
                company VARCHAR(255),
                location VARCHAR(255),
                email VARCHAR(255),
                blog TEXT,
                twitter_username VARCHAR(255),
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS organizations (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                login VARCHAR(255) NOT NULL,
                description TEXT,
                avatar_url TEXT,
                members_count INTEGER,
                repos_count INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS repositories (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                owner_id INTEGER REFERENCES users(id),
                name VARCHAR(255) NOT NULL,
                description TEXT,
                language VARCHAR(255),
                stars INTEGER,
                forks INTEGER,
                open_issues INTEGER,
                license VARCHAR(255),
                topics JSONB,
                default_branch VARCHAR(255),
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS issues (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state VARCHAR(50) NOT NULL,
                body TEXT,
                labels JSONB,
                assignees JSONB,
                milestone JSONB,
                comments_count INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS pull_requests (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state VARCHAR(50) NOT NULL,
                body TEXT,
                merged BOOLEAN DEFAULT FALSE,
                merged_at TIMESTAMP WITH TIME ZONE,
                merge_commit_sha VARCHAR(255),
                requested_reviewers JSONB,
                requested_teams JSONB,
                labels JSONB,
                comments_count INTEGER,
                review_comments_count INTEGER,
                commits_count INTEGER,
                additions INTEGER,
                deletions INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS commits (
                id SERIAL PRIMARY KEY,
                sha VARCHAR(255) UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                message TEXT NOT NULL,
                author JSONB,
                committer JSONB,
                stats JSONB,
                files JSONB,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS branches (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                name VARCHAR(255) NOT NULL,
                commit JSONB,
                protected BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(repository_id, name)
            );

            CREATE TABLE IF NOT EXISTS releases (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                tag_name VARCHAR(255) NOT NULL,
                name VARCHAR(255),
                body TEXT,
                draft BOOLEAN DEFAULT FALSE,
                prerelease BOOLEAN DEFAULT FALSE,
                assets JSONB,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                published_at TIMESTAMP WITH TIME ZONE
            );

            CREATE TABLE IF NOT EXISTS milestones (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                state VARCHAR(50) NOT NULL,
                due_on TIMESTAMP WITH TIME ZONE,
                open_issues INTEGER,
                closed_issues INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS workflows (
                id SERIAL PRIMARY KEY,
                github_id INTEGER UNIQUE NOT NULL,
                repository_id INTEGER REFERENCES repositories(id),
                name VARCHAR(255) NOT NULL,
                state VARCHAR(50) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // User-related methods
    pub async fn upsert_user(&self, user: &crate::User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (
                github_id, login, name, avatar_url, bio, public_repos, followers, following,
                company, location, email, blog, twitter_username, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                login = EXCLUDED.login,
                name = EXCLUDED.name,
                avatar_url = EXCLUDED.avatar_url,
                bio = EXCLUDED.bio,
                public_repos = EXCLUDED.public_repos,
                followers = EXCLUDED.followers,
                following = EXCLUDED.following,
                company = EXCLUDED.company,
                location = EXCLUDED.location,
                email = EXCLUDED.email,
                blog = EXCLUDED.blog,
                twitter_username = EXCLUDED.twitter_username,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(user.github_id)
        .bind(&user.login)
        .bind(&user.name)
        .bind(&user.avatar_url)
        .bind(&user.bio)
        .bind(user.public_repos)
        .bind(user.followers)
        .bind(user.following)
        .bind(&user.company)
        .bind(&user.location)
        .bind(&user.email)
        .bind(&user.blog)
        .bind(&user.twitter_username)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Repository-related methods
    pub async fn upsert_repository(&self, repo: &crate::Repo) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO repositories (
                github_id, owner_id, name, description, language, stars, forks,
                open_issues, license, topics, default_branch, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                language = EXCLUDED.language,
                stars = EXCLUDED.stars,
                forks = EXCLUDED.forks,
                open_issues = EXCLUDED.open_issues,
                license = EXCLUDED.license,
                topics = EXCLUDED.topics,
                default_branch = EXCLUDED.default_branch,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(repo.github_id)
        .bind(repo.owner_id)
        .bind(&repo.name)
        .bind(&repo.description)
        .bind(&repo.language)
        .bind(repo.stars)
        .bind(repo.forks)
        .bind(repo.open_issues)
        .bind(&repo.license)
        .bind(&repo.topics)
        .bind(&repo.default_branch)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Organization-related methods
    pub async fn upsert_organization(&self, org: &crate::Organization) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO organizations (
                github_id, login, description, avatar_url, members_count, repos_count, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                login = EXCLUDED.login,
                description = EXCLUDED.description,
                avatar_url = EXCLUDED.avatar_url,
                members_count = EXCLUDED.members_count,
                repos_count = EXCLUDED.repos_count,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(org.github_id)
        .bind(&org.login)
        .bind(&org.description)
        .bind(&org.avatar_url)
        .bind(org.members_count)
        .bind(org.repos_count)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Issue-related methods
    pub async fn upsert_issue(&self, issue: &crate::Issue) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO issues (
                github_id, repository_id, number, title, state, body, labels,
                assignees, milestone, comments_count, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                title = EXCLUDED.title,
                state = EXCLUDED.state,
                body = EXCLUDED.body,
                labels = EXCLUDED.labels,
                assignees = EXCLUDED.assignees,
                milestone = EXCLUDED.milestone,
                comments_count = EXCLUDED.comments_count,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(issue.github_id)
        .bind(issue.repository_id)
        .bind(issue.number)
        .bind(&issue.title)
        .bind(&issue.state)
        .bind(&issue.body)
        .bind(&issue.labels)
        .bind(&issue.assignees)
        .bind(&issue.milestone)
        .bind(issue.comments_count)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Pull Request-related methods
    pub async fn upsert_pull_request(&self, pr: &crate::PullRequest) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO pull_requests (
                github_id, repository_id, number, title, state, body, merged,
                merged_at, merge_commit_sha, requested_reviewers, requested_teams,
                labels, comments_count, review_comments_count, commits_count,
                additions, deletions, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                title = EXCLUDED.title,
                state = EXCLUDED.state,
                body = EXCLUDED.body,
                merged = EXCLUDED.merged,
                merged_at = EXCLUDED.merged_at,
                merge_commit_sha = EXCLUDED.merge_commit_sha,
                requested_reviewers = EXCLUDED.requested_reviewers,
                requested_teams = EXCLUDED.requested_teams,
                labels = EXCLUDED.labels,
                comments_count = EXCLUDED.comments_count,
                review_comments_count = EXCLUDED.review_comments_count,
                commits_count = EXCLUDED.commits_count,
                additions = EXCLUDED.additions,
                deletions = EXCLUDED.deletions,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(pr.github_id)
        .bind(pr.repository_id)
        .bind(pr.number)
        .bind(&pr.title)
        .bind(&pr.state)
        .bind(&pr.body)
        .bind(pr.merged)
        .bind(&pr.merged_at)
        .bind(&pr.merge_commit_sha)
        .bind(&pr.requested_reviewers)
        .bind(&pr.requested_teams)
        .bind(&pr.labels)
        .bind(pr.comments_count)
        .bind(pr.review_comments_count)
        .bind(pr.commits_count)
        .bind(pr.additions)
        .bind(pr.deletions)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Commit-related methods
    pub async fn upsert_commit(&self, commit: &crate::Commit) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO commits (
                sha, repository_id, message, author, committer, stats, files
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (sha) DO UPDATE SET
                message = EXCLUDED.message,
                author = EXCLUDED.author,
                committer = EXCLUDED.committer,
                stats = EXCLUDED.stats,
                files = EXCLUDED.files
            "#
        )
        .bind(&commit.sha)
        .bind(commit.repository_id)
        .bind(&commit.message)
        .bind(&commit.author)
        .bind(&commit.committer)
        .bind(&commit.stats)
        .bind(&commit.files)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Branch-related methods
    pub async fn upsert_branch(&self, branch: &crate::Branch) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO branches (
                repository_id, name, commit, protected, updated_at
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            ON CONFLICT (repository_id, name) DO UPDATE SET
                commit = EXCLUDED.commit,
                protected = EXCLUDED.protected,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(branch.repository_id)
        .bind(&branch.name)
        .bind(&branch.commit)
        .bind(branch.protected)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Release-related methods
    pub async fn upsert_release(&self, release: &crate::Release) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO releases (
                github_id, repository_id, tag_name, name, body, draft,
                prerelease, assets, published_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                tag_name = EXCLUDED.tag_name,
                name = EXCLUDED.name,
                body = EXCLUDED.body,
                draft = EXCLUDED.draft,
                prerelease = EXCLUDED.prerelease,
                assets = EXCLUDED.assets,
                published_at = EXCLUDED.published_at,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(release.github_id)
        .bind(release.repository_id)
        .bind(&release.tag_name)
        .bind(&release.name)
        .bind(&release.body)
        .bind(release.draft)
        .bind(release.prerelease)
        .bind(&release.assets)
        .bind(&release.published_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Milestone-related methods
    pub async fn upsert_milestone(&self, milestone: &crate::Milestone) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO milestones (
                github_id, repository_id, number, title, description, state,
                due_on, open_issues, closed_issues, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                state = EXCLUDED.state,
                due_on = EXCLUDED.due_on,
                open_issues = EXCLUDED.open_issues,
                closed_issues = EXCLUDED.closed_issues,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(milestone.github_id)
        .bind(milestone.repository_id)
        .bind(milestone.number)
        .bind(&milestone.title)
        .bind(&milestone.description)
        .bind(&milestone.state)
        .bind(&milestone.due_on)
        .bind(milestone.open_issues)
        .bind(milestone.closed_issues)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Workflow-related methods
    pub async fn upsert_workflow(&self, workflow: &crate::Workflow) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO workflows (
                github_id, repository_id, name, state, updated_at
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            ON CONFLICT (github_id) DO UPDATE SET
                name = EXCLUDED.name,
                state = EXCLUDED.state,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(workflow.github_id)
        .bind(workflow.repository_id)
        .bind(&workflow.name)
        .bind(&workflow.state)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Query methods for data analysis
    pub async fn get_repository_stats(&self, repository_id: i32) -> Result<serde_json::Value, sqlx::Error> {
        let stats = sqlx::query(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM issues WHERE repository_id = $1) as total_issues,
                (SELECT COUNT(*) FROM issues WHERE repository_id = $1 AND state = 'open') as open_issues,
                (SELECT COUNT(*) FROM pull_requests WHERE repository_id = $1) as total_prs,
                (SELECT COUNT(*) FROM pull_requests WHERE repository_id = $1 AND state = 'open') as open_prs,
                (SELECT COUNT(*) FROM commits WHERE repository_id = $1) as total_commits,
                (SELECT COUNT(*) FROM branches WHERE repository_id = $1) as total_branches,
                (SELECT COUNT(*) FROM releases WHERE repository_id = $1) as total_releases,
                (SELECT COUNT(*) FROM workflows WHERE repository_id = $1) as total_workflows
            "#
        )
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(serde_json::json!(stats))
    }

    pub async fn get_user_activity(&self, user_id: i32) -> Result<serde_json::Value, sqlx::Error> {
        let activity = sqlx::query(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM issues WHERE assignees @> $1) as assigned_issues,
                (SELECT COUNT(*) FROM pull_requests WHERE requested_reviewers @> $1) as review_requests,
                (SELECT COUNT(*) FROM commits WHERE author->>'id' = $2::text) as commits,
                (SELECT COUNT(*) FROM repositories WHERE owner_id = $3) as owned_repos
            "#
        )
        .bind(serde_json::json!([{"id": user_id}]))
        .bind(user_id.to_string())
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(serde_json::json!(activity))
    }
} 