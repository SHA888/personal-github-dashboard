use actix_web::web;

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // TODO: Add notification routes
    // _cfg.service(
    //     web::scope("/notifications")
    //         .route("", web::get().to(handlers::notifications::list))
    //         .route("/{id}", web::get().to(handlers::notifications::get))
    //         .route("/{id}/read", web::post().to(handlers::notifications::mark_read))
    // );
}
