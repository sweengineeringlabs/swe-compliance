use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Write a file at a relative path under root, creating parent dirs as needed.
pub fn write_file(root: &Path, relative: &str, content: &str) {
    let full = root.join(relative);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&full, content).unwrap();
}

/// Create a minimal standard Rust project that passes most checks.
pub fn create_minimal_project() -> TempDir {
    let tmp = tempfile::Builder::new().prefix("test_").tempdir().unwrap();
    let root = tmp.path();

    // Cargo.toml with full metadata
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
path = "src/lib.rs"
"#);

    // Source files
    write_file(root, "src/lib.rs", r#"pub mod utils;

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

    write_file(root, "src/utils.rs", r#"pub fn helper() -> String {
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

    // Tests directory
    write_file(root, "tests/integration.rs", r#"#[test]
fn test_integration() {
    assert!(true);
}
"#);

    // Root files
    write_file(root, "README.md", "# Test Project\n\nA test project.\n");
    write_file(root, "CHANGELOG.md", "# Changelog\n\n## 0.1.0\n- Initial release\n");
    write_file(root, ".gitignore", "target/\n*.swp\n");

    tmp
}

/// Create a rustboot-style project structure.
pub fn create_rustboot_project() -> TempDir {
    let tmp = tempfile::Builder::new().prefix("test_rb_").tempdir().unwrap();
    let root = tmp.path();

    // Cargo.toml with rustboot naming
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
name = "api_int_test"
path = "tests/src/api_int_test.rs"
"#);

    // Source files under main/src/
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

    // Tests under tests/src/
    write_file(root, "tests/src/api_int_test.rs", r#"#[test]
fn test_api_integration_happy() {
    assert!(true);
}
"#);

    // Root files
    write_file(root, "README.md", "# Rustboot Example\n\nA rustboot test project.\n");
    write_file(root, "CHANGELOG.md", "# Changelog\n\n## 0.1.0\n- Initial release\n");
    write_file(root, ".gitignore", "target/\n*.swp\n");

    tmp
}
