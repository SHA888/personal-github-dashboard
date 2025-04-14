use actix_web::{web, HttpResponse};

pub async fn list_notifications() -> HttpResponse {
    // TODO: Implement notifications listing
    HttpResponse::NotImplemented().finish()
}

pub async fn get_notification(notification_id: web::Path<i32>) -> HttpResponse {
    // TODO: Implement notification retrieval
    HttpResponse::NotImplemented().finish()
}

pub async fn mark_read(notification_id: web::Path<i32>) -> HttpResponse {
    // TODO: Implement mark as read
    HttpResponse::NotImplemented().finish()
}
