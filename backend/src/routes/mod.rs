use actix_web::{web, Scope};
use crate::analytics::Analytics;
use crate::collaboration::Collaboration;
use crate::health::Health;
use crate::project_health::ProjectHealth;

mod user;
mod organization;
mod repository;
mod analytics;
mod collaboration;
mod health;
mod project;

pub fn configure_routes() -> Scope {
    web::scope("/api")
        .service(user::configure_user_routes())
        .service(organization::configure_organization_routes())
        .service(repository::configure_repository_routes())
        .service(analytics::configure_analytics_routes())
        .service(collaboration::configure_collaboration_routes())
        .service(health::configure_health_routes())
        .service(project::configure_project_routes())
} 