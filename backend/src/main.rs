use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use reqwest::Client;
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct Repo {
    name: String,
    owner: String,
}

async fn get_repos() -> impl Responder {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .unwrap()
        .json::<Vec<serde_json::Value>>()
        .await
        .unwrap();

    let repos: Vec<Repo> = response
        .into_iter()
        .map(|r| Repo {
            name: r["name"].as_str().unwrap().to_string(),
            owner: r["owner"]["login"].as_str().unwrap().to_string(),
        })
        .collect();

    HttpResponse::Ok().json(repos)
}

async fn get_commits(path: web::Path<(String, String)>) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = client
        .get(format!("https://api.github.com/repos/{}/{}/stats/commit_activity", owner, repo))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "github-dashboard")
        .send()
        .await
        .unwrap()
        .json::<Vec<serde_json::Value>>()
        .await
        .unwrap();

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/api/repos", web::get().to(get_repos))
            .route("/api/commits/{owner}/{repo}", web::get().to(get_commits))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}