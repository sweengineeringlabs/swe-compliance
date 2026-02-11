use std::fs;
use std::path::Path;
use assert_cmd::Command;
use struct_engine::default_rule_count;
use tempfile::TempDir;

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

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("struct-engine").unwrap()
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
    let tmp = create_minimal_project();
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
    let tmp = create_minimal_project();
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
    let tmp = create_minimal_project();
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
fn test_cli_kind_library() {
    let tmp = create_minimal_project();
    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--kind")
        .arg("library")
        .arg("--checks")
        .arg("1")
        .assert()
        .success();
}

#[test]
fn test_cli_total_checks() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(val["summary"]["total"].as_u64().unwrap(), default_rule_count() as u64);
}

#[test]
fn test_cli_custom_rules() {
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

    cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--rules")
        .arg(&rules_path)
        .assert()
        .success();
}

#[test]
fn test_cli_text_format() {
    let tmp = create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--checks")
        .arg("1-3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("[PASS]") || stdout.contains("[FAIL]") || stdout.contains("[SKIP]")
    );
}

#[test]
fn test_cli_metadata_checks_json() {
    let tmp = create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("9-13")
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
fn test_cli_hygiene_checks_json() {
    let tmp = create_minimal_project();
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--checks")
        .arg("43,44")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}

#[test]
fn test_cli_rustboot_rules() {
    let tmp = create_rustboot_project();
    let rules_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/rules-rustboot.toml");
    let output = cmd()
        .arg("scan")
        .arg(tmp.path())
        .arg("--json")
        .arg("--kind")
        .arg("library")
        .arg("--rules")
        .arg(&rules_path)
        .arg("--checks")
        .arg("1,28")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let results = val["results"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    for r in results {
        assert_eq!(
            r["result"]["status"].as_str().unwrap(), "pass",
            "Check {} failed: {:?}", r["id"], r["result"]
        );
    }
}
