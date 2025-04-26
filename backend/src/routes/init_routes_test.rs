use crate::handlers::{activities, organizations, repositories, users};
use actix_web::web;

pub fn init_routes_no_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/user/{id}", web::get().to(users::get_user_by_id_handler))
            .route(
                "/organization/{id}",
                web::get().to(organizations::get_organization_by_id),
            )
            .route(
                "/repository/{id}",
                web::get().to(repositories::get_repository_by_id),
            )
            .route(
                "/activity/{id}",
                web::get().to(activities::get_activity_by_id),
            ),
    );
}
