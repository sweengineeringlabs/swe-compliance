mod common;

use doc_engine::{scan_with_config, ScanConfig, ProjectType, CheckResult};

#[test]
fn test_check_filter_single() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: ProjectType::OpenSource,
        checks: Some(vec![1]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert_eq!(report.results[0].id.0, 1);
}

#[test]
fn test_check_filter_range() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: ProjectType::OpenSource,
        checks: Some(vec![1, 2, 3]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
}

#[test]
fn test_check_filter_none() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: ProjectType::OpenSource,
        checks: None,
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 67);
}

#[test]
fn test_project_type_internal_skips() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: ProjectType::Internal,
        checks: Some(vec![31, 32]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    // Checks 31 and 32 are open_source only, should be skipped for internal
    for entry in &report.results {
        assert!(matches!(entry.result, CheckResult::Skip { .. }));
    }
}

#[test]
fn test_custom_rules_file() {
    let tmp = common::create_minimal_project();
    let rules_path = tmp.path().join("custom_rules.toml");
    std::fs::write(&rules_path, r#"
[[rules]]
id = 1
category = "custom"
description = "Custom check"
severity = "info"
type = "file_exists"
path = "README.md"
"#).unwrap();

    let config = ScanConfig {
        project_type: ProjectType::OpenSource,
        checks: None,
        rules_path: Some(rules_path),
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert_eq!(report.results[0].category, "custom");
}
