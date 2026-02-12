use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_validate_srs_empty_content() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json("/api/v1/editor/validate", &json!({"content": ""}));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert_eq!(body["error"]["message"].as_str().unwrap(), "SRS content is required");
}

#[tokio::test]
async fn test_validate_srs_with_content() {
    let (app, _tmp) = common::test_app();

    let srs_content = r#"
## 3.1 Project Management

### FR-100 Create Project
**Description:** Create a new project with name and path

### FR-101 List Projects
**Description:** List all projects
"#;

    let req = common::post_json("/api/v1/editor/validate", &json!({"content": srs_content}));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    // Verify response structure
    assert!(body["valid"].is_boolean());
    assert!(body["domain_count"].is_number());
    assert!(body["requirement_count"].is_number());
    assert!(body["domains"].is_array());
}

#[tokio::test]
async fn test_get_srs_no_content() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project = db
        .create_project("test", tmp.path().to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/srs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert_eq!(body["project_id"].as_str().unwrap(), project.id);
    assert_eq!(body["content"].as_str().unwrap(), "");
    assert_eq!(body["updated_at"].as_str().unwrap(), "");
}

#[tokio::test]
async fn test_save_srs() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project = db
        .create_project("test", tmp.path().to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    let srs_content = "# Software Requirements Specification\n\nProject requirements...";
    let req = common::put_json(
        &format!("/api/v1/projects/{}/srs", project.id),
        &json!({"content": srs_content}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert_eq!(body["project_id"].as_str().unwrap(), project.id);
    assert_eq!(body["content"].as_str().unwrap(), srs_content);
    assert!(!body["updated_at"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_get_srs_after_save() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project = db
        .create_project("test", tmp.path().to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Save SRS content
    let srs_content = "# Software Requirements Specification\n\nTest content";
    let save_req = common::put_json(
        &format!("/api/v1/projects/{}/srs", project.id),
        &json!({"content": srs_content}),
    );
    let save_response = app.clone().oneshot(save_req).await.unwrap();
    assert_eq!(save_response.status(), StatusCode::OK);

    // Get SRS content
    let get_req = common::get(&format!("/api/v1/projects/{}/srs", project.id));
    let get_response = app.oneshot(get_req).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let body = common::body_json(get_response).await;

    assert_eq!(body["project_id"].as_str().unwrap(), project.id);
    assert_eq!(body["content"].as_str().unwrap(), srs_content);
    assert!(!body["updated_at"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_get_srs_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/projects/nonexistent-id/srs");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_save_srs_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let req = common::put_json(
        "/api/v1/projects/nonexistent-id/srs",
        &json!({"content": "test content"}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
