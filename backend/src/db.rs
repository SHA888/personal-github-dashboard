use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::env;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::models::{
    User, Repo, Organization, Issue, PullRequest, Commit,
    Branch, Release, Milestone, Workflow
};

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
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                login VARCHAR(255) NOT NULL,
                name VARCHAR(255),
                email VARCHAR(255),
                avatar_url TEXT,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create repositories table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS repositories (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                full_name VARCHAR(255) NOT NULL,
                description TEXT,
                private BOOLEAN NOT NULL,
                fork BOOLEAN NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                pushed_at TIMESTAMP WITH TIME ZONE,
                homepage TEXT,
                size INTEGER NOT NULL,
                stargazers_count INTEGER NOT NULL,
                watchers_count INTEGER NOT NULL,
                language VARCHAR(255),
                forks_count INTEGER NOT NULL,
                archived BOOLEAN NOT NULL,
                disabled BOOLEAN NOT NULL,
                open_issues_count INTEGER NOT NULL,
                default_branch VARCHAR(255) NOT NULL
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create organizations table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS organizations (
                id SERIAL PRIMARY KEY,
                login VARCHAR(255) NOT NULL,
                name VARCHAR(255),
                description TEXT,
                avatar_url TEXT,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create issues table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS issues (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state VARCHAR(50) NOT NULL,
                body TEXT,
                user_id INTEGER REFERENCES users(id),
                labels TEXT[] NOT NULL,
                assignees INTEGER[] NOT NULL,
                milestone_id INTEGER,
                locked BOOLEAN NOT NULL,
                comments_count INTEGER NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                closed_at TIMESTAMP WITH TIME ZONE
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create pull_requests table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS pull_requests (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title TEXT NOT NULL,
                state VARCHAR(50) NOT NULL,
                body TEXT,
                merged BOOLEAN NOT NULL,
                merged_at TIMESTAMP WITH TIME ZONE,
                merge_commit_sha VARCHAR(255),
                requested_reviewers JSONB NOT NULL,
                requested_teams JSONB NOT NULL,
                labels JSONB NOT NULL,
                comments_count INTEGER NOT NULL,
                review_comments_count INTEGER NOT NULL,
                commits_count INTEGER NOT NULL,
                additions INTEGER NOT NULL,
                deletions INTEGER NOT NULL
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create commits table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS commits (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                sha VARCHAR(255) NOT NULL,
                message TEXT NOT NULL,
                author JSONB NOT NULL,
                committer JSONB NOT NULL,
                stats JSONB NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create branches table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS branches (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                name VARCHAR(255) NOT NULL,
                commit_sha VARCHAR(255) NOT NULL,
                protected BOOLEAN NOT NULL,
                protection JSONB
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create releases table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS releases (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                tag_name VARCHAR(255) NOT NULL,
                name VARCHAR(255),
                body TEXT,
                draft BOOLEAN NOT NULL,
                prerelease BOOLEAN NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                published_at TIMESTAMP WITH TIME ZONE,
                assets JSONB NOT NULL
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create milestones table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS milestones (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                number INTEGER NOT NULL,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                state VARCHAR(50) NOT NULL,
                due_on TIMESTAMP WITH TIME ZONE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                closed_at TIMESTAMP WITH TIME ZONE
            )
            "#
        )
        .execute(pool)
        .await?;

        // Create workflows table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS workflows (
                id SERIAL PRIMARY KEY,
                repository_id INTEGER REFERENCES repositories(id),
                name VARCHAR(255) NOT NULL,
                path VARCHAR(255) NOT NULL,
                state VARCHAR(50) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // User-related methods
    pub async fn upsert_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (
                login, name, email, avatar_url, updated_at
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            ON CONFLICT (login) DO UPDATE SET
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                avatar_url = EXCLUDED.avatar_url,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(&user.login)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.avatar_url)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Repository-related methods
    pub async fn upsert_repository(&self, repo: &Repo) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO repositories (
                name, full_name, description, private, fork, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
            ON CONFLICT (name) DO UPDATE SET
                full_name = EXCLUDED.full_name,
                description = EXCLUDED.description,
                private = EXCLUDED.private,
                fork = EXCLUDED.fork,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(&repo.name)
        .bind(&repo.full_name)
        .bind(&repo.description)
        .bind(repo.private)
        .bind(repo.fork)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Organization-related methods
    pub async fn upsert_organization(&self, org: &Organization) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO organizations (
                login, name, description, avatar_url, updated_at
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            ON CONFLICT (login) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                avatar_url = EXCLUDED.avatar_url,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(&org.login)
        .bind(&org.name)
        .bind(&org.description)
        .bind(&org.avatar_url)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Issue-related methods
    pub async fn upsert_issue(&self, issue: &Issue) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO issues (
                repository_id, number, title, state, body, user_id, labels,
                assignees, milestone_id, locked, comments_count, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, CURRENT_TIMESTAMP)
            ON CONFLICT (repository_id, number) DO UPDATE SET
                title = EXCLUDED.title,
                state = EXCLUDED.state,
                body = EXCLUDED.body,
                user_id = EXCLUDED.user_id,
                labels = EXCLUDED.labels,
                assignees = EXCLUDED.assignees,
                milestone_id = EXCLUDED.milestone_id,
                locked = EXCLUDED.locked,
                comments_count = EXCLUDED.comments_count,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(issue.repository_id)
        .bind(issue.number)
        .bind(&issue.title)
        .bind(&issue.state)
        .bind(&issue.body)
        .bind(issue.user_id)
        .bind(&issue.labels)
        .bind(&issue.assignees)
        .bind(issue.milestone_id)
        .bind(issue.locked)
        .bind(issue.comments_count)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Pull Request-related methods
    pub async fn upsert_pull_request(&self, pr: &PullRequest) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO pull_requests (
                repository_id, number, title, state, body, merged,
                merged_at, merge_commit_sha, requested_reviewers, requested_teams,
                labels, comments_count, review_comments_count, commits_count,
                additions, deletions, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, CURRENT_TIMESTAMP)
            ON CONFLICT (repository_id, number) DO UPDATE SET
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
    pub async fn upsert_commit(&self, commit: &Commit) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO commits (
                repository_id, sha, message, author, committer, stats, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (repository_id, sha) DO UPDATE SET
                message = EXCLUDED.message,
                author = EXCLUDED.author,
                committer = EXCLUDED.committer,
                stats = EXCLUDED.stats
            "#
        )
        .bind(commit.repository_id)
        .bind(&commit.sha)
        .bind(&commit.message)
        .bind(&commit.author)
        .bind(&commit.committer)
        .bind(&commit.stats)
        .bind(commit.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Branch-related methods
    pub async fn upsert_branch(&self, branch: &Branch) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO branches (
                repository_id, name, commit_sha, protected, protection
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (repository_id, name) DO UPDATE SET
                commit_sha = EXCLUDED.commit_sha,
                protected = EXCLUDED.protected,
                protection = EXCLUDED.protection
            "#
        )
        .bind(branch.repository_id)
        .bind(&branch.name)
        .bind(&branch.commit_sha)
        .bind(branch.protected)
        .bind(&branch.protection)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Release-related methods
    pub async fn upsert_release(&self, release: &Release) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO releases (
                repository_id, tag_name, name, body, draft,
                prerelease, assets, created_at, published_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (repository_id, tag_name) DO UPDATE SET
                name = EXCLUDED.name,
                body = EXCLUDED.body,
                draft = EXCLUDED.draft,
                prerelease = EXCLUDED.prerelease,
                assets = EXCLUDED.assets,
                published_at = EXCLUDED.published_at
            "#
        )
        .bind(release.repository_id)
        .bind(&release.tag_name)
        .bind(&release.name)
        .bind(&release.body)
        .bind(release.draft)
        .bind(release.prerelease)
        .bind(&release.assets)
        .bind(release.created_at)
        .bind(&release.published_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Milestone-related methods
    pub async fn upsert_milestone(&self, milestone: &Milestone) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO milestones (
                repository_id, number, title, description, state,
                due_on, created_at, updated_at, closed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (repository_id, number) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                state = EXCLUDED.state,
                due_on = EXCLUDED.due_on,
                updated_at = EXCLUDED.updated_at,
                closed_at = EXCLUDED.closed_at
            "#
        )
        .bind(milestone.repository_id)
        .bind(milestone.number)
        .bind(&milestone.title)
        .bind(&milestone.description)
        .bind(&milestone.state)
        .bind(&milestone.due_on)
        .bind(milestone.created_at)
        .bind(milestone.updated_at)
        .bind(milestone.closed_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Workflow-related methods
    pub async fn upsert_workflow(&self, workflow: &Workflow) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO workflows (
                repository_id, name, path, state, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (repository_id, name) DO UPDATE SET
                path = EXCLUDED.path,
                state = EXCLUDED.state,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(workflow.repository_id)
        .bind(&workflow.name)
        .bind(&workflow.path)
        .bind(&workflow.state)
        .bind(workflow.created_at)
        .bind(workflow.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Query methods for data analysis
    pub async fn get_repository_stats(&self, repository_id: i32) -> Result<Value, sqlx::Error> {
        let stats = sqlx::query(
            r#"
            SELECT 
                COUNT(DISTINCT c.id) as total_commits,
                COUNT(DISTINCT i.id) as total_issues,
                COUNT(DISTINCT pr.id) as total_prs,
                COUNT(DISTINCT CASE WHEN i.state = 'open' THEN i.id END) as open_issues,
                COUNT(DISTINCT CASE WHEN pr.state = 'open' THEN pr.id END) as open_prs,
                COUNT(DISTINCT CASE WHEN pr.merged = true THEN pr.id END) as merged_prs,
                COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count > 0 THEN pr.id END) as reviewed_and_merged_prs
            FROM repositories r
            LEFT JOIN issues i ON i.repository_id = r.id
            LEFT JOIN pull_requests pr ON pr.repository_id = r.id
            LEFT JOIN commits c ON c.repository_id = r.id
            WHERE r.id = $1
            "#
        )
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(serde_json::json!({
            "total_commits": stats.get::<i64, _>("total_commits"),
            "total_issues": stats.get::<i64, _>("total_issues"),
            "total_prs": stats.get::<i64, _>("total_prs"),
            "open_issues": stats.get::<i64, _>("open_issues"),
            "open_prs": stats.get::<i64, _>("open_prs"),
            "merged_prs": stats.get::<i64, _>("merged_prs"),
            "reviewed_and_merged_prs": stats.get::<i64, _>("reviewed_and_merged_prs")
        }))
    }

    pub async fn get_repository_activity(&self, repository_id: i32, days: i32) -> Result<Value, sqlx::Error> {
        let activity = sqlx::query(
            r#"
            SELECT 
                DATE_TRUNC('day', created_at) as date,
                COUNT(*) as total_activity,
                SUM(CASE WHEN type = 'commit' THEN 1 ELSE 0 END) as commits
            FROM (
                SELECT created_at, 'commit' as type FROM commits WHERE repository_id = $1
            ) activities
            WHERE created_at >= NOW() - ($2 || ' days')::INTERVAL
            GROUP BY DATE_TRUNC('day', created_at)
            ORDER BY date DESC
            "#
        )
        .bind(repository_id)
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(serde_json::json!({
            "dates": activity.iter().map(|row| row.get::<DateTime<Utc>, _>("date")).collect::<Vec<_>>(),
            "total_activity": activity.iter().map(|row| row.get::<i64, _>("total_activity")).collect::<Vec<_>>(),
            "commits": activity.iter().map(|row| row.get::<i64, _>("commits")).collect::<Vec<_>>()
        }))
    }

    pub async fn get_user_activity(&self, user_id: i32) -> Result<Value, sqlx::Error> {
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

        Ok(serde_json::json!({
            "assigned_issues": activity.get::<i64, _>("assigned_issues"),
            "review_requests": activity.get::<i64, _>("review_requests"),
            "commits": activity.get::<i64, _>("commits"),
            "owned_repos": activity.get::<i64, _>("owned_repos")
        }))
    }
} 