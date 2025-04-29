// TODO: Implement organizations handler module for organization-related endpoints.

use crate::db::{
    create_organization_with_cache, delete_organization_with_cache,
    get_organization_by_id_with_cache, update_organization_description_with_cache,
};
use crate::utils::redis::RedisClient;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateOrganizationDescriptionRequest {
    pub description: Option<String>,
}

pub async fn get_organization_by_id(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    org_id: web::Path<Uuid>,
) -> HttpResponse {
    match get_organization_by_id_with_cache(&pool, &redis, &org_id).await {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn create_organization(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    req: web::Json<CreateOrganizationRequest>,
) -> HttpResponse {
    match create_organization_with_cache(&pool, &redis, &req.name, req.description.as_deref()).await
    {
        Ok(org) => HttpResponse::Created().json(org),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn update_organization_description(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    org_id: web::Path<Uuid>,
    req: web::Json<UpdateOrganizationDescriptionRequest>,
) -> HttpResponse {
    match update_organization_description_with_cache(
        &pool,
        &redis,
        &org_id,
        req.description.as_deref(),
    )
    .await
    {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn delete_organization(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    org_id: web::Path<Uuid>,
) -> HttpResponse {
    match delete_organization_with_cache(&pool, &redis, &org_id).await {
        Ok(affected) if affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}
