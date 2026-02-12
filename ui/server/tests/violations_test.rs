mod common;

use axum::http::StatusCode;
use serde_json::json;
use swe_compliance_server::db::Db;
use tower::ServiceExt;

/// Helper to create a completed scan with fake report data.
async fn setup_completed_scan(tmp: &tempfile::TempDir) -> (String, String) {
    let db = Db::open(&tmp.path().join("test.db")).unwrap();

    // Create project
    let project = db
        .create_project(
            "test-project",
            tmp.path().to_str().unwrap(),
            "Small",
            "OpenSource",
        )
        .unwrap();

    // Create scan
    let scan = db.create_scan(&project.id, "doc-engine", None).unwrap();

    // Create fake report with violations
    let fake_report = json!({
        "standard": "SWE Documentation Compliance",
        "timestamp": "2024-01-01T00:00:00Z",
        "summary": {"total": 3, "passed": 1, "failed": 1, "skipped": 1},
        "results": [
            {
                "id": 1,
                "category": "README",
                "description": "Check 1",
                "result": {"status": "pass"}
            },
            {
                "id": 2,
                "category": "LICENSE",
                "description": "Check 2",
                "result": {
                    "status": "fail",
                    "violations": [
                        {
                            "severity": "Error",
                            "path": "LICENSE",
                            "message": "Missing license file"
                        },
                        {
                            "severity": "Warning",
                            "path": "LICENSE.md",
                            "message": "License file has incorrect name"
                        }
                    ]
                }
            },
            {
                "id": 3,
                "category": "CONTRIB",
                "description": "Check 3",
                "result": {"status": "skip"}
            }
        ]
    });

    // Mark scan as completed
    db.finish_scan(&scan.id, "completed", Some(&fake_report.to_string()))
        .unwrap();

    (project.id, scan.id)
}

/// Test getting violations for completed scan returns violations array.
#[tokio::test]
async fn test_get_violations_for_completed_scan() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
    let violations = body.as_array().unwrap();
    assert_eq!(violations.len(), 2);

    // Check first violation
    assert_eq!(violations[0]["check_id"], 2);
    assert_eq!(violations[0]["category"], "LICENSE");
    assert_eq!(violations[0]["description"], "Check 2");
    assert_eq!(violations[0]["severity"], "Error");
    assert_eq!(violations[0]["file_path"], "LICENSE");
    assert_eq!(violations[0]["message"], "Missing license file");

    // Check second violation
    assert_eq!(violations[1]["check_id"], 2);
    assert_eq!(violations[1]["category"], "LICENSE");
    assert_eq!(violations[1]["description"], "Check 2");
    assert_eq!(violations[1]["severity"], "Warning");
    assert_eq!(violations[1]["file_path"], "LICENSE.md");
    assert_eq!(violations[1]["message"], "License file has incorrect name");
}

/// Test that violations contain all correct fields.
#[tokio::test]
async fn test_violations_have_correct_fields() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    let violations = body.as_array().unwrap();

    for violation in violations {
        assert!(violation["check_id"].is_number());
        assert!(violation["category"].is_string());
        assert!(violation["description"].is_string());
        assert!(violation["severity"].is_string());
        assert!(
            violation["file_path"].is_string() || violation["file_path"].is_null()
        );
        assert!(violation["message"].is_string());
    }
}

/// Test CSV format returns text/csv content type.
#[tokio::test]
async fn test_violations_csv_format() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations?format=csv",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Check content type
    let headers = response.headers();
    assert_eq!(
        headers
            .get(axum::http::header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap(),
        "text/csv"
    );

    let body = common::body_string(response).await;
    assert!(body.starts_with("check_id,category,description,severity,file_path,message"));
    assert!(body.contains("LICENSE"));
    assert!(body.contains("Missing license file"));
}

/// Test violations for running scan returns 400.
#[tokio::test]
async fn test_violations_for_running_scan() {
    let (app, tmp) = common::test_app();

    let db = Db::open(&tmp.path().join("test.db")).unwrap();

    // Create project
    let project = db
        .create_project(
            "test-project",
            tmp.path().to_str().unwrap(),
            "Small",
            "OpenSource",
        )
        .unwrap();

    // Create scan (status will be "running")
    let scan = db.create_scan(&project.id, "doc-engine", None).unwrap();

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations",
            scan.id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("not completed"));
}

/// Test violations for nonexistent scan returns 404.
#[tokio::test]
async fn test_violations_for_nonexistent_scan() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/scans/nonexistent-id/violations"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test violations for scan with no report data returns error.
#[tokio::test]
async fn test_violations_for_scan_without_report() {
    let (app, tmp) = common::test_app();

    let db = Db::open(&tmp.path().join("test.db")).unwrap();

    // Create project
    let project = db
        .create_project(
            "test-project",
            tmp.path().to_str().unwrap(),
            "Small",
            "OpenSource",
        )
        .unwrap();

    // Create scan
    let scan = db.create_scan(&project.id, "doc-engine", None).unwrap();

    // Mark as completed but without report data
    db.finish_scan(&scan.id, "completed", None).unwrap();

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations",
            scan.id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("no report data"));
}

/// Test violations with scan that has no violations (all passed).
#[tokio::test]
async fn test_violations_for_clean_scan() {
    let (app, tmp) = common::test_app();

    let db = Db::open(&tmp.path().join("test.db")).unwrap();

    // Create project
    let project = db
        .create_project(
            "test-project",
            tmp.path().to_str().unwrap(),
            "Small",
            "OpenSource",
        )
        .unwrap();

    // Create scan
    let scan = db.create_scan(&project.id, "doc-engine", None).unwrap();

    // Create report with all passing checks
    let clean_report = json!({
        "standard": "SWE Documentation Compliance",
        "timestamp": "2024-01-01T00:00:00Z",
        "summary": {"total": 2, "passed": 2, "failed": 0, "skipped": 0},
        "results": [
            {
                "id": 1,
                "category": "README",
                "description": "Check 1",
                "result": {"status": "pass"}
            },
            {
                "id": 2,
                "category": "LICENSE",
                "description": "Check 2",
                "result": {"status": "pass"}
            }
        ]
    });

    db.finish_scan(&scan.id, "completed", Some(&clean_report.to_string()))
        .unwrap();

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/violations",
            scan.id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_json(response).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}
