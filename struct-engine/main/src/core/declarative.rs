use std::fs;
use std::path::Path;

use regex::Regex;

use crate::api::traits::CheckRunner;
use crate::api::types::{RuleDef, RuleType, CheckId, CheckResult, ScanContext, Violation};
use crate::core::cargo_manifest::lookup_toml_key;

pub struct DeclarativeCheck {
    pub def: RuleDef,
}

impl CheckRunner for DeclarativeCheck {
    fn id(&self) -> CheckId {
        CheckId(self.def.id)
    }

    fn category(&self) -> &str {
        &self.def.category
    }

    fn description(&self) -> &str {
        &self.def.description
    }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        match &self.def.rule_type {
            RuleType::FileExists { path } => self.check_file_exists(ctx, path),
            RuleType::DirExists { path } => self.check_dir_exists(ctx, path),
            RuleType::DirNotExists { path, message } => self.check_dir_not_exists(ctx, path, message),
            RuleType::FileContentMatches { path, pattern } => self.check_file_content_matches(ctx, path, pattern),
            RuleType::FileContentNotMatches { path, pattern } => self.check_file_content_not_matches(ctx, path, pattern),
            RuleType::GlobContentMatches { glob, pattern } => self.check_glob_content_matches(ctx, glob, pattern),
            RuleType::GlobContentNotMatches { glob, pattern, exclude_pattern } => {
                self.check_glob_content_not_matches(ctx, glob, pattern, exclude_pattern.as_deref())
            }
            RuleType::GlobNamingMatches { glob, pattern } => self.check_glob_naming_matches(ctx, glob, pattern),
            RuleType::GlobNamingNotMatches { glob, pattern, exclude_paths } => {
                self.check_glob_naming_not_matches(ctx, glob, pattern, exclude_paths.as_deref())
            }
            RuleType::CargoKeyExists { key } => self.check_cargo_key_exists(ctx, key),
            RuleType::CargoKeyMatches { key, pattern } => self.check_cargo_key_matches(ctx, key, pattern),
            RuleType::Builtin { .. } => {
                CheckResult::Skip { reason: "Builtin rules should not use DeclarativeCheck".to_string() }
            }
        }
    }
}

impl DeclarativeCheck {
    fn make_violation(&self, path: Option<&Path>, message: &str) -> Violation {
        Violation {
            check_id: CheckId(self.def.id),
            path: path.map(|p| p.to_path_buf()),
            message: message.to_string(),
            severity: self.def.severity.clone(),
        }
    }

    fn check_file_exists(&self, ctx: &ScanContext, path: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_file() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' does not exist", path),
                )],
            }
        }
    }

    fn check_dir_exists(&self, ctx: &ScanContext, path: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_dir() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("Directory '{}' does not exist", path),
                )],
            }
        }
    }

    fn check_dir_not_exists(&self, ctx: &ScanContext, path: &str, message: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_dir() {
            CheckResult::Fail {
                violations: vec![self.make_violation(Some(Path::new(path)), message)],
            }
        } else {
            CheckResult::Pass
        }
    }

    fn check_file_content_matches(&self, ctx: &ScanContext, path: &str, pattern: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if !full.exists() {
            return CheckResult::Skip {
                reason: format!("File '{}' does not exist", path),
            };
        }

        let content = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read '{}': {}", path, e),
                };
            }
        };

        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        if re.is_match(&content) {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' does not match pattern '{}'", path, pattern),
                )],
            }
        }
    }

    fn check_file_content_not_matches(&self, ctx: &ScanContext, path: &str, pattern: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if !full.exists() {
            return CheckResult::Pass;
        }

        let content = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read '{}': {}", path, e),
                };
            }
        };

        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        if re.is_match(&content) {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' contains forbidden pattern '{}'", path, pattern),
                )],
            }
        } else {
            CheckResult::Pass
        }
    }

    fn check_glob_content_matches(&self, ctx: &ScanContext, glob: &str, pattern: &str) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let content_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files().iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !content_re.is_match(&content) {
                violations.push(self.make_violation(
                    Some(file),
                    &format!("File does not match pattern '{}'", pattern),
                ));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_content_not_matches(
        &self,
        ctx: &ScanContext,
        glob: &str,
        pattern: &str,
        exclude_pattern: Option<&str>,
    ) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let content_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let exclude_re = match exclude_pattern {
            Some(ep) => match Regex::new(ep) {
                Ok(r) => Some(r),
                Err(e) => {
                    return CheckResult::Skip {
                        reason: format!("Invalid exclude regex '{}': {}", ep, e),
                    };
                }
            },
            None => None,
        };

        let matching_files: Vec<_> = ctx.files().iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for line in content.lines() {
                if content_re.is_match(line) {
                    if let Some(ref excl) = exclude_re {
                        if excl.is_match(line) {
                            continue;
                        }
                    }
                    violations.push(self.make_violation(
                        Some(file),
                        &format!("File contains forbidden pattern '{}'", pattern),
                    ));
                    break;
                }
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_naming_matches(&self, ctx: &ScanContext, glob: &str, pattern: &str) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let name_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files().iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if !name_re.is_match(&filename) {
                violations.push(self.make_violation(
                    Some(file),
                    &format!("Filename '{}' does not match pattern '{}'", filename, pattern),
                ));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_naming_not_matches(
        &self,
        ctx: &ScanContext,
        glob: &str,
        pattern: &str,
        exclude_paths: Option<&[String]>,
    ) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let name_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files().iter()
            .filter(|f| {
                let path_str = f.to_string_lossy();
                if !glob_re.is_match(&path_str) {
                    return false;
                }
                if let Some(excl) = exclude_paths {
                    for prefix in excl {
                        if path_str.starts_with(prefix.as_str()) {
                            return false;
                        }
                    }
                }
                true
            })
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if name_re.is_match(&filename) {
                let msg = self.def.rule_type.custom_message()
                    .unwrap_or_else(|| format!("Filename '{}' matches forbidden pattern '{}'", filename, pattern));
                violations.push(self.make_violation(Some(file), &msg));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_cargo_key_exists(&self, ctx: &ScanContext, key: &str) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => {
                return CheckResult::Skip {
                    reason: "No Cargo.toml found".to_string(),
                };
            }
        };

        let raw = match &manifest.raw {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: "Cargo.toml not parsed".to_string(),
                };
            }
        };

        if lookup_toml_key(raw, key).is_some() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new("Cargo.toml")),
                    &format!("Key '{}' not found in Cargo.toml", key),
                )],
            }
        }
    }

    fn check_cargo_key_matches(&self, ctx: &ScanContext, key: &str, pattern: &str) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => {
                return CheckResult::Skip {
                    reason: "No Cargo.toml found".to_string(),
                };
            }
        };

        let raw = match &manifest.raw {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: "Cargo.toml not parsed".to_string(),
                };
            }
        };

        let value = match lookup_toml_key(raw, key) {
            Some(v) => v,
            None => {
                return CheckResult::Skip {
                    reason: format!("Key '{}' not found in Cargo.toml", key),
                };
            }
        };

        let value_str = match value.as_str() {
            Some(s) => s.to_string(),
            None => format!("{}", value),
        };

        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        if re.is_match(&value_str) {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new("Cargo.toml")),
                    &format!("Key '{}' value '{}' does not match pattern '{}'", key, value_str, pattern),
                )],
            }
        }
    }
}

impl RuleType {
    fn custom_message(&self) -> Option<String> {
        match self {
            RuleType::DirNotExists { message, .. } => Some(message.clone()),
            _ => None,
        }
    }
}

/// Convert a glob pattern to a regex. Handles *, **, and ?.
/// Returns None if the resulting regex is invalid.
#[allow(clippy::manual_pattern_char_comparison)]
pub fn glob_to_regex(glob: &str) -> Option<Regex> {
    let mut regex = String::with_capacity(glob.len() * 2);
    regex.push('^');

    let chars: Vec<char> = glob.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    if i + 2 < chars.len() && chars[i + 2] == '/' {
                        regex.push_str("(?:.*/)?");
                        i += 3;
                    } else {
                        regex.push_str(".*");
                        i += 2;
                    }
                } else {
                    regex.push_str("[^/]*");
                    i += 1;
                }
            }
            '?' => {
                regex.push_str("[^/]");
                i += 1;
            }
            '.' | '+' | '(' | ')' | '{' | '}' | '[' | ']' | '^' | '$' | '|' | '\\' => {
                regex.push('\\');
                regex.push(chars[i]);
                i += 1;
            }
            c => {
                regex.push(c);
                i += 1;
            }
        }
    }

    regex.push('$');
    Regex::new(&regex).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{CargoManifest, Severity};
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_rule_def(id: u8, rule_type: RuleType) -> RuleDef {
        RuleDef {
            id,
            category: "test".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type,
            project_kind: None,
        }
    }

    fn make_ctx(root: &std::path::Path, files: Vec<std::path::PathBuf>) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            file_index: crate::api::types::FileIndex::from_files(files),
            file_contents: HashMap::new(),
            project_kind: crate::api::types::ProjectKind::Library,
            cargo_manifest: None,
        }
    }

    fn make_ctx_with_manifest(root: &std::path::Path, manifest: CargoManifest) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            file_index: crate::api::types::FileIndex::from_files(vec![]),
            file_contents: HashMap::new(),
            project_kind: crate::api::types::ProjectKind::Library,
            cargo_manifest: Some(manifest),
        }
    }

    // --- glob_to_regex tests ---

    #[test]
    fn test_glob_literal() {
        let re = glob_to_regex("Cargo.toml").unwrap();
        assert!(re.is_match("Cargo.toml"));
        assert!(!re.is_match("cargo.toml"));
    }

    #[test]
    fn test_glob_star() {
        let re = glob_to_regex("*.rs").unwrap();
        assert!(re.is_match("main.rs"));
        assert!(!re.is_match("src/main.rs"));
    }

    #[test]
    fn test_glob_double_star() {
        let re = glob_to_regex("**/*.rs").unwrap();
        assert!(re.is_match("src/main.rs"));
        assert!(re.is_match("src/sub/file.rs"));
    }

    // --- FileExists tests ---

    #[test]
    fn test_file_exists_pass() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        let def = make_rule_def(1, RuleType::FileExists { path: "Cargo.toml".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_file_exists_fail() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(1, RuleType::FileExists { path: "Cargo.toml".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- DirExists tests ---

    #[test]
    fn test_dir_exists_pass() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir(tmp.path().join("src")).unwrap();
        let def = make_rule_def(1, RuleType::DirExists { path: "src".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_dir_exists_fail() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(1, RuleType::DirExists { path: "src".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- DirNotExists tests ---

    #[test]
    fn test_dir_not_exists_pass() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(1, RuleType::DirNotExists {
            path: "src/src".to_string(),
            message: "Nested src/ detected".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_dir_not_exists_fail() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("src/src")).unwrap();
        let def = make_rule_def(1, RuleType::DirNotExists {
            path: "src/src".to_string(),
            message: "Nested src/ detected".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        if let CheckResult::Fail { violations } = check.run(&ctx) {
            assert!(violations[0].message.contains("Nested src/"));
        } else {
            panic!("expected Fail");
        }
    }

    // --- CargoKeyExists tests ---

    #[test]
    fn test_cargo_key_exists_pass() {
        let tmp = TempDir::new().unwrap();
        let raw: toml::Value = r#"
[package]
name = "test"
"#.parse().unwrap();
        let manifest = CargoManifest {
            raw: Some(raw),
            package_name: Some("test".to_string()),
            has_lib: false,
            lib_path: None,
            bins: vec![],
            tests: vec![],
            benches: vec![],
            examples: vec![],
            has_workspace: false,
            edition: None,
            workspace_members: vec![],
        };
        let def = make_rule_def(9, RuleType::CargoKeyExists { key: "package.name".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx_with_manifest(tmp.path(), manifest);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_cargo_key_exists_fail() {
        let tmp = TempDir::new().unwrap();
        let raw: toml::Value = r#"
[package]
name = "test"
"#.parse().unwrap();
        let manifest = CargoManifest {
            raw: Some(raw),
            package_name: Some("test".to_string()),
            has_lib: false,
            lib_path: None,
            bins: vec![],
            tests: vec![],
            benches: vec![],
            examples: vec![],
            has_workspace: false,
            edition: None,
            workspace_members: vec![],
        };
        let def = make_rule_def(12, RuleType::CargoKeyExists { key: "package.description".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx_with_manifest(tmp.path(), manifest);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_cargo_key_exists_no_manifest() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(9, RuleType::CargoKeyExists { key: "package.name".to_string() });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- CargoKeyMatches tests ---

    #[test]
    fn test_cargo_key_matches_pass() {
        let tmp = TempDir::new().unwrap();
        let raw: toml::Value = r#"
[package]
name = "my_package"
"#.parse().unwrap();
        let manifest = CargoManifest {
            raw: Some(raw),
            package_name: Some("my_package".to_string()),
            has_lib: false,
            lib_path: None,
            bins: vec![],
            tests: vec![],
            benches: vec![],
            examples: vec![],
            has_workspace: false,
            edition: None,
            workspace_members: vec![],
        };
        let def = make_rule_def(27, RuleType::CargoKeyMatches {
            key: "package.name".to_string(),
            pattern: r"^[a-z][a-z0-9_]*$".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx_with_manifest(tmp.path(), manifest);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_cargo_key_matches_fail() {
        let tmp = TempDir::new().unwrap();
        let raw: toml::Value = r#"
[package]
name = "MyPackage"
"#.parse().unwrap();
        let manifest = CargoManifest {
            raw: Some(raw),
            package_name: Some("MyPackage".to_string()),
            has_lib: false,
            lib_path: None,
            bins: vec![],
            tests: vec![],
            benches: vec![],
            examples: vec![],
            has_workspace: false,
            edition: None,
            workspace_members: vec![],
        };
        let def = make_rule_def(27, RuleType::CargoKeyMatches {
            key: "package.name".to_string(),
            pattern: r"^[a-z][a-z0-9_]*$".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx_with_manifest(tmp.path(), manifest);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- FileContentMatches tests ---

    #[test]
    fn test_file_content_matches_pass() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join(".gitignore"), "target/\n").unwrap();
        let def = make_rule_def(43, RuleType::FileContentMatches {
            path: ".gitignore".to_string(),
            pattern: r"target/".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_file_content_matches_fail() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join(".gitignore"), "*.log\n").unwrap();
        let def = make_rule_def(43, RuleType::FileContentMatches {
            path: ".gitignore".to_string(),
            pattern: r"target/".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- GlobNamingMatches tests ---

    #[test]
    fn test_glob_naming_matches_pass() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(29, RuleType::GlobNamingMatches {
            glob: "**/*.rs".to_string(),
            pattern: r"^[a-z][a-z0-9_]*\.rs$".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec!["src/main.rs".into(), "src/lib.rs".into()]);
        assert!(matches!(check.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_glob_naming_matches_fail() {
        let tmp = TempDir::new().unwrap();
        let def = make_rule_def(29, RuleType::GlobNamingMatches {
            glob: "**/*.rs".to_string(),
            pattern: r"^[a-z][a-z0-9_]*\.rs$".to_string(),
        });
        let check = DeclarativeCheck { def };
        let ctx = make_ctx(tmp.path(), vec!["src/BadName.rs".into()]);
        assert!(matches!(check.run(&ctx), CheckResult::Fail { .. }));
    }
}
