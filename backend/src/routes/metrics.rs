use actix_web::web;

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // TODO: Add metrics routes
    // _cfg.service(
    //     web::scope("/metrics")
    //         .route("/repository/{id}", web::get().to(handlers::metrics::repository_metrics))
    //         .route("/user", web::get().to(handlers::metrics::user_metrics))
    // );
}
