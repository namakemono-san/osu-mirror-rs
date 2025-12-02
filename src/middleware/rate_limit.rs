use std::{
    collections::HashMap,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use axum::http::StatusCode;
use axum::{body::Body, http::Request, response::Response};
use tokio::sync::RwLock;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    window: Duration,
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            window: Duration::from_secs(window_seconds),
            max_requests,
        }
    }

    pub async fn check(&self, key: &str) -> bool {
        let mut map = self.requests.write().await;
        let now = Instant::now();
        let list = map.entry(key.to_string()).or_default();

        list.retain(|t| now.duration_since(*t) < self.window);

        if list.len() >= self.max_requests {
            return false;
        }
        list.push(now);

        true
    }
}

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<RateLimiter>,
}

impl RateLimitLayer {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::new(max_requests, window_seconds)),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    limiter: Arc<RateLimiter>,
}

impl<S, B> Service<Request<B>> for RateLimitMiddleware<S>
where
    S: Service<Request<B>, Response = Response, Error = std::convert::Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = std::convert::Infallible;

    type Future = futures::future::BoxFuture<'static, Result<Response, std::convert::Infallible>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), std::convert::Infallible>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        let limiter = self.limiter.clone();

        let ip = extract_ip(&req);

        Box::pin(async move {
            if !limiter.check(&ip).await {
                let res = Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(Body::from("Rate limit exceeded"))
                    .unwrap();
                return Ok(res);
            }

            inner.call(req).await
        })
    }
}

fn extract_ip<B>(req: &Request<B>) -> String {
    if let Some(ip) = req.headers().get("cf-connecting-ip") {
        if let Ok(ip) = ip.to_str() {
            return ip.to_string();
        }
    }

    if let Some(ip) = req.headers().get("x-forwarded-for") {
        if let Ok(ip) = ip.to_str() {
            return ip.split(',').next().unwrap_or(ip).trim().to_string();
        }
    }

    if let Some(addr) = req.extensions().get::<std::net::SocketAddr>() {
        return addr.ip().to_string();
    }

    "unknown".to_string()
}
