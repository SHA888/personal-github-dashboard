use actix_web::web;

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // TODO: Add authentication routes
    // _cfg.service(
    //     web::scope("/auth")
    //         .route("/login", web::get().to(handlers::auth::github_login))
    //         .route("/callback", web::get().to(handlers::auth::github_callback))
    //         .route("/logout", web::post().to(handlers::auth::logout))
    // );
}
