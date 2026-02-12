mod common;

use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

/// Test creating a scan with valid data returns 202 with scan details.
#[tokio::test]
async fn test_create_scan_with_valid_data() {
    let (app, tmp) = common::test_app();

    // Create a project first
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Create a scan
    let scan_body = json!({
        "project_id": project_id,
        "engine": "doc-engine",
        "checks": "1,2,3",
        "phase": "planning",
        "module": "core"
    });

    let response = app
        .clone()
        .oneshot(common::post_json("/api/v1/scans", &scan_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let body = common::body_json(response).await;
    assert!(body["id"].is_string());
    assert_eq!(body["project_id"], project_id);
    assert_eq!(body["engine"], "doc-engine");
    assert_eq!(body["status"], "running");
    assert!(body["started_at"].is_string());
    assert!(body["finished_at"].is_null());
}

/// Test creating a scan with invalid engine returns 400.
#[tokio::test]
async fn test_create_scan_with_invalid_engine() {
    let (app, tmp) = common::test_app();

    // Create a project first
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Try to create a scan with invalid engine
    let scan_body = json!({
        "project_id": project_id,
        "engine": "invalid-engine"
    });

    let response = app
        .clone()
        .oneshot(common::post_json("/api/v1/scans", &scan_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("engine must be 'doc-engine' or 'struct-engine'"));
}

/// Test creating a scan with nonexistent project returns 404.
#[tokio::test]
async fn test_create_scan_with_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let scan_body = json!({
        "project_id": "nonexistent-project-id",
        "engine": "doc-engine"
    });

    let response = app
        .oneshot(common::post_json("/api/v1/scans", &scan_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test getting a scan by ID returns scan details.
#[tokio::test]
async fn test_get_scan_by_id() {
    let (app, tmp) = common::test_app();

    // Create a project
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Create a scan
    let scan_body = json!({
        "project_id": project_id,
        "engine": "struct-engine"
    });

    let scan_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/scans", &scan_body))
        .await
        .unwrap();

    let scan = common::body_json(scan_response).await;
    let scan_id = scan["id"].as_str().unwrap();

    // Get the scan
    let response = app
        .clone()
        .oneshot(common::get(&format!("/api/v1/scans/{}", scan_id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert_eq!(body["id"], scan_id);
    assert_eq!(body["project_id"], project_id);
    assert_eq!(body["engine"], "struct-engine");
    assert!(body["status"].is_string());
}

/// Test getting nonexistent scan returns 404.
#[tokio::test]
async fn test_get_nonexistent_scan() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/scans/nonexistent-scan-id"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test listing scans for a project returns empty array initially.
#[tokio::test]
async fn test_list_scans_for_project_empty() {
    let (app, tmp) = common::test_app();

    // Create a project
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // List scans (should be empty)
    let response = app
        .clone()
        .oneshot(common::get(&format!("/api/v1/projects/{}/scans", project_id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

/// Test listing scans for a project returns scans after creation.
#[tokio::test]
async fn test_list_scans_for_project_with_scans() {
    let (app, tmp) = common::test_app();

    // Create a project
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Create a scan
    let scan_body = json!({
        "project_id": project_id,
        "engine": "doc-engine"
    });

    let _scan_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/scans", &scan_body))
        .await
        .unwrap();

    // List scans
    let response = app
        .clone()
        .oneshot(common::get(&format!("/api/v1/projects/{}/scans", project_id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 1);
    assert_eq!(body[0]["project_id"], project_id);
    assert_eq!(body[0]["engine"], "doc-engine");
}

/// Test listing scans for nonexistent project returns 404.
#[tokio::test]
async fn test_list_scans_for_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/projects/nonexistent-id/scans"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test getting trends for a project returns empty array initially.
#[tokio::test]
async fn test_get_trends_for_project_empty() {
    let (app, tmp) = common::test_app();

    // Create a project
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Get trends (should be empty)
    let response = app
        .clone()
        .oneshot(common::get(&format!("/api/v1/projects/{}/trends", project_id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

/// Test getting trends with since query parameter.
#[tokio::test]
async fn test_get_trends_with_since_parameter() {
    let (app, tmp) = common::test_app();

    // Create a project
    let project_body = json!({
        "name": "test-project",
        "root_path": tmp.path().to_str().unwrap(),
        "scope": "Small",
        "project_type": "OpenSource"
    });

    let project_response = app
        .clone()
        .oneshot(common::post_json("/api/v1/projects", &project_body))
        .await
        .unwrap();

    let project = common::body_json(project_response).await;
    let project_id = project["id"].as_str().unwrap();

    // Get trends with since parameter
    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/projects/{}/trends?since=2024-01-01T00:00:00Z",
            project_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
}

/// Test getting trends for nonexistent project returns 404.
#[tokio::test]
async fn test_get_trends_for_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/projects/nonexistent-id/trends"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}
