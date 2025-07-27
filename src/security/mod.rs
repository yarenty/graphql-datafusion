use actix_web::http::header::{
    CONTENT_SECURITY_POLICY,
    // CACHE_CONTROL,  CROSS_ORIGIN_EMBEDDER_POLICY,
    // CROSS_ORIGIN_OPENER_POLICY, CROSS_ORIGIN_RESOURCE_POLICY,
    HeaderValue,
    STRICT_TRANSPORT_SECURITY,
    X_CONTENT_TYPE_OPTIONS,
    X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
};
use actix_web::{Error, FromRequest, dev::ServiceRequest, dev::ServiceResponse};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::Ready;
// use std::sync::Arc;
// use std::time::{Duration, Instant};

pub struct SecurityHeadersMiddleware;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for SecurityHeadersMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(SecurityHeadersMiddlewareService { service })
    }
}

pub struct SecurityHeadersMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for SecurityHeadersMiddlewareService<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        async {
            let mut res = fut.await?;

            // Security headers
            res.headers_mut().insert(
                STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
            );

            res.headers_mut()
                .insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

            res.headers_mut()
                .insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));

            res.headers_mut()
                .insert(X_XSS_PROTECTION, HeaderValue::from_static("1; mode=block"));

            // Content Security Policy
            res.headers_mut().insert(
                CONTENT_SECURITY_POLICY,
                HeaderValue::from_static(
                    "default-src 'self'; \
                    script-src 'self' 'unsafe-inline' 'unsafe-eval'; \
                    style-src 'self' 'unsafe-inline'; \
                    img-src 'self' data:; \
                    connect-src 'self' ws: wss:;",
                ),
            );

            // // CORS headers
            // res.headers_mut()
            //     .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
            //
            // res.headers_mut().insert(
            //     "Access-Control-Allow-Methods",
            //     HeaderValue::from_static("GET, POST, OPTIONS"),
            // );
            //
            // res.headers_mut().insert(
            //     "Access-Control-Allow-Headers",
            //     HeaderValue::from_static("Content-Type, Authorization"),
            // );

            Ok(res)
        }
    }
}

pub struct SecurityGuard;

impl FromRequest for SecurityGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // Check for potential security issues
        if req.method().as_str() == "OPTIONS" {
            return futures::future::ok(Self);
        }

        // Check for potential XSS attacks
        if let Some(content_type) = req.headers().get("Content-Type") {
            if content_type
                .to_str()
                .unwrap_or_default()
                .contains("javascript")
            {
                return futures::future::err(Error::from(actix_web::error::ErrorBadRequest(
                    "Invalid content type",
                )));
            }
        }

        // Check for potential CSRF attacks
        if let Some(referer) = req.headers().get("Referer") {
            if !referer.to_str().unwrap_or_default().starts_with("https://") {
                return futures::future::err(Error::from(actix_web::error::ErrorBadRequest(
                    "Invalid referer",
                )));
            }
        }

        futures::future::ok(Self)
    }
}
