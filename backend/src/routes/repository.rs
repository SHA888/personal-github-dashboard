use actix_web::{web, HttpResponse, error::ResponseError};
use crate::AppState;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

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

#[derive(Debug, Serialize, FromRow)]
pub struct ActivityResponse {
    pub id: i32,
    pub repository_id: i32,
    pub activity_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub fn configure_repository_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/repositories")
            .service(
                web::resource("")
                    .route(web::get().to(list_repositories))
            )
            .service(
                web::resource("/{owner}/{repo}")
                    .route(web::get().to(get_repository))
            )
            .service(
                web::resource("/{owner}/{repo}/activity")
                    .route(web::get().to(get_repository_activity))
            )
            .app_data(web::Data::new(app_state.clone()))
    );
}

async fn list_repositories(
    query: web::Query<RepositoryQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    match sqlx::query_as::<_, RepositoryResponse>(
        r#"
        SELECT * FROM repositories
        ORDER BY updated_at DESC
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&app_state.pool)
    .await {
        Ok(repositories) => HttpResponse::Ok().json(repositories),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    }
}

async fn get_repository(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();

    match sqlx::query_as::<_, RepositoryResponse>(
        r#"
        SELECT * FROM repositories
        WHERE owner = $1 AND name = $2
        "#
    )
    .bind(owner)
    .bind(repo)
    .fetch_one(&app_state.pool)
    .await {
        Ok(repository) => HttpResponse::Ok().json(repository),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    }
}

async fn get_repository_activity(
    path: web::Path<(String, String)>,
    query: web::Query<RepositoryQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    match sqlx::query_as::<_, ActivityResponse>(
        r#"
        SELECT a.*, at.name as activity_type
        FROM analytics_data a
        JOIN activity_types at ON a.activity_type_id = at.id
        WHERE a.repository_id = (
            SELECT id FROM repositories
            WHERE owner = $1 AND name = $2
        )
        ORDER BY a.created_at DESC
        LIMIT $3 OFFSET $4
        "#
    )
    .bind(owner)
    .bind(repo)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&app_state.pool)
    .await {
        Ok(activities) => HttpResponse::Ok().json(activities),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    }
} 