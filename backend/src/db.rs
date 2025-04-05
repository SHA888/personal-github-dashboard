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

    // Add similar methods for other entities (issues, PRs, commits, etc.)
} 