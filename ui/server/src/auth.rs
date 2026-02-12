use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// JWT claims payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Issue a JWT token for a given username.
pub fn issue_token(secret: &str, username: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);
    let claims = Claims {
        sub: username.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("token creation failed: {e}")))
}

/// Validate a JWT token and return the claims.
pub fn validate_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("invalid token: {e}")))
}

/// Authenticated user extracted from request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub username: String,
}

/// Query parameter for WebSocket JWT authentication.
#[derive(Debug, Deserialize)]
pub struct WsAuthQuery {
    pub token: Option<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = parts
            .extensions
            .get::<JwtSecret>()
            .ok_or_else(|| AppError::Internal("JWT secret not configured".into()))?;

        // Try Authorization header first
        if let Some(token) = extract_bearer_token(&parts.headers) {
            let claims = validate_token(&secret.0, &token)?;
            return Ok(AuthUser {
                username: claims.sub,
            });
        }

        // Fall back to query parameter (for WebSocket connections)
        let Query(ws_query) = Query::<WsAuthQuery>::try_from_uri(&parts.uri)
            .map_err(|_| AppError::Unauthorized("missing authentication".into()))?;

        if let Some(token) = ws_query.token {
            let claims = validate_token(&secret.0, &token)?;
            return Ok(AuthUser {
                username: claims.sub,
            });
        }

        Err(AppError::Unauthorized("missing authentication".into()))
    }
}

/// Extract bearer token from Authorization header.
fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string())
}

/// Wrapper for JWT secret stored in request extensions.
#[derive(Debug, Clone)]
pub struct JwtSecret(pub String);
