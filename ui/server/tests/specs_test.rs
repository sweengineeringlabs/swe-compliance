use axum::http::StatusCode;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_get_specs_no_files() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project = db
        .create_project("test", tmp.path().to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_specs_with_spec_file() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project_root = tmp.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();
    let project = db
        .create_project("test", project_root.to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Create a .spec file
    let docs_dir = project_root.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();
    std::fs::write(docs_dir.join("test.spec"), "spec content").unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let specs = body.as_array().unwrap();
    assert_eq!(specs.len(), 1);

    let spec = &specs[0];
    assert_eq!(spec["name"].as_str().unwrap(), "test.spec");
    assert_eq!(spec["extension"].as_str().unwrap(), "spec");
    assert_eq!(spec["domain"].as_str().unwrap(), "docs");
    assert!(spec["path"].as_str().unwrap().contains("test.spec"));
    assert!(spec["size_bytes"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_specs_with_multiple_file_types() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project_root = tmp.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();
    let project = db
        .create_project("test", project_root.to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Create multiple spec files with different extensions
    std::fs::write(project_root.join("feature.spec"), "spec").unwrap();
    std::fs::write(project_root.join("design.arch"), "arch").unwrap();
    std::fs::write(project_root.join("test.test"), "test").unwrap();
    std::fs::write(project_root.join("deployment.deploy"), "deploy").unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let specs = body.as_array().unwrap();
    assert_eq!(specs.len(), 4);

    // Verify all extensions are present
    let extensions: Vec<String> = specs
        .iter()
        .map(|s| s["extension"].as_str().unwrap().to_string())
        .collect();
    assert!(extensions.contains(&"spec".to_string()));
    assert!(extensions.contains(&"arch".to_string()));
    assert!(extensions.contains(&"test".to_string()));
    assert!(extensions.contains(&"deploy".to_string()));
}

#[tokio::test]
async fn test_get_specs_nested_directories() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project_root = tmp.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();
    let project = db
        .create_project("test", project_root.to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Create nested spec files
    let user_mgmt = project_root.join("user-management");
    std::fs::create_dir_all(&user_mgmt).unwrap();
    std::fs::write(user_mgmt.join("user.spec"), "user spec").unwrap();

    let auth = project_root.join("auth");
    std::fs::create_dir_all(&auth).unwrap();
    std::fs::write(auth.join("login.spec"), "login spec").unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let specs = body.as_array().unwrap();
    assert_eq!(specs.len(), 2);

    // Verify domain inference from parent directory
    let domains: Vec<String> = specs
        .iter()
        .map(|s| s["domain"].as_str().unwrap().to_string())
        .collect();
    assert!(domains.contains(&"user-management".to_string()));
    assert!(domains.contains(&"auth".to_string()));
}

#[tokio::test]
async fn test_get_specs_ignores_hidden_directories() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project_root = tmp.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();
    let project = db
        .create_project("test", project_root.to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Create spec in hidden directory (should be ignored)
    let hidden_dir = project_root.join(".git");
    std::fs::create_dir_all(&hidden_dir).unwrap();
    std::fs::write(hidden_dir.join("test.spec"), "hidden spec").unwrap();

    // Create spec in normal directory
    let normal_dir = project_root.join("docs");
    std::fs::create_dir_all(&normal_dir).unwrap();
    std::fs::write(normal_dir.join("test.spec"), "normal spec").unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let specs = body.as_array().unwrap();
    // Should only find the spec in normal directory
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0]["domain"].as_str().unwrap(), "docs");
}

#[tokio::test]
async fn test_get_specs_ignores_non_spec_files() {
    let (app, tmp) = common::test_app();

    // Create a project
    let db = swe_compliance_server::db::Db::open(&tmp.path().join("test.db")).unwrap();
    let project_root = tmp.path().join("project");
    std::fs::create_dir_all(&project_root).unwrap();
    let project = db
        .create_project("test", project_root.to_str().unwrap(), "Small", "OpenSource")
        .unwrap();

    // Create various files
    std::fs::write(project_root.join("test.spec"), "spec").unwrap();
    std::fs::write(project_root.join("readme.md"), "markdown").unwrap();
    std::fs::write(project_root.join("code.rs"), "rust code").unwrap();
    std::fs::write(project_root.join("data.json"), "json").unwrap();

    let req = common::get(&format!("/api/v1/projects/{}/specs", project.id));
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let specs = body.as_array().unwrap();
    // Should only find the .spec file
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0]["name"].as_str().unwrap(), "test.spec");
}

#[tokio::test]
async fn test_get_specs_nonexistent_project() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/projects/nonexistent-id/specs");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
