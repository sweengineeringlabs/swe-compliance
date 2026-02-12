use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_list_templates_without_configured_dir() {
    let (app, _tmp) = common::test_app();

    let req = common::get("/api/v1/templates");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("template directory not configured"));
}

#[tokio::test]
async fn test_copy_template_without_configured_dir() {
    let (app, tmp) = common::test_app();

    let req = common::post_json(
        "/api/v1/templates/test.md/copy",
        &json!({"destination": tmp.path().to_str().unwrap()}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("template directory not configured"));
}

#[tokio::test]
async fn test_list_templates_empty_dir() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    let (app, _tmp2) = common::test_app_with_templates(template_dir);

    let req = common::get("/api/v1/templates");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_templates_with_files() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    // Create template files
    std::fs::write(template_dir.join("readme.md"), "# Template README").unwrap();
    std::fs::write(template_dir.join("spec-template.spec"), "Spec template").unwrap();

    let (app, _tmp2) = common::test_app_with_templates(template_dir);

    let req = common::get("/api/v1/templates");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let templates = body.as_array().unwrap();
    assert_eq!(templates.len(), 2);

    // Verify template structure
    for template in templates {
        assert!(template.get("name").is_some());
        assert!(template.get("path").is_some());
        assert!(template.get("size_bytes").is_some());
        assert!(template["size_bytes"].as_u64().unwrap() > 0);
    }
}

#[tokio::test]
async fn test_list_templates_nested_directories() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    // Create nested template structure
    let specs_dir = template_dir.join("specs");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(specs_dir.join("feature.spec"), "Feature spec").unwrap();

    let docs_dir = template_dir.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();
    std::fs::write(docs_dir.join("readme.md"), "README").unwrap();

    let (app, _tmp2) = common::test_app_with_templates(template_dir);

    let req = common::get("/api/v1/templates");
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = common::body_json(response).await;

    assert!(body.is_array());
    let templates = body.as_array().unwrap();
    assert_eq!(templates.len(), 2);

    // Verify paths include subdirectories
    let paths: Vec<String> = templates
        .iter()
        .map(|t| t["path"].as_str().unwrap().to_string())
        .collect();
    assert!(paths.iter().any(|p| p.contains("specs")));
    assert!(paths.iter().any(|p| p.contains("docs")));
}

#[tokio::test]
async fn test_copy_template_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    let template_content = "# Template Content\n\nThis is a template.";
    std::fs::write(template_dir.join("readme.md"), template_content).unwrap();

    let (app, tmp2) = common::test_app_with_templates(template_dir);

    let dest_dir = tmp2.path().join("destination");
    let req = common::post_json(
        "/api/v1/templates/readme.md/copy",
        &json!({"destination": dest_dir.to_str().unwrap()}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify file was copied
    let copied_file = dest_dir.join("readme.md");
    assert!(copied_file.exists());
    let content = std::fs::read_to_string(copied_file).unwrap();
    assert_eq!(content, template_content);
}

#[tokio::test]
async fn test_copy_nonexistent_template() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    let (app, tmp2) = common::test_app_with_templates(template_dir);

    let dest_dir = tmp2.path().join("destination");
    let req = common::post_json(
        "/api/v1/templates/nonexistent.md/copy",
        &json!({"destination": dest_dir.to_str().unwrap()}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("template not found"));
}

#[tokio::test]
async fn test_copy_template_path_traversal_in_name() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    let (app, tmp2) = common::test_app_with_templates(template_dir);

    let dest_dir = tmp2.path().join("destination");
    // Use a single-segment name containing ".." (multi-segment paths won't match the route)
    let req = common::post_json(
        "/api/v1/templates/..secret/copy",
        &json!({"destination": dest_dir.to_str().unwrap()}),
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
async fn test_copy_template_path_traversal_in_destination() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();
    std::fs::write(template_dir.join("test.md"), "content").unwrap();

    let (app, _tmp2) = common::test_app_with_templates(template_dir);

    let req = common::post_json(
        "/api/v1/templates/test.md/copy",
        &json!({"destination": "../etc"}),
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
async fn test_copy_template_directory() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();

    // Create a template directory with files
    let template_subdir = template_dir.join("project-template");
    std::fs::create_dir_all(&template_subdir).unwrap();
    std::fs::write(template_subdir.join("readme.md"), "README").unwrap();
    std::fs::write(template_subdir.join("config.yaml"), "config").unwrap();

    let (app, tmp2) = common::test_app_with_templates(template_dir);

    let dest_dir = tmp2.path().join("destination");
    let req = common::post_json(
        "/api/v1/templates/project-template/copy",
        &json!({"destination": dest_dir.to_str().unwrap()}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify directory and files were copied
    let copied_dir = dest_dir.join("project-template");
    assert!(copied_dir.exists());
    assert!(copied_dir.join("readme.md").exists());
    assert!(copied_dir.join("config.yaml").exists());
}

#[tokio::test]
async fn test_copy_template_to_existing_directory() {
    let tmp = tempfile::TempDir::new().unwrap();
    let template_dir = tmp.path().join("templates");
    std::fs::create_dir_all(&template_dir).unwrap();
    std::fs::write(template_dir.join("test.md"), "template content").unwrap();

    let (app, tmp2) = common::test_app_with_templates(template_dir);

    // Create destination directory with existing file
    let dest_dir = tmp2.path().join("destination");
    std::fs::create_dir_all(&dest_dir).unwrap();
    std::fs::write(dest_dir.join("existing.txt"), "existing").unwrap();

    let req = common::post_json(
        "/api/v1/templates/test.md/copy",
        &json!({"destination": dest_dir.to_str().unwrap()}),
    );
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify both files exist
    assert!(dest_dir.join("test.md").exists());
    assert!(dest_dir.join("existing.txt").exists());
}
