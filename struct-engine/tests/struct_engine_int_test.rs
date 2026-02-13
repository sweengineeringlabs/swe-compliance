use std::fs;
use std::path::Path;
use tempfile::TempDir;

use struct_engine::{
    default_rule_count, scan, scan_with_config, ScanConfig, ProjectKind, CheckResult,
};

fn write_file(root: &Path, relative: &str, content: &str) {
    let full = root.join(relative);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&full, content).unwrap();
}

fn create_minimal_project() -> TempDir {
    let tmp = tempfile::Builder::new().prefix("test_").tempdir().unwrap();
    let root = tmp.path();

    write_file(root, "Cargo.toml", r#"[package]
name = "test_project"
version = "0.1.0"
edition = "2021"
description = "A test project for struct-engine"
license = "MIT"
repository = "https://github.com/example/test_project"
authors = ["Test Author"]
rust-version = "1.70"
keywords = ["test"]
categories = ["development-tools"]

[lib]
path = "main/src/lib.rs"
"#);

    write_file(root, "main/src/lib.rs", r#"pub mod utils;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
"#);

    write_file(root, "main/src/utils.rs", r#"pub fn helper() -> String {
    "hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper(), "hello");
    }
}
"#);

    write_file(root, "tests/test_project_int_test.rs", r#"#[test]
fn test_integration_happy() {
    assert!(true);
}
"#);

    write_file(root, "README.md", "# Test Project\n\nA test project.\n");
    write_file(root, "CHANGELOG.md", "# Changelog\n\n## 0.1.0\n- Initial release\n");
    write_file(root, ".gitignore", "target/\n*.swp\n");

    tmp
}

fn create_rustboot_project() -> TempDir {
    let tmp = tempfile::Builder::new().prefix("test_rb_").tempdir().unwrap();
    let root = tmp.path();

    write_file(root, "Cargo.toml", r#"[package]
name = "rustboot_example"
version = "0.1.0"
edition = "2021"
description = "A rustboot test project"
license = "MIT"
repository = "https://github.com/example/rustboot_example"
authors = ["Test Author"]
rust-version = "1.70"

[lib]
path = "main/src/lib.rs"

[[test]]
name = "rustboot_example_int_test"
path = "tests/rustboot_example_int_test.rs"
"#);

    write_file(root, "main/src/lib.rs", r#"pub mod utils;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
"#);

    write_file(root, "main/src/utils.rs", r#"pub fn helper() -> String {
    "hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper(), "hello");
    }
}
"#);

    write_file(root, "tests/rustboot_example_int_test.rs", r#"#[test]
fn test_api_integration_happy() {
    assert!(true);
}
"#);

    write_file(root, "README.md", "# Rustboot Example\n\nA rustboot test project.\n");
    write_file(root, "CHANGELOG.md", "# Changelog\n\n## 0.1.0\n- Initial release\n");
    write_file(root, ".gitignore", "target/\n*.swp\n");

    tmp
}

#[test]
fn test_scan_minimal_project() {
    let tmp = create_minimal_project();
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
    let tmp = create_minimal_project();
    let report1 = scan(tmp.path()).unwrap();
    let report2 = scan_with_config(tmp.path(), &ScanConfig::default()).unwrap();
    assert_eq!(report1.results.len(), report2.results.len());
    assert_eq!(report1.summary.total, report2.summary.total);
}

#[test]
fn test_check_filter() {
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3]),
        rules_path: None,
        recursive: false,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 3);
    let ids: Vec<u8> = report.results.iter().map(|e| e.id.0).collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[test]
fn test_structure_checks_pass() {
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3]),
        rules_path: None,
        recursive: false,
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
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![9, 10, 11, 12, 13]),
        rules_path: None,
        recursive: false,
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
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![27]),
        rules_path: None,
        recursive: false,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(matches!(report.results[0].result, CheckResult::Pass));
}

#[test]
fn test_hygiene_checks_pass() {
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![43, 44]),
        rules_path: None,
        recursive: false,
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
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![39, 42]),
        rules_path: None,
        recursive: false,
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
    let tmp = create_minimal_project();
    // Checks 4 and 5 no longer have project_kind restriction — they run for all projects.
    // Since the minimal project uses main/src/ layout, both should pass.
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![4, 5]),
        rules_path: None,
        recursive: false,
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
fn test_auto_detect_library() {
    let tmp = create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    assert_eq!(report.project_kind, ProjectKind::Library);
}

#[test]
fn test_summary_math() {
    let tmp = create_minimal_project();
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
    let tmp = create_minimal_project();
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
        recursive: false,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert_eq!(report.results.len(), 1);
    assert!(matches!(report.results[0].result, CheckResult::Pass));
}

#[test]
fn test_rustboot_project_with_rustboot_rules() {
    let tmp = create_rustboot_project();
    let rules_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/rules-rustboot.toml");
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![1, 2, 3, 4, 5]),
        rules_path: Some(rules_path),
        recursive: false,
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
    let tmp = create_rustboot_project();
    let rules_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/rules-rustboot.toml");
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![28]),
        rules_path: Some(rules_path),
        recursive: false,
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
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![19]),
        rules_path: None,
        recursive: false,
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
    let tmp = create_minimal_project();
    let config = ScanConfig {
        project_kind: Some(ProjectKind::Library),
        checks: Some(vec![31]),
        rules_path: None,
        recursive: false,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    assert!(
        matches!(report.results[0].result, CheckResult::Pass),
        "Check 31 (module names) should pass: {:?}",
        report.results[0].result
    );
}
