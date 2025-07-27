use actix_web::{Error, FromRequest, dev::ServiceRequest, dev::ServiceResponse};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::Ready;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct RateLimiter {
    limits: HashMap<String, LimitConfig>,
    state: Arc<Mutex<HashMap<String, RateLimitState>>>,
}

#[derive(Debug)]
pub struct LimitConfig {
    pub limit: u32,
    pub window: Duration,
    pub burst: u32,
}

#[derive(Debug)]
struct RateLimitState {
    count: u32,
    last_reset: Instant,
    burst_count: u32,
    last_burst: Instant,
}

impl RateLimiter {
    pub fn new() -> Self {
        let limits = HashMap::from([
            (
                "query".to_string(),
                LimitConfig {
                    limit: 100,
                    window: Duration::from_secs(60),
                    burst: 5,
                },
            ),
            (
                "subscription".to_string(),
                LimitConfig {
                    limit: 50,
                    window: Duration::from_secs(60),
                    burst: 3,
                },
            ),
        ]);

        Self {
            limits,
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn check_limit(&self, endpoint: &str, ip: &str) -> Result<(), Error> {
        let mut state = self.state.lock().await;
        let config = self.limits.get(endpoint).unwrap_or_else(|| {
            &self.limits["query"] // Default to query limits
        });

        let key = format!("{}:{}", endpoint, ip);
        let current = state.entry(key.clone()).or_insert(RateLimitState {
            count: 0,
            last_reset: Instant::now(),
            burst_count: 0,
            last_burst: Instant::now(),
        });

        // Check burst limit
        if current.burst_count >= config.burst {
            if Instant::now() - current.last_burst > Duration::from_secs(1) {
                current.burst_count = 1;
                current.last_burst = Instant::now();
            } else {
                return Err(Error::from(actix_web::error::ErrorTooManyRequests(
                    "Rate limit exceeded (burst)",
                )));
            }
        } else {
            current.burst_count += 1;
            current.last_burst = Instant::now();
        }

        // Check main limit
        if Instant::now() - current.last_reset > config.window {
            current.count = 1;
            current.last_reset = Instant::now();
        } else if current.count >= config.limit {
            return Err(Error::from(actix_web::error::ErrorTooManyRequests(
                "Rate limit exceeded",
            )));
        } else {
            current.count += 1;
        }

        Ok(())
    }
}

pub struct RateLimitMiddleware {
    limiter: Arc<RateLimiter>,
}

impl RateLimitMiddleware {
    pub fn new(limiter: Arc<RateLimiter>) -> Self {
        Self { limiter }
    }
}

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(RateLimitMiddlewareService {
            service,
            limiter: self.limiter.clone(),
        })
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    limiter: Arc<RateLimiter>,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        let endpoint = req.path().to_string();

        let result = self.limiter.check_limit(&endpoint, &ip);

        //TODO: read all this from config!!!!
        if let Err(e) = result {
            return self.service.call(req).map(|res| {
                res.map(|mut res| {
                    res.headers_mut().insert(
                        "X-RateLimit-Limit",
                        format!("{}/min", crate::config::Config::default().max_batch_size)
                            .parse()
                            .unwrap(),
                    );
                    res.headers_mut()
                        .insert("X-RateLimit-Remaining", "0".parse().unwrap());
                    res.headers_mut()
                        .insert("Retry-After", "60".parse().unwrap());
                    res
                })
            });
        }

        self.service.call(req)
    }
}

#[derive(Debug)]
pub struct RateLimitGuard;

impl FromRequest for RateLimitGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        let endpoint = req.path().to_string();
        let limiter = req.app_data::<Arc<RateLimiter>>().unwrap();

        let result = limiter.check_limit(&endpoint, &ip);

        if let Err(e) = result {
            futures::future::err(e)
        } else {
            futures::future::ok(Self)
        }
    }
}
