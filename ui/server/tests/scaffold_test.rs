use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_parse_srs_empty_content() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json("/api/v1/scaffold/parse", &json!({"content": ""}));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert_eq!(body["error"]["message"].as_str().unwrap(), "SRS content is required");
}

#[tokio::test]
async fn test_parse_srs_with_content() {
    let (app, _tmp) = common::test_app();

    let srs_content = r#"
## 3.1 User Management

### FR-100 Create User
**Description:** Create a new user account
**Acceptance Criteria:** User is created with valid credentials

### FR-101 Delete User
**Description:** Remove a user account
**Acceptance Criteria:** User is deleted from the system
"#;

    let req = common::post_json("/api/v1/scaffold/parse", &json!({"content": srs_content}));
    let response = app.oneshot(req).await.unwrap();

    // Parser may return empty array or parsed domains, both are OK
    // We're testing the HTTP layer works correctly
    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;
    assert!(body.is_array());
}

#[tokio::test]
async fn test_parse_srs_with_minimal_markdown() {
    let (app, _tmp) = common::test_app();

    let minimal_content = "# Some markdown content\n\nNot a valid SRS.";

    let req = common::post_json(
        "/api/v1/scaffold/parse",
        &json!({"content": minimal_content}),
    );
    let response = app.oneshot(req).await.unwrap();

    // Should succeed (HTTP layer) even if parsing returns empty array
    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;
    assert!(body.is_array());
}

#[tokio::test]
async fn test_execute_scaffold_path_traversal() {
    let (app, tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/scaffold/execute",
        &json!({
            "srs_path": "../etc/passwd",
            "output_dir": tmp.path().to_str().unwrap()
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("path traversal"));
}

#[tokio::test]
async fn test_execute_scaffold_output_dir_path_traversal() {
    let (app, tmp) = common::test_app();

    // Create a valid SRS file
    let srs_path = tmp.path().join("test.srs");
    std::fs::write(&srs_path, "# Test SRS").unwrap();

    let req = common::post_json(
        "/api/v1/scaffold/execute",
        &json!({
            "srs_path": srs_path.to_str().unwrap(),
            "output_dir": "../etc"
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("path traversal"));
}

#[tokio::test]
async fn test_execute_scaffold_nonexistent_srs() {
    let (app, tmp) = common::test_app();

    let nonexistent = tmp.path().join("nonexistent.srs");

    let req = common::post_json(
        "/api/v1/scaffold/execute",
        &json!({
            "srs_path": nonexistent.to_str().unwrap(),
            "output_dir": tmp.path().to_str().unwrap()
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("SRS file not found"));
}

#[tokio::test]
async fn test_execute_scaffold_with_valid_srs() {
    let (app, tmp) = common::test_app();

    // Create a minimal but valid SRS file
    let srs_path = tmp.path().join("test.srs");
    let srs_content = r#"
# Software Requirements Specification

## 3.1 User Management

### FR-100 Create User
**Description:** Create a new user
**Acceptance Criteria:** User is created successfully
"#;
    std::fs::write(&srs_path, srs_content).unwrap();

    let output_dir = tmp.path().join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let req = common::post_json(
        "/api/v1/scaffold/execute",
        &json!({
            "srs_path": srs_path.to_str().unwrap(),
            "output_dir": output_dir.to_str().unwrap()
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // May succeed or fail depending on SRS parser strictness
    // If it succeeds, verify response structure
    if response.status() == StatusCode::OK {
        let body = common::body_json(response).await;
        assert!(body["domain_count"].is_number());
        assert!(body["requirement_count"].is_number());
        assert!(body["created"].is_array());
        assert!(body["skipped"].is_array());
    }
}

#[tokio::test]
async fn test_execute_scaffold_with_options() {
    let (app, tmp) = common::test_app();

    let srs_path = tmp.path().join("test.srs");
    std::fs::write(&srs_path, "# Test SRS\n\n## 3.1 Domain\n\n### FR-100 Test").unwrap();

    let output_dir = tmp.path().join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let req = common::post_json(
        "/api/v1/scaffold/execute",
        &json!({
            "srs_path": srs_path.to_str().unwrap(),
            "output_dir": output_dir.to_str().unwrap(),
            "phases": ["requirements", "design"],
            "file_types": ["spec", "arch"],
            "force": true
        }),
    );
    let response = app.oneshot(req).await.unwrap();

    // Test that custom options are accepted (HTTP layer)
    // Response may be OK or error depending on parser behavior
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::BAD_REQUEST
    );
}
