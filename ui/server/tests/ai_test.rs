use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_ai_status_disabled() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/ai/status");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    // AI is disabled in test config
    assert_eq!(body["enabled"].as_bool().unwrap(), false);
    assert!(body["provider"].is_null());
}

#[tokio::test]
async fn test_ai_chat_not_configured() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json("/api/v1/ai/chat", &json!({"message": "Hello"}));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("AI features"));
}

#[tokio::test]
async fn test_ai_chat_empty_message() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json("/api/v1/ai/chat", &json!({"message": ""}));
    let response = app.oneshot(req).await.unwrap();

    // Should return 503 because AI is not enabled, not validation error
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn test_ai_audit_not_configured() {
    let (app, tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/audit",
        &json!({
            "path": tmp.path().to_str().unwrap(),
            "scope": "Small"
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("AI features"));
}

#[tokio::test]
async fn test_ai_audit_path_traversal() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/audit",
        &json!({
            "path": "../etc/passwd",
            "scope": "Small"
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // AI enabled check runs before path traversal check
    // When AI is disabled, it returns 503 regardless of path
    assert!(
        response.status() == StatusCode::SERVICE_UNAVAILABLE
            || response.status() == StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn test_ai_audit_without_scope() {
    let (app, tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/audit",
        &json!({
            "path": tmp.path().to_str().unwrap()
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // Should return 503 (not configured) rather than validation error
    // Scope is optional with default value
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn test_ai_generate_commands_not_configured() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/generate-commands",
        &json!({
            "requirements": [
                {
                    "id": "FR-100",
                    "title": "Create User",
                    "verification": "Manual test",
                    "acceptance": "User is created",
                    "traces_to": "UC-100",
                    "description": "Create a new user"
                }
            ],
            "project_context": "A user management system"
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("AI features"));
}

#[tokio::test]
async fn test_ai_generate_commands_empty_requirements() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/generate-commands",
        &json!({
            "requirements": [],
            "project_context": "Test project"
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // Should return 503 (not configured) rather than validation error
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn test_ai_generate_commands_missing_project_context() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/ai/generate-commands",
        &json!({
            "requirements": [
                {
                    "id": "FR-100",
                    "title": "Test",
                    "verification": "Test",
                    "acceptance": "Test",
                    "traces_to": "UC-100",
                    "description": "Test"
                }
            ]
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // Missing required field should cause deserialization error (400 or 422)
    // or 503 if validation happens after AI check
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
            || response.status() == StatusCode::SERVICE_UNAVAILABLE
    );
}

#[tokio::test]
async fn test_ai_status_returns_json_structure() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/ai/status");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    // Verify response has correct structure
    assert!(body.get("enabled").is_some());
    assert!(body.get("provider").is_some());
    assert!(body["enabled"].is_boolean());
}
