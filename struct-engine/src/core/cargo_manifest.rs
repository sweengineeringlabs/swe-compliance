use std::path::Path;

use crate::api::types::{CargoManifest, BinTarget, TestTarget, BenchTarget, ExampleTarget};
use crate::spi::types::ScanError;

/// Parse a Cargo.toml file into a CargoManifest.
pub fn parse_cargo_toml(root: &Path) -> Result<Option<CargoManifest>, ScanError> {
    let cargo_path = root.join("Cargo.toml");
    if !cargo_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&cargo_path)
        .map_err(|e| ScanError::Config(format!("Cannot read Cargo.toml: {}", e)))?;

    let raw: toml::Value = content.parse()
        .map_err(|e| ScanError::Config(format!("Cannot parse Cargo.toml: {}", e)))?;

    let package = raw.get("package");

    let package_name = package
        .and_then(|p| p.get("name"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let edition = package
        .and_then(|p| p.get("edition"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let has_lib = raw.get("lib").is_some();
    let lib_path = raw.get("lib")
        .and_then(|l| l.get("path"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let bins = raw.get("bin")
        .and_then(|b| b.as_array())
        .map(|arr| {
            arr.iter().map(|b| BinTarget {
                name: b.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                path: b.get("path").and_then(|v| v.as_str()).map(String::from),
            }).collect()
        })
        .unwrap_or_default();

    let tests = raw.get("test")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter().map(|t| TestTarget {
                name: t.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                path: t.get("path").and_then(|v| v.as_str()).map(String::from),
            }).collect()
        })
        .unwrap_or_default();

    let benches = raw.get("bench")
        .and_then(|b| b.as_array())
        .map(|arr| {
            arr.iter().map(|b| BenchTarget {
                name: b.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                harness: b.get("harness").and_then(|v| v.as_bool()),
            }).collect()
        })
        .unwrap_or_default();

    let examples = raw.get("example")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter().map(|e| ExampleTarget {
                name: e.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                path: e.get("path").and_then(|v| v.as_str()).map(String::from),
            }).collect()
        })
        .unwrap_or_default();

    let has_workspace = raw.get("workspace").is_some();

    Ok(Some(CargoManifest {
        raw: Some(raw),
        package_name,
        has_lib,
        lib_path,
        bins,
        tests,
        benches,
        examples,
        has_workspace,
        edition,
    }))
}

/// Look up a dotted key path in a TOML Value (e.g. "package.name").
pub fn lookup_toml_key<'a>(value: &'a toml::Value, key: &str) -> Option<&'a toml::Value> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = value;
    for part in parts {
        current = current.get(part)?;
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_minimal_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"
"#).unwrap();

        let manifest = parse_cargo_toml(tmp.path()).unwrap().unwrap();
        assert_eq!(manifest.package_name.as_deref(), Some("test-pkg"));
        assert_eq!(manifest.edition.as_deref(), Some("2021"));
        assert!(!manifest.has_lib);
        assert!(!manifest.has_workspace);
        assert!(manifest.bins.is_empty());
    }

    #[test]
    fn test_parse_lib_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "mylib"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#).unwrap();

        let manifest = parse_cargo_toml(tmp.path()).unwrap().unwrap();
        assert!(manifest.has_lib);
        assert_eq!(manifest.lib_path.as_deref(), Some("src/lib.rs"));
    }

    #[test]
    fn test_parse_bin_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "mybin"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mybin"
path = "src/main.rs"
"#).unwrap();

        let manifest = parse_cargo_toml(tmp.path()).unwrap().unwrap();
        assert_eq!(manifest.bins.len(), 1);
        assert_eq!(manifest.bins[0].name, "mybin");
        assert_eq!(manifest.bins[0].path.as_deref(), Some("src/main.rs"));
    }

    #[test]
    fn test_parse_workspace_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[workspace]
members = ["crate-a", "crate-b"]
"#).unwrap();

        let manifest = parse_cargo_toml(tmp.path()).unwrap().unwrap();
        assert!(manifest.has_workspace);
        assert!(manifest.package_name.is_none());
    }

    #[test]
    fn test_parse_no_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        let result = parse_cargo_toml(tmp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_with_tests_and_benches() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"

[[test]]
name = "integration"
path = "tests/integration.rs"

[[bench]]
name = "bench1"
harness = false
"#).unwrap();

        let manifest = parse_cargo_toml(tmp.path()).unwrap().unwrap();
        assert_eq!(manifest.tests.len(), 1);
        assert_eq!(manifest.tests[0].name, "integration");
        assert_eq!(manifest.benches.len(), 1);
        assert_eq!(manifest.benches[0].harness, Some(false));
    }

    #[test]
    fn test_lookup_toml_key() {
        let val: toml::Value = r#"
[package]
name = "foo"
version = "1.0"
"#.parse().unwrap();

        assert_eq!(
            lookup_toml_key(&val, "package.name").and_then(|v| v.as_str()),
            Some("foo")
        );
        assert!(lookup_toml_key(&val, "package.nonexistent").is_none());
        assert!(lookup_toml_key(&val, "nonexistent.key").is_none());
    }
}
