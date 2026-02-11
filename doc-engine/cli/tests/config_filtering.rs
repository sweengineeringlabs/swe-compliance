mod common;

use doc_engine_scan::{default_rule_count, scan_with_config, ScanConfig, ProjectScope, ProjectType, CheckResult};

#[test]
fn test_check_filter_single() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
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
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
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
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), default_rule_count());
}

#[test]
fn test_project_type_internal_skips() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::Internal),
        project_scope: ProjectScope::Large,
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
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: Some(rules_path),
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert_eq!(report.results[0].category, "custom");
}

#[test]
fn test_scope_small_integration() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Small,
        checks: None,
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    // With scope=small, medium and large rules should be skipped
    let total = report.results.len();
    assert_eq!(total, default_rule_count());
    // Count scope-skipped checks
    let scope_skipped: usize = report.results.iter().filter(|e| {
        if let CheckResult::Skip { ref reason } = e.result {
            reason.contains("scope")
        } else {
            false
        }
    }).count();
    assert!(scope_skipped > 0, "Expected some checks to be skipped for scope");
}

#[test]
fn test_scope_medium_runs_small_and_medium() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Medium,
        checks: Some(vec![1, 11, 89]),  // small=1, medium=11, large=89
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
    // Check 1 (small) should NOT be scope-skipped
    assert!(!matches!(report.results[0].result, CheckResult::Skip { ref reason } if reason.contains("scope")));
    // Check 11 (medium) should NOT be scope-skipped
    assert!(!matches!(report.results[1].result, CheckResult::Skip { ref reason } if reason.contains("scope")));
    // Check 89 (large) SHOULD be scope-skipped
    assert!(matches!(report.results[2].result, CheckResult::Skip { ref reason } if reason.contains("scope")));
}

#[test]
fn test_scope_report_field() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Small,
        checks: Some(vec![1]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.project_scope, ProjectScope::Small);
}

#[test]
fn test_scope_json_output() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Medium,
        checks: Some(vec![1]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let json = doc_engine_scan::format_report_json(&report);
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(val["project_scope"], "medium");
}
