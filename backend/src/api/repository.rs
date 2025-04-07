use crate::AppState;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct RepositoryResponse {
    pub id: i32,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddRepositoryRequest {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepositoryQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CommitRecord {
    pub id: i32,
    pub sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub created_at: DateTime<Utc>,
}

pub fn configure_repository_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api/repositories")
            .app_data(app_state.clone())
            .route("", web::post().to(add_repository))
            .route("/{owner}/{repo}", web::get().to(get_repository)),
    );
}

async fn add_repository(
    req: web::Json<AddRepositoryRequest>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    match app_state
        .github
        .sync_repository(&req.owner, &req.name)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": format!("Successfully added repository {}/{}", req.owner, req.name)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to add repository: {}", e)
        })),
    }
}

async fn get_repository(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    let owner_clone = owner.clone();
    let repo_clone = repo.clone();
    match sqlx::query_as::<_, RepositoryResponse>(
        r#"
        SELECT id, owner, name, description, language, stars, forks, open_issues, is_private, created_at, updated_at
        FROM repositories
        WHERE owner = $1 AND name = $2
        "#,
    )
    .bind(&owner)
    .bind(&repo)
    .fetch_optional(&app_state.pool)
    .await
    {
        Ok(Some(repository)) => HttpResponse::Ok().json(repository),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": format!("Repository {}/{} not found", owner_clone, repo_clone)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get repository: {}", e)
        })),
    }
}

#[allow(dead_code)]
async fn list_repositories(
    query: web::Query<RepositoryQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match sqlx::query_as::<_, RepositoryResponse>(
        r#"
        SELECT id, owner, name, description, language, stars, forks, open_issues, is_private, created_at, updated_at
        FROM repositories
        ORDER BY updated_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page)
    .bind((page - 1) * per_page)
    .fetch_all(&app_state.pool)
    .await
    {
        Ok(repositories) => HttpResponse::Ok().json(repositories),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to list repositories: {}", e)
        })),
    }
}

#[allow(dead_code)]
async fn get_repository_activity(
    path: web::Path<(String, String)>,
    query: web::Query<RepositoryQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match sqlx::query_as::<_, CommitRecord>(
        r#"
        SELECT
            c.id,
            c.sha,
            c.message,
            c.author_name,
            c.author_email,
            c.created_at
        FROM commits c
        JOIN repositories r ON c.repository_id = r.id
        WHERE r.owner = $1 AND r.name = $2
        ORDER BY c.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(owner)
    .bind(repo)
    .bind(per_page)
    .bind((page - 1) * per_page)
    .fetch_all(&app_state.pool)
    .await
    {
        Ok(commits) => HttpResponse::Ok().json(commits),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get repository activity: {}", e)
        })),
    }
}
