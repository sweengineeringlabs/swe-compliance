use std::path::Path;
use doc_engine_scan::{scan_with_config, ScanConfig, ScanError, ProjectType, ProjectScope};

#[test]
fn test_nonexistent_path() {
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
    };
    let result = scan_with_config(Path::new("/nonexistent/path/xyz"), &config);
    assert!(result.is_err());
    match result.unwrap_err() {
        ScanError::Path(msg) => assert!(msg.contains("does not exist")),
        other => panic!("Expected ScanError::Path, got: {:?}", other),
    }
}

#[test]
fn test_bad_rules_path() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: Some("/nonexistent/rules.toml".into()),
    };
    let result = scan_with_config(tmp.path(), &config);
    assert!(result.is_err());
    match result.unwrap_err() {
        ScanError::Config(msg) => assert!(msg.contains("Cannot read rules file")),
        other => panic!("Expected ScanError::Config, got: {:?}", other),
    }
}

#[test]
fn test_malformed_rules() {
    let tmp = tempfile::TempDir::new().unwrap();
    let rules_path = tmp.path().join("bad_rules.toml");
    std::fs::write(&rules_path, "not valid toml {{{{").unwrap();

    let config = ScanConfig {
        project_type: Some(ProjectType::OpenSource),
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: Some(rules_path),
    };
    let result = scan_with_config(tmp.path(), &config);
    assert!(result.is_err());
    match result.unwrap_err() {
        ScanError::Config(msg) => assert!(msg.contains("TOML parse error")),
        other => panic!("Expected ScanError::Config, got: {:?}", other),
    }
}
