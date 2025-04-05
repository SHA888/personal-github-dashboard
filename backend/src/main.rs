use actix_web::{web, App, HttpServer, Responder, HttpResponse, error, Error};
use actix_web::middleware::Logger;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use serde::Serialize;
use reqwest::Client;
use std::env;
use std::time::Duration;
use dotenv::dotenv;
use serde::Deserialize;

// Error handling
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// Pagination parameters
#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    per_page: Option<u32>,
}

// User-related structs
#[derive(Serialize)]
struct User {
    login: String,
    name: Option<String>,
    avatar_url: String,
    bio: Option<String>,
    public_repos: i32,
    followers: i32,
    following: i32,
    company: Option<String>,
    location: Option<String>,
    email: Option<String>,
    blog: Option<String>,
    twitter_username: Option<String>,
}

#[derive(Serialize)]
struct Organization {
    login: String,
    description: Option<String>,
    avatar_url: String,
    members_count: i32,
    repos_count: i32,
}

// Repository-related structs
#[derive(Serialize)]
struct Repo {
    name: String,
    owner: String,
    description: Option<String>,
    language: Option<String>,
    stars: i32,
    forks: i32,
    open_issues: i32,
    license: Option<String>,
    topics: Vec<String>,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    default_branch: String,
}

#[derive(Serialize)]
struct RepoStats {
    languages: serde_json::Value,
    contributors: Vec<serde_json::Value>,
    traffic: serde_json::Value,
    participation: serde_json::Value,
    code_frequency: serde_json::Value,
    commit_activity: serde_json::Value,
}

#[derive(Serialize)]
struct Issue {
    number: i32,
    title: String,
    state: String,
    created_at: String,
    updated_at: String,
    labels: Vec<serde_json::Value>,
    assignees: Vec<serde_json::Value>,
    milestone: Option<serde_json::Value>,
    comments: i32,
    body: Option<String>,
}

#[derive(Serialize)]
struct PullRequest {
    number: i32,
    title: String,
    state: String,
    created_at: String,
    updated_at: String,
    merged: bool,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    requested_reviewers: Vec<serde_json::Value>,
    requested_teams: Vec<serde_json::Value>,
    labels: Vec<serde_json::Value>,
    comments: i32,
    review_comments: i32,
    commits: i32,
    additions: i32,
    deletions: i32,
}

#[derive(Serialize)]
struct Commit {
    sha: String,
    message: String,
    author: serde_json::Value,
    committer: serde_json::Value,
    date: String,
    stats: serde_json::Value,
    files: Vec<serde_json::Value>,
}

// Helper function for pagination
fn get_pagination_params(params: web::Query<PaginationParams>) -> (u32, u32) {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(30).min(100);
    (page, per_page)
}

// Helper function for error handling
fn handle_error(err: reqwest::Error) -> Error {
    error::InternalError::new(
        ErrorResponse {
            error: "GitHub API Error".to_string(),
            message: err.to_string(),
        },
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    )
    .into()
}

// User endpoints
async fn get_user() -> Result<HttpResponse, Error> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    let user = User {
        login: response["login"].as_str().unwrap().to_string(),
        name: response["name"].as_str().map(|s| s.to_string()),
        avatar_url: response["avatar_url"].as_str().unwrap().to_string(),
        bio: response["bio"].as_str().map(|s| s.to_string()),
        public_repos: response["public_repos"].as_i64().unwrap() as i32,
        followers: response["followers"].as_i64().unwrap() as i32,
        following: response["following"].as_i64().unwrap() as i32,
        company: response["company"].as_str().map(|s| s.to_string()),
        location: response["location"].as_str().map(|s| s.to_string()),
        email: response["email"].as_str().map(|s| s.to_string()),
        blog: response["blog"].as_str().map(|s| s.to_string()),
        twitter_username: response["twitter_username"].as_str().map(|s| s.to_string()),
    };

    Ok(HttpResponse::Ok().json(user))
}

async fn get_organizations(params: web::Query<PaginationParams>) -> Result<HttpResponse, Error> {
    let (page, per_page) = get_pagination_params(params);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user/orgs")
        .query(&[("page", page), ("per_page", per_page)])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    let orgs: Vec<Organization> = response
        .into_iter()
        .map(|o| Organization {
            login: o["login"].as_str().unwrap().to_string(),
            description: o["description"].as_str().map(|s| s.to_string()),
            avatar_url: o["avatar_url"].as_str().unwrap().to_string(),
            members_count: o["members_count"].as_i64().unwrap_or(0) as i32,
            repos_count: o["repos_count"].as_i64().unwrap_or(0) as i32,
        })
        .collect();

    Ok(HttpResponse::Ok().json(orgs))
}

// Repository endpoints
async fn get_repos(params: web::Query<PaginationParams>) -> Result<HttpResponse, Error> {
    let (page, per_page) = get_pagination_params(params);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user/repos")
        .query(&[("page", page), ("per_page", per_page)])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    let repos: Vec<Repo> = response
        .into_iter()
        .map(|r| Repo {
            name: r["name"].as_str().unwrap().to_string(),
            owner: r["owner"]["login"].as_str().unwrap().to_string(),
            description: r["description"].as_str().map(|s| s.to_string()),
            language: r["language"].as_str().map(|s| s.to_string()),
            stars: r["stargazers_count"].as_i64().unwrap() as i32,
            forks: r["forks_count"].as_i64().unwrap() as i32,
            open_issues: r["open_issues_count"].as_i64().unwrap() as i32,
            license: r["license"].as_object().and_then(|l| l["name"].as_str()).map(|s| s.to_string()),
            topics: r["topics"].as_array().unwrap_or(&vec![]).iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect(),
            created_at: r["created_at"].as_str().unwrap().to_string(),
            updated_at: r["updated_at"].as_str().unwrap().to_string(),
            pushed_at: r["pushed_at"].as_str().unwrap().to_string(),
            default_branch: r["default_branch"].as_str().unwrap().to_string(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(repos))
}

async fn get_repo_stats(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (owner, repo) = path.into_inner();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();

    // Get languages
    let languages = client
        .get(format!("https://api.github.com/repos/{}/{}/languages", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    // Get contributors
    let contributors = client
        .get(format!("https://api.github.com/repos/{}/{}/contributors", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    // Get traffic data
    let traffic = client
        .get(format!("https://api.github.com/repos/{}/{}/traffic/views", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    // Get participation data
    let participation = client
        .get(format!("https://api.github.com/repos/{}/{}/stats/participation", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    // Get code frequency
    let code_frequency = client
        .get(format!("https://api.github.com/repos/{}/{}/stats/code_frequency", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    // Get commit activity
    let commit_activity = client
        .get(format!("https://api.github.com/repos/{}/{}/stats/commit_activity", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<serde_json::Value>()
        .await
        .map_err(handle_error)?;

    let stats = RepoStats {
        languages,
        contributors,
        traffic,
        participation,
        code_frequency,
        commit_activity,
    };

    Ok(HttpResponse::Ok().json(stats))
}

async fn get_issues(path: web::Path<(String, String)>, params: web::Query<PaginationParams>) -> Result<HttpResponse, Error> {
    let (owner, repo) = path.into_inner();
    let (page, per_page) = get_pagination_params(params);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get(format!("https://api.github.com/repos/{}/{}/issues", owner, repo))
        .query(&[("page", page), ("per_page", per_page)])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    let issues: Vec<Issue> = response
        .into_iter()
        .map(|i| Issue {
            number: i["number"].as_i64().unwrap() as i32,
            title: i["title"].as_str().unwrap().to_string(),
            state: i["state"].as_str().unwrap().to_string(),
            created_at: i["created_at"].as_str().unwrap().to_string(),
            updated_at: i["updated_at"].as_str().unwrap().to_string(),
            labels: i["labels"].as_array().unwrap().to_vec(),
            assignees: i["assignees"].as_array().unwrap().to_vec(),
            milestone: i["milestone"].as_object().map(|m| serde_json::Value::Object(m.clone())),
            comments: i["comments"].as_i64().unwrap() as i32,
            body: i["body"].as_str().map(|s| s.to_string()),
        })
        .collect();

    Ok(HttpResponse::Ok().json(issues))
}

async fn get_pull_requests(path: web::Path<(String, String)>, params: web::Query<PaginationParams>) -> Result<HttpResponse, Error> {
    let (owner, repo) = path.into_inner();
    let (page, per_page) = get_pagination_params(params);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get(format!("https://api.github.com/repos/{}/{}/pulls", owner, repo))
        .query(&[("page", page), ("per_page", per_page)])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    let prs: Vec<PullRequest> = response
        .into_iter()
        .map(|pr| PullRequest {
            number: pr["number"].as_i64().unwrap() as i32,
            title: pr["title"].as_str().unwrap().to_string(),
            state: pr["state"].as_str().unwrap().to_string(),
            created_at: pr["created_at"].as_str().unwrap().to_string(),
            updated_at: pr["updated_at"].as_str().unwrap().to_string(),
            merged: pr["merged"].as_bool().unwrap(),
            merged_at: pr["merged_at"].as_str().map(|s| s.to_string()),
            merge_commit_sha: pr["merge_commit_sha"].as_str().map(|s| s.to_string()),
            requested_reviewers: pr["requested_reviewers"].as_array().unwrap().to_vec(),
            requested_teams: pr["requested_teams"].as_array().unwrap().to_vec(),
            labels: pr["labels"].as_array().unwrap().to_vec(),
            comments: pr["comments"].as_i64().unwrap() as i32,
            review_comments: pr["review_comments"].as_i64().unwrap() as i32,
            commits: pr["commits"].as_i64().unwrap() as i32,
            additions: pr["additions"].as_i64().unwrap() as i32,
            deletions: pr["deletions"].as_i64().unwrap() as i32,
        })
        .collect();

    Ok(HttpResponse::Ok().json(prs))
}

async fn get_commits(path: web::Path<(String, String)>, params: web::Query<PaginationParams>) -> Result<HttpResponse, Error> {
    let (owner, repo) = path.into_inner();
    let (page, per_page) = get_pagination_params(params);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get(format!("https://api.github.com/repos/{}/{}/commits", owner, repo))
        .query(&[("page", page), ("per_page", per_page)])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .map_err(handle_error)?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(handle_error)?;

    let commits: Vec<Commit> = response
        .into_iter()
        .map(|c| Commit {
            sha: c["sha"].as_str().unwrap().to_string(),
            message: c["commit"]["message"].as_str().unwrap().to_string(),
            author: c["author"].clone(),
            committer: c["committer"].clone(),
            date: c["commit"]["author"]["date"].as_str().unwrap().to_string(),
            stats: c["stats"].clone(),
            files: c["files"].as_array().unwrap().to_vec(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(commits))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize rate limiter
    let store = MemoryStore::new();
    let store = MemoryStoreActor::from(store).start();
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                RateLimiter::new(store.clone())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(60)
            )
            // User endpoints
            .route("/api/user", web::get().to(get_user))
            .route("/api/organizations", web::get().to(get_organizations))
            
            // Repository endpoints
            .route("/api/repos", web::get().to(get_repos))
            .route("/api/repos/{owner}/{repo}/stats", web::get().to(get_repo_stats))
            .route("/api/repos/{owner}/{repo}/issues", web::get().to(get_issues))
            .route("/api/repos/{owner}/{repo}/pulls", web::get().to(get_pull_requests))
            .route("/api/repos/{owner}/{repo}/commits", web::get().to(get_commits))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}