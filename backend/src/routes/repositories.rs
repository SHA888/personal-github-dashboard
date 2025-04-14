use actix_web::web;

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // TODO: Add repository routes
    // _cfg.service(
    //     web::scope("/repositories")
    //         .route("", web::get().to(handlers::repositories::list))
    //         .route("/{id}", web::get().to(handlers::repositories::get))
    // );
}
