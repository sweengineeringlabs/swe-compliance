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

    // Create fake report
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

/// Test JSON report returns application/json content type.
#[tokio::test]
async fn test_json_report_format() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!("/api/v1/scans/{}/report", scan_id)))
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
        "application/json"
    );

    let body = common::body_json(response).await;
    assert_eq!(body["standard"], "SWE Documentation Compliance");
    assert_eq!(body["timestamp"], "2024-01-01T00:00:00Z");
    assert_eq!(body["summary"]["total"], 3);
    assert_eq!(body["summary"]["passed"], 1);
    assert_eq!(body["summary"]["failed"], 1);
    assert_eq!(body["summary"]["skipped"], 1);
    assert!(body["results"].is_array());
}

/// Test JSON report with explicit format parameter.
#[tokio::test]
async fn test_json_report_with_format_param() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/report?format=json",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers
            .get(axum::http::header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap(),
        "application/json"
    );
}

/// Test markdown report contains expected headers.
#[tokio::test]
async fn test_markdown_report_format() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/report?format=markdown",
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
        "text/markdown"
    );

    let body = common::body_string(response).await;
    assert!(body.contains("# Compliance Scan Report"));
    assert!(body.contains("**Standard:** SWE Documentation Compliance"));
    assert!(body.contains("**Timestamp:** 2024-01-01T00:00:00Z"));
    assert!(body.contains("## Summary"));
    assert!(body.contains("## Results"));
    assert!(body.contains("README"));
    assert!(body.contains("LICENSE"));
    assert!(body.contains("CONTRIB"));
}

/// Test PDF format returns 400 (not implemented).
#[tokio::test]
async fn test_pdf_format_not_implemented() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/report?format=pdf",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("PDF export is not yet implemented"));
}

/// Test invalid format returns 400.
#[tokio::test]
async fn test_invalid_report_format() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/report?format=invalid",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("unsupported format"));
}

/// Test report for running scan returns 400.
#[tokio::test]
async fn test_report_for_running_scan() {
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
        .oneshot(common::get(&format!("/api/v1/scans/{}/report", scan.id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("not completed"));
}

/// Test report for nonexistent scan returns 404.
#[tokio::test]
async fn test_report_for_nonexistent_scan() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/scans/nonexistent-id/report"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test audit report contains expected ISO 15289 headers.
#[tokio::test]
async fn test_audit_report_format() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/audit-report",
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
        "text/markdown"
    );

    let body = common::body_string(response).await;
    assert!(body.contains("# Audit Status Report (ISO/IEC/IEEE 15289)"));
    assert!(body.contains("**Report ID:** ASR-"));
    assert!(body.contains("**Engine:** doc-engine"));
    assert!(body.contains("**Date:** 2024-01-01T00:00:00Z"));
    assert!(body.contains("## 1. Audit Overview"));
    assert!(body.contains("## 2. Compliance Summary"));
    assert!(body.contains("**Compliance Rate:**"));
    assert!(body.contains("## 3. Non-conformance Details"));
    assert!(body.contains("## 4. Recommendations"));
}

/// Test audit report for running scan returns 400.
#[tokio::test]
async fn test_audit_report_for_running_scan() {
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
            "/api/v1/scans/{}/audit-report",
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

/// Test audit report for nonexistent scan returns 404.
#[tokio::test]
async fn test_audit_report_for_nonexistent_scan() {
    let (app, _tmp) = common::test_app();

    let response = app
        .oneshot(common::get("/api/v1/scans/nonexistent-id/audit-report"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"].as_str().unwrap().contains("not found"));
}

/// Test report for completed scan without report data returns error.
#[tokio::test]
async fn test_report_for_scan_without_data() {
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
        .oneshot(common::get(&format!("/api/v1/scans/{}/report", scan.id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = common::body_json(response).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("no report data"));
}

/// Test audit report includes non-conformance details.
#[tokio::test]
async fn test_audit_report_includes_violations() {
    let (app, tmp) = common::test_app();
    let (_project_id, scan_id) = setup_completed_scan(&tmp).await;

    let response = app
        .clone()
        .oneshot(common::get(&format!(
            "/api/v1/scans/{}/audit-report",
            scan_id
        )))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::body_string(response).await;
    // Should include the failed check
    assert!(body.contains("Check 2"));
    assert!(body.contains("LICENSE"));
    assert!(body.contains("Missing license file"));
}
