mod common;

use struct_engine::{
    default_rule_count, scan, scan_with_config, ScanConfig, ProjectKind, CheckResult,
};

#[test]
fn test_scan_minimal_project() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
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
    assert!(report.summary.failed > 0);
    assert_eq!(report.summary.total as usize, default_rule_count());
}

#[test]
fn test_scan_returns_all_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.results.len(), default_rule_count());
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
fn test_check_filter() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
    let ids: Vec<u8> = report.results.iter().map(|e| e.id.0).collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[test]
fn test_structure_checks_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_cargo_metadata_checks_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![9, 10, 11, 12, 13]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_naming_snake_case_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![27]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(matches!(report.results[0].result, CheckResult::Pass));
}

#[test]
fn test_hygiene_checks_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![43, 44]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_documentation_checks() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![39, 42]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_project_kind_skip() {
    let tmp = common::create_minimal_project();
    // Check 4 (rustboot main/src/) is workspace-only; with Library, it should skip
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![4, 5]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Skip { .. }),
            "Check {} should be skipped but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_auto_detect_library() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.project_kind, ProjectKind::Library);
}

#[test]
fn test_summary_math() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(
        report.summary.total,
        report.summary.passed + report.summary.failed + report.summary.skipped
    );
}

#[test]
fn test_nonexistent_path() {
    let result = scan(std::path::Path::new("/nonexistent/path/xyz"));
    assert!(result.is_err());
}

#[test]
fn test_custom_rules() {
    let tmp = common::create_minimal_project();
    let rules_path = tmp.path().join("custom_rules.toml");
    std::fs::write(&rules_path, r#"
[[rules]]
id = 1
category = "custom"
description = "Custom check"
severity = "info"
type = "file_exists"
path = "Cargo.toml"
"#).unwrap();

    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: None,
        rules_path: Some(rules_path),
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(matches!(report.results[0].result, CheckResult::Pass));
}

#[test]
fn test_rustboot_project_with_rustboot_rules() {
    let tmp = common::create_rustboot_project();
    let rules_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("rules-rustboot.toml");
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3, 4, 5]),
        rules_path: Some(rules_path),
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    for entry in &report.results {
        assert!(
            matches!(entry.result, CheckResult::Pass),
            "Check {} should pass but got {:?}", entry.id.0, entry.result
        );
    }
}

#[test]
fn test_rustboot_naming_check() {
    let tmp = common::create_rustboot_project();
    let rules_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("rules-rustboot.toml");
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![28]),
        rules_path: Some(rules_path),
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(
        matches!(report.results[0].result, CheckResult::Pass),
        "Check 28 (rustboot prefix) should pass for rustboot project: {:?}",
        report.results[0].result
    );
}

#[test]
fn test_lib_path_check_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![19]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(
        matches!(report.results[0].result, CheckResult::Pass),
        "Check 19 (lib path) should pass: {:?}",
        report.results[0].result
    );
}

#[test]
fn test_module_names_match_pass() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![31]),
        rules_path: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(
        matches!(report.results[0].result, CheckResult::Pass),
        "Check 31 (module names) should pass: {:?}",
        report.results[0].result
    );
}
