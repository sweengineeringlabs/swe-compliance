mod common;

use doc_engine::{scan, scan_with_config, ScanConfig};

#[test]
fn test_scan_minimal_project() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    // A minimal compliant project should have many passes
    assert!(report.summary.passed > 0);
    assert_eq!(
        report.summary.total,
        report.summary.passed + report.summary.failed + report.summary.skipped
    );
}

#[test]
fn test_scan_empty_dir() {
    let tmp = tempfile::TempDir::new().unwrap();
    let report = scan(tmp.path()).unwrap();
    // Should have many failures but no panics
    assert!(report.summary.failed > 0);
    assert_eq!(report.summary.total, 53);
}

#[test]
fn test_scan_returns_53_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.results.len(), 53);
}

#[test]
fn test_scan_with_config_default() {
    let tmp = common::create_minimal_project();
    let report1 = scan(tmp.path()).unwrap();
    let report2 = scan_with_config(tmp.path(), &ScanConfig::default()).unwrap();
    assert_eq!(report1.results.len(), report2.results.len());
    assert_eq!(report1.summary.total, report2.summary.total);
}

#[test]
fn test_traceability_checks_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: doc_engine::ProjectType::OpenSource,
        checks: Some(vec![51, 52, 53]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
    for entry in &report.results {
        assert!(
            matches!(entry.result, doc_engine::CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_traceability_checks_skip_empty() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = ScanConfig {
        project_type: doc_engine::ProjectType::OpenSource,
        checks: Some(vec![51, 52, 53]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
    for entry in &report.results {
        assert!(
            matches!(entry.result, doc_engine::CheckResult::Skip { .. }),
            "Check {} should skip on empty dir but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_scan_summary_math() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(
        report.summary.total,
        report.summary.passed + report.summary.failed + report.summary.skipped
    );
}
