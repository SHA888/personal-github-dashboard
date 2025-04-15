use crate::{
    error::AppError,
    utils::{config::Config, jwt},
};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use std::{future::Future, pin::Pin};

pub struct AuthMiddleware;

impl Default for AuthMiddleware {
    fn default() -> Self {
        AuthMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if the route is public
        if self.is_public_route(&req) {
            return Box::pin(self.service.call(req));
        }

        // Get JWT from cookie
        let token = match get_token_from_request(&req) {
            Some(token) => token,
            None => {
                return Box::pin(async move {
                    Err(Error::from(AppError::Unauthorized(
                        "No authentication token provided".to_string(),
                    )))
                });
            }
        };

        // Get config from app data
        let config = match req.app_data::<actix_web::web::Data<Config>>() {
            Some(config) => config,
            None => {
                return Box::pin(async move {
                    Err(Error::from(AppError::InternalError(
                        "Server configuration not found".to_string(),
                    )))
                });
            }
        };

        // Validate JWT
        match jwt::validate_token(&token, &config.jwt_secret) {
            Ok(claims) => {
                // Add validated user ID to request extensions
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(fut)
            }
            Err(e) => Box::pin(async move { Err(Error::from(e)) }),
        }
    }
}

// Helper function to get token from cookie or Authorization header
fn get_token_from_request(req: &ServiceRequest) -> Option<String> {
    // First try to get from cookie
    if let Some(cookie) = req.cookie("auth_token") {
        return Some(cookie.value().to_string());
    }

    // Then try Authorization header
    if let Some(auth_header) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                return Some(stripped.to_string());
            }
        }
    }

    None
}

// Helper function to identify public routes that don't need auth
fn is_public_route(path: &str) -> bool {
    const PUBLIC_ROUTES: [&str; 4] = [
        "/api/v1/health",
        "/api/v1/auth/github",
        "/api/v1/auth/github/callback",
        "/api/v1/auth/logout",
    ];

    PUBLIC_ROUTES.iter().any(|route| path.starts_with(route))
}
