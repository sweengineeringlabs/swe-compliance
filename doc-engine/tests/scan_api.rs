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
    assert_eq!(report.summary.total, 50);
}

#[test]
fn test_scan_returns_50_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.results.len(), 50);
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
fn test_scan_summary_math() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(
        report.summary.total,
        report.summary.passed + report.summary.failed + report.summary.skipped
    );
}
