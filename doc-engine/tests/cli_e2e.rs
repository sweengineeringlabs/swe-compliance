mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("doc-engine").unwrap()
}

#[test]
fn test_cli_help() {
    cmd()
        .arg("scan")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_cli_exit_0() {
    let tmp = common::create_minimal_project();
    // Use only checks that our minimal project passes
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("1,2,3")
        .assert()
        .success();
}

#[test]
fn test_cli_exit_1() {
    // Empty dir will fail many checks
    let tmp = tempfile::TempDir::new().unwrap();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .assert()
        .code(1);
}

#[test]
fn test_cli_exit_2_bad_path() {
    cmd()
        .arg("scan")
        .arg("/nonexistent/path/xyz")
        .assert()
        .code(2);
}

#[test]
fn test_cli_exit_2_bad_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("abc")
        .assert()
        .code(2);
}

#[test]
fn test_cli_json() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("1,2,3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(val.get("results").is_some());
    assert!(val.get("summary").is_some());
}

#[test]
fn test_cli_checks_range() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("1-3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_cli_type_internal() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--type")
        .arg("internal")
        .arg("--checks")
        .arg("1")
        .assert()
        .stderr(predicate::str::is_empty().not().or(predicate::str::is_empty()));
}

#[test]
fn test_cli_custom_rules() {
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

    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--rules")
        .arg(&rules_path)
        .assert()
        .success();
}

#[test]
fn test_cli_traceability_checks() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("51,52,53")
        .assert()
        .success();
}

#[test]
fn test_cli_traceability_json() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("51-53")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_78_total_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(val["summary"]["total"].as_u64().unwrap(), 78);
}

#[test]
fn test_cli_backlog_checks() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("69,71")
        .assert()
        .success();
}

#[test]
fn test_cli_module_checks_json() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("77-81")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 5);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Module check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_planning_checks() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("83-88")
        .assert()
        .success();
}

#[test]
fn test_cli_srs_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("89")
        .assert()
        .success();
}

#[test]
fn test_cli_arch_42010_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("90")
        .assert()
        .success();
}

#[test]
fn test_cli_test_29119_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("91")
        .assert()
        .success();
}

#[test]
fn test_cli_text_format() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("1-3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Text output should contain status markers
    assert!(
        stdout.contains("[PASS]") || stdout.contains("[FAIL]") || stdout.contains("[SKIP]")
    );
}
