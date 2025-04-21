use actix_web::{web, HttpResponse};
use octocrab::Octocrab;
use personal_github_dashboard::error::AppError;
use personal_github_dashboard::utils::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: Option<u32>,
    pub forks_count: Option<u32>,
    pub open_issues_count: Option<u32>,
}

#[derive(Deserialize)]
pub struct RepoParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn get_repositories(query: web::Query<RepoParams>) -> Result<HttpResponse, AppError> {
    let config = Config::from_env();
    let octocrab = Octocrab::builder()
        .personal_token(config.github_personal_access_token.clone())
        .build()
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // clamp to [1,255] and cast into u8
    let page_num = query.page.unwrap_or(1).clamp(1, 255) as u8;
    let per_page_num = query.per_page.unwrap_or(10).clamp(1, 255) as u8;

    let page = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .page(page_num)
        .per_page(per_page_num)
        .send()
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let repos = page
        .items
        .into_iter()
        .map(|r| RepositoryInfo {
            name: r.name,
            description: r.description,
            html_url: r.html_url.map(|u| u.to_string()).unwrap_or_default(),
            stargazers_count: r.stargazers_count,
            forks_count: r.forks_count,
            open_issues_count: r.open_issues_count,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(repos))
}
