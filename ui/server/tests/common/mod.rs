use std::path::PathBuf;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::Value;
use tempfile::TempDir;

use swe_compliance_server::auth::{issue_token, JwtSecret};
use swe_compliance_server::config::ServerConfig;
use swe_compliance_server::db::Db;
use swe_compliance_server::middleware::ScanSemaphore;
use swe_compliance_server::routes::{build_router, AppState};
use swe_compliance_server::ws::WsBroadcaster;

pub const TEST_JWT_SECRET: &str = "test-secret-for-integration-tests";

/// Create a test app with an in-memory SQLite database.
pub fn test_app() -> (Router, TempDir) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let db_path = tmp.path().join("test.db");

    let config = ServerConfig {
        host: "127.0.0.1".into(),
        port: 0,
        jwt_secret: TEST_JWT_SECRET.into(),
        db_path: db_path.clone(),
        cors_origins: vec!["*".into()],
        rate_limit_per_min: 1000,
        max_concurrent_scans: 5,
        template_dir: None,
        ai_enabled: false,
    };

    let db = Db::open(&db_path).expect("failed to open test database");

    let state = AppState {
        db,
        config,
        ws_broadcaster: WsBroadcaster::new(),
        scan_semaphore: ScanSemaphore::new(5),
    };

    let app = build_router(state);
    (app, tmp)
}

/// Create a test app with a configured template directory.
pub fn test_app_with_templates(template_dir: PathBuf) -> (Router, TempDir) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let db_path = tmp.path().join("test.db");

    let config = ServerConfig {
        host: "127.0.0.1".into(),
        port: 0,
        jwt_secret: TEST_JWT_SECRET.into(),
        db_path: db_path.clone(),
        cors_origins: vec!["*".into()],
        rate_limit_per_min: 1000,
        max_concurrent_scans: 5,
        template_dir: Some(template_dir),
        ai_enabled: false,
    };

    let db = Db::open(&db_path).expect("failed to open test database");

    let state = AppState {
        db,
        config,
        ws_broadcaster: WsBroadcaster::new(),
        scan_semaphore: ScanSemaphore::new(5),
    };

    let app = build_router(state);
    (app, tmp)
}

/// Issue a valid JWT token for testing.
pub fn test_token() -> String {
    issue_token(TEST_JWT_SECRET, "testuser").expect("failed to issue test token")
}

/// Build a GET request with auth.
pub fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

/// Build a POST request with JSON body and auth.
pub fn post_json(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", test_token()))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

/// Build a PATCH request with JSON body and auth.
pub fn patch_json(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(Method::PATCH)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", test_token()))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

/// Build a PUT request with JSON body and auth.
pub fn put_json(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(Method::PUT)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", test_token()))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

/// Build a DELETE request with auth.
pub fn delete(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::DELETE)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

/// Build a request without auth (for testing public endpoints or auth failures).
pub fn get_no_auth(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

/// Build a POST request without auth.
pub fn post_json_no_auth(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

/// Extract the response body as a JSON Value.
pub async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// Extract the response body as a string.
pub async fn body_string(response: axum::response::Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}
