mod common;
mod scaffold_fixtures;

use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;
use scaffold_fixtures::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("doc-engine").unwrap()
}

// === TC-001: FR-SCA-001: Scaffold command =================================

#[test]
fn e2e_scaffold_help() {
    cmd()
        .arg("scaffold")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("SRS"))
        .stdout(predicate::str::contains("--force"))
        .stdout(predicate::str::contains("--output"));
}

#[test]
fn e2e_scaffold_basic() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("cli_output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Scaffold complete"))
        .stdout(predicate::str::contains("2 domains"))
        .stdout(predicate::str::contains("3 requirements"))
        .stdout(predicate::str::contains("22 files created"))
        .stdout(predicate::str::contains("0 skipped"));
}

#[test]
fn e2e_scaffold_shows_created_files() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");

    let output = cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Created files shown with + prefix
    assert!(stdout.contains("+ docs/1-requirements/rule_loading/rule_loading.spec.yaml"));
    assert!(stdout.contains("+ docs/3-design/rule_loading/rule_loading.arch.yaml"));
    assert!(stdout.contains("+ docs/5-testing/rule_loading/rule_loading.test.yaml"));
    assert!(stdout.contains("+ docs/6-deployment/rule_loading/rule_loading.deploy.yaml"));
    assert!(stdout.contains("+ docs/1-requirements/brd.spec.yaml"));
    assert!(stdout.contains("+ docs/1-requirements/brd.spec"));
}

#[test]
fn e2e_scaffold_shows_skipped_files() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");

    // First run
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    // Second run - should show skipped with ~
    let output = cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("~ docs/1-requirements/rule_loading/rule_loading.spec.yaml"));
    assert!(stdout.contains("22 skipped"));
    assert!(stdout.contains("0 files created"));
}

#[test]
fn e2e_scaffold_force_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");

    // First run
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    // Second run with --force
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("22 files created"))
        .stdout(predicate::str::contains("0 skipped"));
}

#[test]
fn e2e_scaffold_short_output_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("short_out");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("-o")
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Scaffold complete"));

    // Verify files created
    assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml").exists());
}

#[test]
fn e2e_scaffold_missing_srs() {
    cmd()
        .arg("scaffold")
        .arg("/nonexistent/srs.md")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("cannot resolve SRS path"));
}

#[test]
fn e2e_scaffold_empty_srs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("empty.md");
    fs::write(&srs_path, "# Empty SRS\n\nNo requirements.\n").unwrap();

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(tmp.path().join("out"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("no domains"));
}

#[test]
fn e2e_scaffold_large_srs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("5 domains"))
        .stdout(predicate::str::contains("10 requirements"))
        .stdout(predicate::str::contains("52 files created"));
}

// === TC-004: FR-SCA-004: Manual test execution plan =======================

#[test]
fn e2e_scaffold_manual_exec_steps_populated() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, STEPS_FIXTURE_SRS).unwrap();

    cmd()
        .args(["scaffold", srs_path.to_str().unwrap()])
        .args(["--output", tmp.path().join("output").to_str().unwrap()])
        .assert()
        .success();

    let manual = fs::read_to_string(
        tmp.path().join("output/docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // Verify CLI-produced output has auto-populated steps
    assert!(manual.contains("Run `doc-engine scan <PATH>`"));
    assert!(manual.contains("Execute `doc-engine --help` and observe output"));
    // Demonstration without backtick command falls back to _TODO_ (acceptance is never used for Steps)
    assert!(manual.contains("_TODO_"));
}

// === TC-007: FR-SCA-007: Output directory =================================

#[test]
fn e2e_scaffold_creates_nested_output_dirs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("deeply/nested/output/path");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
}

// === TC-008: FR-SCA-008: Phase and type filters ===========================

#[test]
fn e2e_scaffold_phase_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("testing")
        .assert()
        .success()
        .stdout(predicate::str::contains("8 files created"))
        .stdout(predicate::str::contains("0 skipped"));

    assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml").exists());
    assert!(!output_dir.join("docs/1-requirements/rule_loading").exists());
}

#[test]
fn e2e_scaffold_phase_comma_separated() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("requirements,design")
        .assert()
        .success()
        .stdout(predicate::str::contains("10 files created"));

    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml").exists());
    assert!(!output_dir.join("docs/5-testing/rule_loading").exists());
}

#[test]
fn e2e_scaffold_phase_invalid() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--phase")
        .arg("invalid")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("unknown phase 'invalid'"));
}

#[test]
fn e2e_scaffold_phase_in_help() {
    cmd()
        .arg("scaffold")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--phase"));
}

#[test]
fn e2e_scaffold_phase_case_insensitive() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("Testing")
        .assert()
        .success()
        .stdout(predicate::str::contains("8 files created"));

    assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml").exists());
}

#[test]
fn e2e_scaffold_phase_three_phases() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("requirements,design,deployment")
        .assert()
        .success()
        // 2 domains × (2 req + 2 design + 2 deploy) + 2 BRD = 14
        .stdout(predicate::str::contains("14 files created"));

    assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml").exists());
    assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml").exists());
    assert!(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml").exists());
    assert!(!output_dir.join("docs/5-testing/rule_loading").exists());
}

#[test]
fn e2e_scaffold_phase_all_four_explicit() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("requirements,design,testing,deployment")
        .assert()
        .success()
        .stdout(predicate::str::contains("22 files created"));
}

#[test]
fn e2e_scaffold_phase_with_force() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // First run: all phases
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    // Second run: design only + force
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("design")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("4 files created"))
        .stdout(predicate::str::contains("0 skipped"));
}

#[test]
fn e2e_scaffold_phase_partial_invalid() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    // One valid, one invalid
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--phase")
        .arg("testing,bogus")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("unknown phase 'bogus'"));
}

#[test]
fn e2e_scaffold_phase_with_spaces() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // Spaces around comma-separated values
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("design , deployment")
        .assert()
        .success()
        .stdout(predicate::str::contains("8 files created"));
}

#[test]
fn e2e_scaffold_phase_large_srs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("deployment")
        .assert()
        .success()
        .stdout(predicate::str::contains("5 domains"))
        .stdout(predicate::str::contains("10 requirements"))
        // 5 domains × 2 deploy files = 10
        .stdout(predicate::str::contains("10 files created"));
}

#[test]
fn e2e_scaffold_type_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .args(["scaffold", srs_path.to_str().unwrap()])
        .args(["--output", output_dir.to_str().unwrap()])
        .args(["--type", "exec"])
        .assert()
        .success();

    // Only .exec files should be created
    let entries: Vec<_> = walkdir::WalkDir::new(&output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    assert_eq!(entries.len(), 4, "exec filter should produce 4 files");
    for entry in &entries {
        let name = entry.file_name().to_string_lossy();
        assert!(
            name.ends_with(".manual.exec") || name.ends_with(".auto.exec"),
            "Unexpected file: {}", name,
        );
    }
}

#[test]
fn e2e_scaffold_type_flag_combined_with_phase() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    cmd()
        .args(["scaffold", srs_path.to_str().unwrap()])
        .args(["--output", output_dir.to_str().unwrap()])
        .args(["--phase", "testing"])
        .args(["--type", "exec"])
        .assert()
        .success();

    let entries: Vec<_> = walkdir::WalkDir::new(&output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    assert_eq!(entries.len(), 4, "testing+exec should produce 4 files");
}

// === TC-009: FR-SCA-009: Scaffold status report ===========================

#[test]
fn e2e_scaffold_report_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success()
        .stderr(predicate::str::contains("Report saved to"));

    assert!(report_path.exists(), "Report file should be created");
    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Report should be valid JSON");
    assert!(parsed.is_object());
}

#[test]
fn e2e_scaffold_report_content() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["domain_count"], 2);
    assert_eq!(parsed["requirement_count"], 3);
    assert!(parsed["created"].is_array());
    assert!(parsed["skipped"].is_array());
    assert_eq!(parsed["created"].as_array().unwrap().len(), 22);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 0);
    // ISO 15289 metadata fields present
    assert_eq!(parsed["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(parsed["clause"], "9");
    assert_eq!(parsed["tool"], "doc-engine");
    assert!(parsed["tool_version"].is_string());
    assert!(parsed["timestamp"].is_string());
    assert!(parsed["srs_source"].is_string());
    assert!(parsed["phases"].is_array());
    assert!(parsed["force"].is_boolean());
}

#[test]
fn e2e_scaffold_report_creates_parent_dirs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("nested/deep/report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    assert!(report_path.exists(), "Report file should be created in nested dir");
    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed["domain_count"], 2);
}

#[test]
fn e2e_scaffold_report_in_help() {
    cmd()
        .arg("scaffold")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--report"));
}

#[test]
fn e2e_scaffold_report_skipped_files() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    // First run creates all files
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    // Second run without --force: all skipped
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["created"].as_array().unwrap().len(), 0);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 22);
    assert_eq!(parsed["domain_count"], 2);
    assert_eq!(parsed["requirement_count"], 3);
}

#[test]
fn e2e_scaffold_report_with_force() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    // First run
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success();

    // Second run with --force: all re-created
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--force")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["created"].as_array().unwrap().len(), 22);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 0);
}

#[test]
fn e2e_scaffold_report_with_phase_filter() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("testing")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // 2 domains × 4 testing files = 8
    assert_eq!(parsed["created"].as_array().unwrap().len(), 8);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 0);
    assert_eq!(parsed["domain_count"], 2);
    assert_eq!(parsed["requirement_count"], 3);
}

#[test]
fn e2e_scaffold_report_with_multiple_phases() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("requirements,design")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // 2 domains × (2 req + 2 design) + 2 BRD = 10
    assert_eq!(parsed["created"].as_array().unwrap().len(), 10);
}

#[test]
fn e2e_scaffold_report_large_srs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["domain_count"], 5);
    assert_eq!(parsed["requirement_count"], 10);
    // 5 domains × 10 files + 2 BRD = 52
    assert_eq!(parsed["created"].as_array().unwrap().len(), 52);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 0);
}

#[test]
fn e2e_scaffold_report_created_paths_are_strings() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    let created = parsed["created"].as_array().unwrap();
    for entry in created {
        assert!(entry.is_string(), "Each created entry should be a string path");
    }
    // Spot-check known paths
    let paths: Vec<&str> = created.iter().map(|e| e.as_str().unwrap()).collect();
    assert!(paths.iter().any(|p| p.contains("rule_loading") && p.ends_with(".spec.yaml")));
    assert!(paths.iter().any(|p| p.contains("file_discovery") && p.ends_with(".test.yaml")));
    assert!(paths.iter().any(|p| p.contains("brd.spec.yaml")));
    assert!(paths.iter().any(|p| p.contains("brd.spec") && !p.contains(".yaml")));
}

#[test]
fn e2e_scaffold_report_matches_stdout_counts() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    let output = cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // stdout says "2 domains, 3 requirements, 22 files created, 0 skipped"
    let domain_count = parsed["domain_count"].as_u64().unwrap();
    let req_count = parsed["requirement_count"].as_u64().unwrap();
    let created_count = parsed["created"].as_array().unwrap().len();
    let skipped_count = parsed["skipped"].as_array().unwrap().len();

    assert!(stdout.contains(&format!("{} domains", domain_count)));
    assert!(stdout.contains(&format!("{} requirements", req_count)));
    assert!(stdout.contains(&format!("{} files created", created_count)));
    assert!(stdout.contains(&format!("{} skipped", skipped_count)));
}

#[test]
fn e2e_scaffold_report_overwritten_on_rerun() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    // First run: all created
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content1 = fs::read_to_string(&report_path).unwrap();
    let parsed1: serde_json::Value = serde_json::from_str(&content1).unwrap();
    assert_eq!(parsed1["created"].as_array().unwrap().len(), 22);

    // Second run: all skipped, same report path
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content2 = fs::read_to_string(&report_path).unwrap();
    let parsed2: serde_json::Value = serde_json::from_str(&content2).unwrap();
    assert_eq!(parsed2["created"].as_array().unwrap().len(), 0);
    assert_eq!(parsed2["skipped"].as_array().unwrap().len(), 22);
}

#[test]
fn e2e_scaffold_report_phase_filter_skipped_mix() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    // Pre-create one testing file
    let test_dir = output_dir.join("docs/5-testing/rule_loading");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(test_dir.join("rule_loading.test.yaml"), "existing").unwrap();

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("testing")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // 8 total testing files: 1 skipped, 7 created
    assert_eq!(parsed["created"].as_array().unwrap().len(), 7);
    assert_eq!(parsed["skipped"].as_array().unwrap().len(), 1);

    let skipped: Vec<&str> = parsed["skipped"].as_array().unwrap()
        .iter().map(|v| v.as_str().unwrap()).collect();
    assert!(skipped.iter().any(|p| p.contains("rule_loading.test.yaml")));
}

#[test]
fn e2e_scaffold_no_report_without_flag() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    // Run without --report flag
    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .assert()
        .success()
        .stderr(predicate::str::contains("Report saved to").not());

    assert!(!report_path.exists(), "Report file should not be created without --report");
}

#[test]
fn e2e_scaffold_report_iso15289_metadata() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(parsed["clause"], "9");
    assert_eq!(parsed["tool"], "doc-engine");
    assert_eq!(parsed["tool_version"], "0.1.0");
    // Timestamp format: YYYY-MM-DDTHH:MM:SSZ
    let ts = parsed["timestamp"].as_str().unwrap();
    assert_eq!(ts.len(), 20, "ISO 8601 UTC timestamp must be 20 chars: {}", ts);
    assert!(ts.ends_with('Z'), "timestamp must end with Z: {}", ts);
    assert_eq!(&ts[4..5], "-");
    assert_eq!(&ts[7..8], "-");
    assert_eq!(&ts[10..11], "T");
}

#[test]
fn e2e_scaffold_report_srs_source_is_absolute() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    let srs_source = parsed["srs_source"].as_str().unwrap();
    assert!(
        std::path::Path::new(srs_source).is_absolute(),
        "srs_source should be an absolute path: {}",
        srs_source
    );
}

#[test]
fn e2e_scaffold_report_phases_empty_when_no_filter() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    let phases = parsed["phases"].as_array().unwrap();
    assert!(phases.is_empty(), "phases should be empty when no --phase filter: {:?}", phases);
}

#[test]
fn e2e_scaffold_report_phases_populated_with_filter() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--phase")
        .arg("testing")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    let phases: Vec<&str> = parsed["phases"].as_array().unwrap()
        .iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(phases, vec!["testing"]);
}

#[test]
fn e2e_scaffold_report_force_true() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--force")
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["force"], true);
}

#[test]
fn e2e_scaffold_report_force_false() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_dir = tmp.path().join("output");
    let report_path = tmp.path().join("report.json");

    cmd()
        .arg("scaffold")
        .arg(&srs_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--report")
        .arg(&report_path)
        .assert()
        .success();

    let content = fs::read_to_string(&report_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["force"], false);
}
