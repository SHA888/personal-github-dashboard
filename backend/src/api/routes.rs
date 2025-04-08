use actix_web::web;
use crate::api::handlers::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/organizations")
                    .route("", web::get().to(list_organizations))
                    .route("/{id}", web::get().to(get_organization)),
            )
            .service(
                web::scope("/repositories")
                    .route("", web::get().to(list_repositories))
                    .route("/{id}", web::get().to(get_repository)),
            )
            .service(
                web::scope("/users")
                    .route("", web::get().to(list_users))
                    .route("/{id}", web::get().to(get_user)),
            ),
    );
} 