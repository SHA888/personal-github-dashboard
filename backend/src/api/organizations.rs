use crate::AppState;
use actix_web::{web, HttpResponse};
use log;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
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
            .route("/{org}", web::get().to(get_organization)),
    );
}

async fn list_organizations(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.github.fetch_user_organizations().await {
        Ok(octocrab_orgs) => {
            let response_orgs = octocrab_orgs
                .iter()
                .map(|org| Organization {
                    id: 0,
                    github_id: org.id.0 as i64,
                    name: org.login.clone(),
                    description: org.description.clone(),
                    avatar_url: Some(org.avatar_url.to_string()),
                    repositories_count: 0,
                })
                .collect::<Vec<_>>();

            HttpResponse::Ok().json(json!({
                "organizations": response_orgs
            }))
        }
        Err(e) => {
            log::error!("Failed to list organizations: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to list organizations",
                "message": e.to_string(),
            }))
        }
    }
}

async fn get_organization(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let org_name = path.into_inner();
    match app_state
        .github
        .list_all_repositories_and_organizations()
        .await
    {
        Ok((repositories, organizations)) => {
            if !organizations.contains(&org_name) {
                return HttpResponse::NotFound().json(json!({
                    "error": format!("Organization {} not found", org_name)
                }));
            }

            let org_repos = repositories
                .iter()
                .filter(|(owner, _)| owner == &org_name)
                .map(|(_, name)| {
                    json!({
                        "id": 0,
                        "name": name,
                        "description": null,
                        "stars": 0,
                        "forks": 0
                    })
                })
                .collect::<Vec<_>>();

            HttpResponse::Ok().json(json!({
                "id": 0,
                "github_id": 0,
                "name": org_name,
                "description": null,
                "avatar_url": null,
                "repositories": org_repos,
                "members": []
            }))
        }
        Err(e) => {
            log::error!("Failed to get organization details: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to get organization details",
                "message": e.to_string(),
            }))
        }
    }
}
