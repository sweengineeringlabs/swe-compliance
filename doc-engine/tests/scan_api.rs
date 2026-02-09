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
    assert_eq!(report.summary.total, 78);
}

#[test]
fn test_scan_returns_67_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.results.len(), 78);
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
        project_type: Some(doc_engine::ProjectType::OpenSource),
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
        project_type: Some(doc_engine::ProjectType::OpenSource),
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
fn test_backlog_checks_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![69, 71, 72]),
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
fn test_module_checks_pass_no_modules() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![77, 78, 79, 80, 81]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 5);
    for entry in &report.results {
        assert!(
            matches!(entry.result, doc_engine::CheckResult::Pass),
            "Check {} should pass (no modules) but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_internal_usage_skip_open_source() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![70]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Skip { .. }),
        "Check 70 should be skipped for OpenSource but got {:?}", report.results[0].result
    );
}

#[test]
fn test_planning_checks_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![83, 84, 85, 86, 87, 88]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 6);
    for entry in &report.results {
        assert!(
            matches!(entry.result, doc_engine::CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_srs_29148_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![89]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Pass),
        "Check 89 should pass but got {:?}", report.results[0].result
    );
}

#[test]
fn test_srs_29148_skip_empty() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![89]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Skip { .. }),
        "Check 89 should skip on empty dir but got {:?}", report.results[0].result
    );
}

#[test]
fn test_arch_42010_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![90]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Pass),
        "Check 90 should pass but got {:?}", report.results[0].result
    );
}

#[test]
fn test_arch_42010_skip_empty() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![90]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Skip { .. }),
        "Check 90 should skip on empty dir but got {:?}", report.results[0].result
    );
}

#[test]
fn test_test_29119_pass_minimal() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![91]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Pass),
        "Check 91 should pass but got {:?}", report.results[0].result
    );
}

#[test]
fn test_test_29119_skip_empty() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = ScanConfig {
        project_type: Some(doc_engine::ProjectType::OpenSource),
        checks: Some(vec![91]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(
        matches!(report.results[0].result, doc_engine::CheckResult::Skip { .. }),
        "Check 91 should skip on empty dir but got {:?}", report.results[0].result
    );
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
