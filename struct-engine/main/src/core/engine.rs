use std::collections::HashMap;
use std::path::Path;

use crate::api::traits::{ComplianceEngine, FileScanner};
use crate::api::types::{ScanConfig, ScanReport, ScanSummary, CheckEntry, CheckResult, ProjectKind, ScanContext, ScanError};
use super::cargo_manifest;
use super::rules::{self, DEFAULT_RULES};
use super::scanner::FileSystemScanner;

/// Detect project kind from Cargo.toml content.
///
/// Reads `Cargo.toml` at the project root and determines the project kind
/// from `[workspace]`, `[lib]`, and `[[bin]]` sections.
pub fn detect_project_kind(root: &Path) -> ProjectKind {
    let manifest = match cargo_manifest::parse_cargo_toml(root) {
        Ok(Some(m)) => m,
        _ => return ProjectKind::Library, // default fallback
    };

    if manifest.has_workspace {
        return ProjectKind::Workspace;
    }

    match (manifest.has_lib, !manifest.bins.is_empty()) {
        (true, true) => ProjectKind::Both,
        (true, false) => ProjectKind::Library,
        (false, true) => ProjectKind::Binary,
        (false, false) => {
            // No explicit targets: Cargo defaults to library if src/lib.rs exists,
            // binary if src/main.rs exists
            let has_lib = root.join("src/lib.rs").exists() || root.join("main/src/lib.rs").exists();
            let has_bin = root.join("src/main.rs").exists() || root.join("main/src/main.rs").exists();
            match (has_lib, has_bin) {
                (true, true) => ProjectKind::Both,
                (true, false) => ProjectKind::Library,
                (false, true) => ProjectKind::Binary,
                (false, false) => ProjectKind::Library,
            }
        }
    }
}

/// Struct-engine compliance engine.
pub struct StructComplianceEngine;

impl ComplianceEngine for StructComplianceEngine {
    fn scan(&self, root: &Path) -> Result<ScanReport, ScanError> {
        self.scan_with_config(root, &ScanConfig::default())
    }

    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError> {
        // Validate root path exists
        if !root.exists() {
            return Err(ScanError::Path(format!("Path '{}' does not exist", root.display())));
        }

        // 0. Resolve project kind: explicit config overrides auto-detection
        let resolved_kind = match &config.project_kind {
            Some(kind) => kind.clone(),
            None => detect_project_kind(root),
        };

        // 1. Load rules (embedded default or external path)
        let rules_toml = match &config.rules_path {
            Some(path) => {
                std::fs::read_to_string(path).map_err(|e| {
                    ScanError::Config(format!("Cannot read rules file '{}': {}", path.display(), e))
                })?
            }
            None => DEFAULT_RULES.to_string(),
        };

        // 2. Parse rules and build registry
        let ruleset = rules::parse_rules(&rules_toml)?;
        let registry = rules::build_registry(&ruleset.rules)?;

        // 3. Scanner discovers all files (single traversal)
        let scanner = FileSystemScanner;
        let files = scanner.scan_files(root);

        // 4. Parse Cargo.toml for context
        let cargo_manifest = cargo_manifest::parse_cargo_toml(root)
            .unwrap_or(None);

        // 5. Create ScanContext
        let ctx = ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_kind: resolved_kind.clone(),
            cargo_manifest,
        };

        // 6. Filter and run checks
        let mut results = Vec::new();
        for runner in &registry {
            let check_id = runner.id().0;

            // Filter by --checks if specified
            if let Some(ref check_ids) = config.checks {
                if !check_ids.contains(&check_id) {
                    continue;
                }
            }

            // Filter by project_kind: find the matching rule def
            let rule_def = ruleset.rules.iter().find(|r| r.id == check_id);
            if let Some(rule) = rule_def {
                if let Some(ref rule_kind) = rule.project_kind {
                    if *rule_kind != resolved_kind {
                        results.push(CheckEntry {
                            id: runner.id(),
                            category: runner.category().to_string(),
                            description: runner.description().to_string(),
                            result: CheckResult::Skip {
                                reason: format!(
                                    "Skipped: requires {:?} project (detected {:?})",
                                    rule_kind, resolved_kind
                                ),
                            },
                        });
                        continue;
                    }
                }
            }

            // 7. Run the check
            let result = runner.run(&ctx);
            results.push(CheckEntry {
                id: runner.id(),
                category: runner.category().to_string(),
                description: runner.description().to_string(),
                result,
            });
        }

        // 8. Compute summary
        let total = results.len() as u8;
        let passed = results.iter().filter(|e| matches!(e.result, CheckResult::Pass)).count() as u8;
        let failed = results.iter().filter(|e| matches!(e.result, CheckResult::Fail { .. })).count() as u8;
        let skipped = results.iter().filter(|e| matches!(e.result, CheckResult::Skip { .. })).count() as u8;

        // 9. Return ScanReport
        Ok(ScanReport {
            results,
            summary: ScanSummary { total, passed, failed, skipped },
            project_kind: resolved_kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::ComplianceEngine;
    use crate::core::rules::default_rule_count;
    use tempfile::TempDir;

    #[test]
    fn test_nonexistent_path() {
        let engine = StructComplianceEngine;
        let result = engine.scan(std::path::Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::Path(_)));
    }

    #[test]
    fn test_default_config() {
        let tmp = TempDir::new().unwrap();
        let engine = StructComplianceEngine;
        let report = engine.scan(tmp.path()).unwrap();
        let expected = default_rule_count();
        assert_eq!(report.results.len(), expected);
        assert_eq!(report.summary.total, expected as u8);
    }

    #[test]
    fn test_check_filter() {
        let tmp = TempDir::new().unwrap();
        let engine = StructComplianceEngine;
        let config = ScanConfig {
            project_kind: Some(ProjectKind::Library),
            checks: Some(vec![1, 2, 3]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 3);
        let ids: Vec<u8> = report.results.iter().map(|e| e.id.0).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_summary_counts() {
        let tmp = TempDir::new().unwrap();
        let engine = StructComplianceEngine;
        let report = engine.scan(tmp.path()).unwrap();
        assert_eq!(
            report.summary.total,
            report.summary.passed + report.summary.failed + report.summary.skipped
        );
    }

    #[test]
    fn test_detect_library() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "mylib"
version = "0.1.0"

[lib]
path = "src/lib.rs"
"#).unwrap();
        assert_eq!(detect_project_kind(tmp.path()), ProjectKind::Library);
    }

    #[test]
    fn test_detect_binary() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "mybin"
version = "0.1.0"

[[bin]]
name = "mybin"
path = "src/main.rs"
"#).unwrap();
        assert_eq!(detect_project_kind(tmp.path()), ProjectKind::Binary);
    }

    #[test]
    fn test_detect_both() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "myboth"
version = "0.1.0"

[lib]
path = "src/lib.rs"

[[bin]]
name = "myboth"
path = "src/main.rs"
"#).unwrap();
        assert_eq!(detect_project_kind(tmp.path()), ProjectKind::Both);
    }

    #[test]
    fn test_detect_workspace() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[workspace]
members = ["crate-a"]
"#).unwrap();
        assert_eq!(detect_project_kind(tmp.path()), ProjectKind::Workspace);
    }

    #[test]
    fn test_detect_no_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(detect_project_kind(tmp.path()), ProjectKind::Library);
    }

    #[test]
    fn test_auto_detect_in_scan() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), r#"
[package]
name = "mylib"
version = "0.1.0"

[lib]
path = "src/lib.rs"
"#).unwrap();
        let engine = StructComplianceEngine;
        let report = engine.scan(tmp.path()).unwrap();
        assert_eq!(report.project_kind, ProjectKind::Library);
    }
}
