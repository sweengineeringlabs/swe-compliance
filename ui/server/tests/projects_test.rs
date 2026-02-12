mod common;

use axum::http::StatusCode;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_project_with_valid_data() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Test Project",
            "root_path": root
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let body = common::body_json(res).await;
    assert!(body["id"].as_str().is_some());
    assert_eq!(body["name"], "Test Project");
    assert_eq!(body["root_path"], root);
    assert_eq!(body["scope"], "Small");
    assert_eq!(body["project_type"], "OpenSource");
    assert!(body["created_at"].as_str().is_some());
    assert!(body["updated_at"].as_str().is_some());
}

#[tokio::test]
async fn test_create_project_with_custom_scope_and_type() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Large Internal Project",
            "root_path": root,
            "scope": "Large",
            "project_type": "Internal"
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let body = common::body_json(res).await;
    assert_eq!(body["scope"], "Large");
    assert_eq!(body["project_type"], "Internal");
}

#[tokio::test]
async fn test_create_project_with_empty_name() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "",
            "root_path": root
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("name is required"));
}

#[tokio::test]
async fn test_create_project_with_whitespace_only_name() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "   ",
            "root_path": root
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_project_with_path_traversal_dotdot() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Malicious Project",
            "root_path": format!("{root}/../etc")
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("path traversal"));
}

#[tokio::test]
async fn test_create_project_with_null_byte() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Malicious Project",
            "root_path": format!("{root}\0/etc")
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_project_with_nonexistent_path() {
    let (app, _tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Nonexistent Project",
            "root_path": "/path/that/does/not/exist/12345"
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("root_path does not exist"));
}

#[tokio::test]
async fn test_create_project_with_invalid_scope() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Invalid Scope Project",
            "root_path": root,
            "scope": "Huge"
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("invalid scope"));
}

#[tokio::test]
async fn test_create_project_with_invalid_project_type() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Invalid Type Project",
            "root_path": root,
            "project_type": "Commercial"
        }),
    );

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("invalid project_type"));
}

#[tokio::test]
async fn test_list_projects_returns_created_projects() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create first project
    let req1 = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project Alpha",
            "root_path": root
        }),
    );
    app.clone().oneshot(req1).await.unwrap();

    // Create second project
    let req2 = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project Beta",
            "root_path": root
        }),
    );
    app.clone().oneshot(req2).await.unwrap();

    // List projects
    let req = common::get("/api/v1/projects");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    let projects = body.as_array().unwrap();
    assert_eq!(projects.len(), 2);

    // Projects are ordered by created_at DESC, so Beta should be first
    assert_eq!(projects[0]["name"], "Project Beta");
    assert_eq!(projects[1]["name"], "Project Alpha");
}

#[tokio::test]
async fn test_list_projects_when_empty() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/projects");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    let projects = body.as_array().unwrap();
    assert_eq!(projects.len(), 0);
}

#[tokio::test]
async fn test_get_project_by_id_returns_correct_project() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Specific Project",
            "root_path": root,
            "scope": "Medium"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Get project by ID
    let req = common::get(&format!("/api/v1/projects/{}", project_id));
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    assert_eq!(body["id"], project_id);
    assert_eq!(body["name"], "Specific Project");
    assert_eq!(body["scope"], "Medium");
}

#[tokio::test]
async fn test_get_nonexistent_project_returns_404() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/projects/00000000-0000-0000-0000-000000000000");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn test_update_project_name() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Original Name",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Update project name
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "name": "Updated Name"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    assert_eq!(body["name"], "Updated Name");
    assert_eq!(body["id"], project_id);

    // Verify name was updated
    let req = common::get(&format!("/api/v1/projects/{}", project_id));
    let res = app.oneshot(req).await.unwrap();
    let body = common::body_json(res).await;
    assert_eq!(body["name"], "Updated Name");
}

#[tokio::test]
async fn test_update_project_scope() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project with default scope
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Test Project",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();
    assert_eq!(create_body["scope"], "Small");

    // Update scope
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "scope": "Large"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    assert_eq!(body["scope"], "Large");
}

#[tokio::test]
async fn test_update_project_type() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project with default type
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Test Project",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();
    assert_eq!(create_body["project_type"], "OpenSource");

    // Update project_type
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "project_type": "Internal"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    assert_eq!(body["project_type"], "Internal");
}

#[tokio::test]
async fn test_update_multiple_fields() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Original",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Update multiple fields
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "name": "Updated",
            "scope": "Medium",
            "project_type": "Internal"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    assert_eq!(body["name"], "Updated");
    assert_eq!(body["scope"], "Medium");
    assert_eq!(body["project_type"], "Internal");
}

#[tokio::test]
async fn test_update_project_with_invalid_scope() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Test Project",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Try to update with invalid scope
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "scope": "ExtraLarge"
        }),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("invalid scope"));
}

#[tokio::test]
async fn test_update_project_with_invalid_project_type() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Test Project",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Try to update with invalid project_type
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "project_type": "Private"
        }),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(res).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("invalid project_type"));
}

#[tokio::test]
async fn test_update_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let req = common::patch_json(
        "/api/v1/projects/00000000-0000-0000-0000-000000000000",
        &serde_json::json!({
            "name": "Updated"
        }),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_project_returns_204() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project to Delete",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Delete project
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_get_deleted_project_returns_404() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project to Delete",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Delete project
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    app.clone().oneshot(req).await.unwrap();

    // Try to get deleted project
    let req = common::get(&format!("/api/v1/projects/{}", project_id));
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_projects_excludes_deleted() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create first project
    let req1 = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project One",
            "root_path": root
        }),
    );
    app.clone().oneshot(req1).await.unwrap();

    // Create second project
    let req2 = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project Two",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req2).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Delete second project
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    app.clone().oneshot(req).await.unwrap();

    // List projects should only return first project
    let req = common::get("/api/v1/projects");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = common::body_json(res).await;
    let projects = body.as_array().unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0]["name"], "Project One");
}

#[tokio::test]
async fn test_delete_nonexistent_project_returns_404() {
    let (app, _tmp) = common::test_app();

    let req = common::delete("/api/v1/projects/00000000-0000-0000-0000-000000000000");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_already_deleted_project_returns_404() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project to Delete Twice",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Delete project first time
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Try to delete again
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_deleted_project_returns_404() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create project
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "Project to Delete",
            "root_path": root
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    let create_body = common::body_json(res).await;
    let project_id = create_body["id"].as_str().unwrap();

    // Delete project
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    app.clone().oneshot(req).await.unwrap();

    // Try to update deleted project
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "name": "Updated Name"
        }),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_full_crud_workflow() {
    let (app, tmp) = common::test_app();
    let root = tmp.path().to_str().unwrap();

    // Create
    let req = common::post_json(
        "/api/v1/projects",
        &serde_json::json!({
            "name": "CRUD Test Project",
            "root_path": root,
            "scope": "Small",
            "project_type": "OpenSource"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = common::body_json(res).await;
    let project_id = body["id"].as_str().unwrap().to_string();

    // Read (Get)
    let req = common::get(&format!("/api/v1/projects/{}", project_id));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = common::body_json(res).await;
    assert_eq!(body["name"], "CRUD Test Project");

    // Read (List)
    let req = common::get("/api/v1/projects");
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = common::body_json(res).await;
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Update
    let req = common::patch_json(
        &format!("/api/v1/projects/{}", project_id),
        &serde_json::json!({
            "name": "Updated CRUD Test Project",
            "scope": "Large"
        }),
    );
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = common::body_json(res).await;
    assert_eq!(body["name"], "Updated CRUD Test Project");
    assert_eq!(body["scope"], "Large");

    // Delete
    let req = common::delete(&format!("/api/v1/projects/{}", project_id));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify deletion (Get should return 404)
    let req = common::get(&format!("/api/v1/projects/{}", project_id));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // Verify deletion (List should be empty)
    let req = common::get("/api/v1/projects");
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = common::body_json(res).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}
