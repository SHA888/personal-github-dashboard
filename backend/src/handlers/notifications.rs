use actix_web::{web, HttpResponse};
use uuid::Uuid;

// TODO: Implement notification handlers

#[allow(unused_variables)]
pub async fn list_notifications(_user_id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::NotImplemented().body("List notifications endpoint not implemented yet")
}

#[allow(unused_variables)]
pub async fn get_notification(_notification_id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::NotImplemented().body("Get notification endpoint not implemented yet")
}

#[allow(unused_variables)]
pub async fn mark_read(_notification_id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::NotImplemented().body("Mark notification read endpoint not implemented yet")
}
