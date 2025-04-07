use crate::AppState;
use actix_web::{web, HttpResponse};
use log;
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Organization {
    pub id: i32,
    pub github_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub repositories_count: i32,
}

pub fn configure_organizations_routes(
    cfg: &mut web::ServiceConfig,
    app_state: &web::Data<AppState>,
) {
    cfg.service(
        web::scope("/orgs")
            .app_data(app_state.clone())
            .route("", web::get().to(list_organizations))
            .route("/{org}", web::get().to(get_organization))
            .route("/sync", web::post().to(sync_organizations)),
    );
}

async fn list_organizations(app_state: web::Data<AppState>) -> HttpResponse {
    // First, ensure we have the latest data by fetching from GitHub
    match app_state.github.fetch_user_organizations().await {
        Ok(_) => {
            // Now read from the database
            match sqlx::query_as!(
                Organization,
                r#"
                SELECT
                    o.id,
                    o.github_id,
                    o.name,
                    o.description,
                    o.avatar_url,
                    COUNT(r.id)::int as "repositories_count!"
                FROM organizations o
                LEFT JOIN repositories r ON r.owner = o.name
                GROUP BY o.id, o.github_id, o.name, o.description, o.avatar_url
                ORDER BY o.name
                "#
            )
            .fetch_all(&app_state.github.pool)
            .await
            {
                Ok(organizations) => HttpResponse::Ok().json(json!({
                    "organizations": organizations
                })),
                Err(e) => {
                    log::error!("Failed to fetch organizations from database: {}", e);
                    HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to fetch organizations",
                        "message": e.to_string(),
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to sync organizations from GitHub: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to sync organizations",
                "message": e.to_string(),
            }))
        }
    }
}

async fn get_organization(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let org_name = path.into_inner();

    // First try to sync the specific organization
    match app_state.github.sync_organization(&org_name).await {
        Ok(_) => {
            // Now read from database
            match sqlx::query!(
                r#"
                SELECT
                    o.id,
                    o.github_id,
                    o.name,
                    o.description,
                    o.avatar_url,
                    COUNT(r.id)::int as repositories_count
                FROM organizations o
                LEFT JOIN repositories r ON r.owner = o.name
                WHERE o.name = $1
                GROUP BY o.id, o.github_id, o.name, o.description, o.avatar_url
                "#,
                org_name
            )
            .fetch_optional(&app_state.github.pool)
            .await
            {
                Ok(Some(org)) => {
                    // Fetch repositories for this organization
                    match sqlx::query!(
                        r#"
                        SELECT id, name, description, stars, forks
                        FROM repositories
                        WHERE owner = $1
                        ORDER BY name
                        "#,
                        org_name
                    )
                    .fetch_all(&app_state.github.pool)
                    .await
                    {
                        Ok(repos) => {
                            let repos_json = repos
                                .iter()
                                .map(|repo| {
                                    json!({
                                        "id": repo.id,
                                        "name": repo.name,
                                        "description": repo.description,
                                        "stars": repo.stars,
                                        "forks": repo.forks
                                    })
                                })
                                .collect::<Vec<_>>();

                            HttpResponse::Ok().json(json!({
                                "id": org.id,
                                "github_id": org.github_id,
                                "name": org.name,
                                "description": org.description,
                                "avatar_url": org.avatar_url,
                                "repositories": repos_json,
                                "repositories_count": org.repositories_count,
                                "members": [] // TODO: Implement members fetching
                            }))
                        }
                        Err(e) => {
                            log::error!("Failed to fetch repositories for organization: {}", e);
                            HttpResponse::InternalServerError().json(json!({
                                "error": "Failed to fetch organization repositories",
                                "message": e.to_string(),
                            }))
                        }
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(json!({
                    "error": format!("Organization {} not found", org_name)
                })),
                Err(e) => {
                    log::error!("Failed to fetch organization from database: {}", e);
                    HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to fetch organization",
                        "message": e.to_string(),
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to sync organization from GitHub: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to sync organization",
                "message": e.to_string(),
            }))
        }
    }
}

// Endpoint to manually trigger organization sync
async fn sync_organizations(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.github.fetch_user_organizations().await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Organizations synchronized successfully"
        })),
        Err(e) => {
            log::error!("Failed to sync organizations: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to sync organizations",
                "message": e.to_string(),
            }))
        }
    }
}
