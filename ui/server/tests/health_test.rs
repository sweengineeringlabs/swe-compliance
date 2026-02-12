mod common;

use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

/// Test that GET /health returns 200 with status "ok" and version info.
#[tokio::test]
async fn test_health_check_returns_ok() {
    let (app, _tmp) = common::test_app();
    let response = app.oneshot(common::get_no_auth("/health")).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert_eq!(body["status"], "ok");
    assert!(body["version"].is_string());
    assert!(body["engines"]["doc_engine"].is_string());
    assert!(body["engines"]["struct_engine"].is_string());
}

/// Test that POST /api/v1/auth/login with valid credentials returns 200 with token.
#[tokio::test]
async fn test_login_with_valid_credentials() {
    let (app, _tmp) = common::test_app();
    let request_body = json!({
        "username": "testuser",
        "password": "testpass123"
    });

    let response = app
        .oneshot(common::post_json_no_auth("/api/v1/auth/login", &request_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body["token"].is_string());
    assert!(!body["token"].as_str().unwrap().is_empty());
    assert_eq!(body["expires_in"], 86400);
}

/// Test that POST /api/v1/auth/login with empty username returns 400.
#[tokio::test]
async fn test_login_with_empty_username() {
    let (app, _tmp) = common::test_app();
    let request_body = json!({
        "username": "",
        "password": "testpass123"
    });

    let response = app
        .oneshot(common::post_json_no_auth("/api/v1/auth/login", &request_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("username and password are required"));
}

/// Test that POST /api/v1/auth/login with empty password returns 400.
#[tokio::test]
async fn test_login_with_empty_password() {
    let (app, _tmp) = common::test_app();
    let request_body = json!({
        "username": "testuser",
        "password": ""
    });

    let response = app
        .oneshot(common::post_json_no_auth("/api/v1/auth/login", &request_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("username and password are required"));
}

/// Test that POST /api/v1/auth/login with both empty username and password returns 400.
#[tokio::test]
async fn test_login_with_empty_credentials() {
    let (app, _tmp) = common::test_app();
    let request_body = json!({
        "username": "",
        "password": ""
    });

    let response = app
        .oneshot(common::post_json_no_auth("/api/v1/auth/login", &request_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("username and password are required"));
}

/// Test that accessing a protected endpoint without auth returns 401.
#[tokio::test]
async fn test_protected_endpoint_without_auth() {
    let (app, _tmp) = common::test_app();
    let response = app
        .oneshot(common::get_no_auth("/api/v1/projects"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("missing authentication"));
}

/// Test that accessing a protected endpoint with invalid token returns 401.
#[tokio::test]
async fn test_protected_endpoint_with_invalid_token() {
    let (app, _tmp) = common::test_app();

    let request = axum::http::Request::builder()
        .method(axum::http::Method::GET)
        .uri("/api/v1/projects")
        .header(axum::http::header::AUTHORIZATION, "Bearer invalid-token-xyz")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("invalid token"));
}

/// Test that accessing a protected endpoint with valid token returns 200.
#[tokio::test]
async fn test_protected_endpoint_with_valid_token() {
    let (app, _tmp) = common::test_app();
    let response = app
        .oneshot(common::get("/api/v1/projects"))
        .await
        .unwrap();

    // Should return 200 (empty list) or other success status, not 401
    assert_eq!(response.status(), StatusCode::OK);
}

/// Test that malformed Authorization header (missing Bearer prefix) returns 401.
#[tokio::test]
async fn test_protected_endpoint_with_malformed_auth_header() {
    let (app, _tmp) = common::test_app();

    let request = axum::http::Request::builder()
        .method(axum::http::Method::GET)
        .uri("/api/v1/projects")
        .header(axum::http::header::AUTHORIZATION, common::test_token())
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("missing authentication"));
}
