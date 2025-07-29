//! Minimal security module
//!
//! This module provides basic security functionality.
//! For now, it's a minimal implementation that can be extended later.

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use futures_util::future::{LocalBoxFuture, Ready, ready};

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_cors: bool,
    pub enable_https_redirect: bool,
    pub enable_content_security_policy: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_cors: true,
            enable_https_redirect: false,
            enable_content_security_policy: true,
        }
    }
}

/// Security middleware
#[derive(Debug, Clone)]
pub struct SecurityMiddleware {
    config: SecurityConfig,
}

impl SecurityMiddleware {
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SecurityMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityMiddlewareService {
            service,
            config: self.config.clone(),
        }))
    }
}

pub struct SecurityMiddlewareService<S> {
    service: S,
    config: SecurityConfig,
}

impl<S, B> Service<ServiceRequest> for SecurityMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // For now, just pass through without security checks
        // This can be extended later with proper security headers and checks
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
