use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::{issue_token, JwtSecret};
use crate::error::AppError;
use crate::routes::AppState;

/// Health check response (FR-1202).
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub engines: EngineVersions,
}

#[derive(Debug, Serialize)]
pub struct EngineVersions {
    pub doc_engine: String,
    pub struct_engine: String,
}

/// GET /health — returns server version and engine versions.
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        engines: EngineVersions {
            doc_engine: "0.1.0".into(),
            struct_engine: "0.1.0".into(),
        },
    })
}

/// Login request body.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response body.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: u64,
}

/// POST /api/v1/auth/login — issue JWT token (FR-1201).
pub async fn login(
    axum::Extension(secret): axum::Extension<JwtSecret>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // In production, validate credentials against a user store.
    // For development, accept any non-empty username/password.
    if body.username.is_empty() || body.password.is_empty() {
        return Err(AppError::BadRequest(
            "username and password are required".into(),
        ));
    }

    let token = issue_token(&secret.0, &body.username)?;
    Ok(Json(LoginResponse {
        token,
        expires_in: 86400, // 24 hours
    }))
}
