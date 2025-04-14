use actix_web::{web, HttpResponse};

pub async fn repository_metrics(repo_id: web::Path<i32>) -> HttpResponse {
    // TODO: Implement repository metrics
    HttpResponse::NotImplemented().finish()
}

pub async fn user_metrics() -> HttpResponse {
    // TODO: Implement user metrics
    HttpResponse::NotImplemented().finish()
}
