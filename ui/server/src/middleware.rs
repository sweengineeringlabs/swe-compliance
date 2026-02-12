use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::error::AppError;

/// Build CORS layer with configurable origins (NFR-203).
pub fn cors_layer(origins: &[String]) -> CorsLayer {
    let origin = if origins.iter().any(|o| o == "*") {
        AllowOrigin::any()
    } else {
        let values: Vec<HeaderValue> = origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        AllowOrigin::list(values)
    };

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PATCH,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .max_age(std::time::Duration::from_secs(3600))
}

/// Per-user rate limiting state (FR-1205).
#[derive(Debug, Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<HashMap<String, RateWindow>>>,
    max_per_min: u32,
}

#[derive(Debug)]
struct RateWindow {
    count: u32,
    window_start: Instant,
}

impl RateLimiter {
    pub fn new(max_per_min: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            max_per_min,
        }
    }

    fn check(&self, user: &str) -> Result<(), AppError> {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();
        let window = state.entry(user.to_string()).or_insert(RateWindow {
            count: 0,
            window_start: now,
        });

        if now.duration_since(window.window_start).as_secs() >= 60 {
            window.count = 1;
            window.window_start = now;
            Ok(())
        } else if window.count >= self.max_per_min {
            Err(AppError::RateLimited)
        } else {
            window.count += 1;
            Ok(())
        }
    }
}

/// Rate limiting middleware (FR-1205).
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let rate_limiter = request
        .extensions()
        .get::<RateLimiter>()
        .cloned();

    // Extract user from JWT claims if present
    let user = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string())
        .unwrap_or_else(|| "anonymous".into());

    if let Some(limiter) = rate_limiter {
        limiter.check(&user)?;
    }

    Ok(next.run(request).await)
}

/// Concurrent scan limiter.
#[derive(Debug, Clone)]
pub struct ScanSemaphore {
    semaphore: Arc<tokio::sync::Semaphore>,
}

impl ScanSemaphore {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(tokio::sync::Semaphore::new(max_concurrent)),
        }
    }

    pub async fn acquire(&self) -> Result<tokio::sync::OwnedSemaphorePermit, AppError> {
        self.semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| AppError::ServiceUnavailable("scan capacity exceeded".into()))
    }
}
