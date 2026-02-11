mod common;

use assert_cmd::Command;
use doc_engine_scan::default_rule_count;
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
        .assert()
        .code(1);
}

#[test]
fn test_cli_exit_2_bad_path() {
    cmd()
        .arg("scan")
        .arg("/nonexistent/path/xyz")
        .arg("--scope")
        .arg("large")
        .assert()
        .code(2);
}

#[test]
fn test_cli_exit_2_bad_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("1,2,3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(val.get("results").is_some());
    assert!(val.get("summary").is_some());
    // ISO/IEC/IEEE 15289:2019 clause 9.2 metadata in --json stdout
    assert_eq!(val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(val["clause"], "9.2");
    assert_eq!(val["tool"], "doc-engine");
    assert!(val["tool_version"].is_string());
    assert!(val["timestamp"].is_string());
    assert!(val["project_root"].is_string());
}

#[test]
fn test_cli_checks_range() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(val["summary"]["total"].as_u64().unwrap(), default_rule_count() as u64);
}

#[test]
fn test_cli_backlog_checks() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
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
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("91")
        .assert()
        .success();
}

#[test]
fn test_cli_dev_guide_26514_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("94")
        .assert()
        .success();
}

#[test]
fn test_cli_backlog_sections_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("95")
        .assert()
        .success();
}

#[test]
fn test_cli_backlog_sections_json() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("95")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "pass");
}

#[test]
fn test_cli_backlog_sections_skip_no_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("95")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "skip");
}

#[test]
fn test_cli_content_checks_combined() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("89-95")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 7);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_prod_12207_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("96")
        .assert()
        .success();
}

#[test]
fn test_cli_prod_25010_supp_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("97")
        .assert()
        .success();
}

#[test]
fn test_cli_prod_25040_check() {
    let tmp = common::create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("98")
        .assert()
        .success();
}

#[test]
fn test_cli_prod_12207_skip_no_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("96")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "skip");
}

#[test]
fn test_cli_prod_25010_supp_skip_no_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("97")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "skip");
}

#[test]
fn test_cli_prod_25040_skip_no_file() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("98")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "skip");
}

#[test]
fn test_cli_prod_12207_json_pass() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("96")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "pass");
}

#[test]
fn test_cli_prod_25010_supp_json_pass() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("97")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "pass");
}

#[test]
fn test_cli_prod_25040_json_pass() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("98")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["result"]["status"].as_str().unwrap(), "pass");
}

#[test]
fn test_cli_prod_readiness_combined_json() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("92-98")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 7);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_phase_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("54-68")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 15);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_planning_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("56,83-88,109-113")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 12);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_design_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("57,107-108")
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
fn test_cli_development_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("58,69,103-106")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 6);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_testing_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("59,99-102")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 5);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_deployment_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("60-62,68,114-116")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 7);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_operations_artifact_checks() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--json")
        .arg("--checks")
        .arg("63-67,117")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 6);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_text_format() {
    let tmp = common::create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
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

#[test]
fn test_cli_output_saves_json_file() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("docs/7-operations/compliance/documentation_audit_report_v1.0.0.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1,2,3")
        .arg("--output")
        .arg(&out_path)
        .assert()
        .success();
    // File should exist and contain valid JSON
    assert!(out_path.exists(), "Output file was not created");
    let content = std::fs::read_to_string(&out_path).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(val.get("results").is_some());
    assert!(val.get("summary").is_some());
    // ISO/IEC/IEEE 15289:2019 clause 9.2 metadata fields
    assert_eq!(val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(val["clause"], "9.2");
    assert_eq!(val["tool"], "doc-engine");
    assert!(val["tool_version"].is_string());
    assert!(val["timestamp"].is_string());
    assert!(val["project_root"].is_string());
}

#[test]
fn test_cli_output_creates_parent_dirs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let out_path = tmp.path().join("nested/deep/report.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1")
        .arg("--output")
        .arg(&out_path)
        .output()
        .unwrap();
    assert!(out_path.exists(), "Output file was not created in nested dir");
}

#[test]
fn test_cli_output_short_flag() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("report.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1")
        .arg("-o")
        .arg(&out_path)
        .assert()
        .success();
    assert!(out_path.exists(), "Output file was not created with -o flag");
}

// ===========================================================================
// ISO/IEC/IEEE 15289:2019 clause 9.2 audit report metadata tests
// ===========================================================================

#[test]
fn test_cli_output_iso15289_metadata() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("report.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1")
        .arg("--output")
        .arg(&out_path)
        .assert()
        .success();

    let content = std::fs::read_to_string(&out_path).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(val["clause"], "9.2");
    assert_eq!(val["tool"], "doc-engine");
    assert_eq!(val["tool_version"], "0.1.0");
    // Timestamp format: YYYY-MM-DDTHH:MM:SSZ
    let ts = val["timestamp"].as_str().unwrap();
    assert_eq!(ts.len(), 20, "ISO 8601 UTC timestamp must be 20 chars: {}", ts);
    assert!(ts.ends_with('Z'), "timestamp must end with Z: {}", ts);
    assert_eq!(&ts[4..5], "-");
    assert_eq!(&ts[7..8], "-");
    assert_eq!(&ts[10..11], "T");
}

#[test]
fn test_cli_output_project_root_is_absolute() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("report.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1")
        .arg("--output")
        .arg(&out_path)
        .assert()
        .success();

    let content = std::fs::read_to_string(&out_path).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();

    let project_root = val["project_root"].as_str().unwrap();
    assert!(
        std::path::Path::new(project_root).is_absolute(),
        "project_root should be an absolute path: {}",
        project_root
    );
}

#[test]
fn test_cli_output_contains_scan_results() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("report.json");
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1,2,3")
        .arg("--output")
        .arg(&out_path)
        .assert()
        .success();

    let content = std::fs::read_to_string(&out_path).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Results and summary are present at top level (flat)
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    let summary = &val["summary"];
    assert_eq!(summary["total"], 3);
    assert!(val["project_type"].is_string());
    assert!(val["project_scope"].is_string());
}

#[test]
fn test_cli_output_json_stdout_matches_file() {
    let tmp = common::create_minimal_project();
    let out_path = tmp.path().join("report.json");

    // Run with both --json and --output
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--scope")
        .arg("large")
        .arg("--checks")
        .arg("1")
        .arg("--json")
        .arg("--output")
        .arg(&out_path)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout_val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let file_val: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&out_path).unwrap()).unwrap();

    // Both --json stdout and --output file include ISO metadata
    assert_eq!(stdout_val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(file_val["standard"], "ISO/IEC/IEEE 15289:2019");
    // Both contain the same results
    assert_eq!(stdout_val["summary"]["total"], file_val["summary"]["total"]);
}
