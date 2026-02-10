use std::collections::HashMap;
use std::path::Path;

use crate::api::traits::ComplianceEngine;
use crate::api::types::{ScanConfig, ScanReport, ScanSummary, CheckEntry};
use crate::spi::traits::FileScanner;
use crate::spi::types::{CheckResult, ProjectType, ScanContext, ScanError};
use super::rules::{self, DEFAULT_RULES};
use super::scanner::FileSystemScanner;

/// Detect project type from LICENSE file content.
///
/// Reads `LICENSE`, `LICENSE.md`, or `LICENSE.txt` at the project root and
/// looks for common open-source license identifiers (MIT, Apache, GPL, BSD,
/// etc.). Returns [`ProjectType::OpenSource`] if found, otherwise
/// [`ProjectType::Internal`].
pub fn detect_project_type(root: &Path) -> ProjectType {
    let license_path = root.join("LICENSE");
    let content = if license_path.exists() {
        std::fs::read_to_string(&license_path).unwrap_or_default()
    } else {
        let alt_paths = [root.join("LICENSE.md"), root.join("LICENSE.txt")];
        alt_paths.iter()
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_default()
    };

    if content.is_empty() {
        return ProjectType::Internal;
    }

    let upper = content.to_uppercase();
    let oss_indicators = [
        "MIT LICENSE",
        "APACHE LICENSE",
        "GNU GENERAL PUBLIC LICENSE",
        "GNU LESSER GENERAL PUBLIC",
        "BSD ",
        "MOZILLA PUBLIC LICENSE",
        "ISC LICENSE",
        "BOOST SOFTWARE LICENSE",
        "THE UNLICENSE",
        "CREATIVE COMMONS",
        "EUROPEAN UNION PUBLIC",
        "OPEN SOFTWARE LICENSE",
        "ARTISTIC LICENSE",
        "ZLIB LICENSE",
        "DO WHAT THE FUCK YOU WANT",
    ];

    if oss_indicators.iter().any(|ind| upper.contains(ind)) {
        ProjectType::OpenSource
    } else {
        ProjectType::Internal
    }
}

/// Doc-engine compliance engine.
pub struct DocComplianceEngine;

impl ComplianceEngine for DocComplianceEngine {
    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError> {
        // Validate root path exists
        if !root.exists() {
            return Err(ScanError::Path(format!("Path '{}' does not exist", root.display())));
        }

        // 0. Resolve project type: explicit config overrides auto-detection from LICENSE
        let resolved_pt = match &config.project_type {
            Some(pt) => pt.clone(),
            None => detect_project_type(root),
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

        // 3. Scanner discovers all files (single traversal per NFR-201)
        let scanner = FileSystemScanner;
        let files = scanner.scan_files(root);

        // 4. Create ScanContext
        let ctx = ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_type: resolved_pt.clone(),
            project_scope: config.project_scope,
        };

        // 5. Filter and run checks
        let mut results = Vec::new();
        for runner in &registry {
            let check_id = runner.id().0;

            // Filter by --checks if specified
            if let Some(ref check_ids) = config.checks {
                if !check_ids.contains(&check_id) {
                    continue;
                }
            }

            // Filter by project_type: find the matching rule def
            let rule_def = ruleset.rules.iter().find(|r| r.id == check_id);
            if let Some(rule) = rule_def {
                if let Some(ref rule_pt) = rule.project_type {
                    if *rule_pt != resolved_pt {
                        results.push(CheckEntry {
                            id: runner.id(),
                            category: runner.category().to_string(),
                            description: runner.description().to_string(),
                            result: CheckResult::Skip {
                                reason: format!(
                                    "Skipped: requires {:?} project (detected {:?})",
                                    rule_pt, resolved_pt
                                ),
                            },
                        });
                        continue;
                    }
                }

                // Filter by scope: skip rules that require a higher scope tier
                if let Some(ref rule_scope) = rule.scope {
                    if *rule_scope > config.project_scope {
                        results.push(CheckEntry {
                            id: runner.id(),
                            category: runner.category().to_string(),
                            description: runner.description().to_string(),
                            result: CheckResult::Skip {
                                reason: format!(
                                    "Skipped: requires {:?} scope (configured {:?})",
                                    rule_scope, config.project_scope
                                ),
                            },
                        });
                        continue;
                    }
                }
            }

            // 6. Run the check
            let result = runner.run(&ctx);
            results.push(CheckEntry {
                id: runner.id(),
                category: runner.category().to_string(),
                description: runner.description().to_string(),
                result,
            });
        }

        // 7. Compute summary
        let total = results.len() as u8;
        let passed = results.iter().filter(|e| matches!(e.result, CheckResult::Pass)).count() as u8;
        let failed = results.iter().filter(|e| matches!(e.result, CheckResult::Fail { .. })).count() as u8;
        let skipped = results.iter().filter(|e| matches!(e.result, CheckResult::Skip { .. })).count() as u8;

        // 8. Return ScanReport
        Ok(ScanReport {
            results,
            summary: ScanSummary { total, passed, failed, skipped },
            project_type: resolved_pt,
            project_scope: config.project_scope,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::ComplianceEngine;
    use crate::core::rules::default_rule_count;
    use crate::spi::types::{ProjectScope, ProjectType};
    use tempfile::TempDir;

    #[test]
    fn test_nonexistent_path() {
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Large,
            checks: None,
            rules_path: None,
        };
        let result = engine.scan_with_config(std::path::Path::new("/nonexistent/path/xyz"), &config);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::Path(_)));
    }

    #[test]
    fn test_all_checks_run() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Large,
            checks: None,
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        let expected = default_rule_count();
        assert_eq!(report.results.len(), expected);
        assert_eq!(report.summary.total, expected as u8);
    }

    #[test]
    fn test_check_filter() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Large,
            checks: Some(vec![1, 2, 3]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 3);
        let ids: Vec<u8> = report.results.iter().map(|e| e.id.0).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_project_type_skip() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        // Checks 31 and 32 are open_source only; with Internal, they should be skipped
        let config = ScanConfig {
            project_type: Some(ProjectType::Internal),
            project_scope: ProjectScope::Large,
            checks: Some(vec![31, 32]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 2);
        for entry in &report.results {
            assert!(matches!(entry.result, CheckResult::Skip { .. }));
        }
    }

    #[test]
    fn test_summary_counts() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Large,
            checks: None,
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(
            report.summary.total,
            report.summary.passed + report.summary.failed + report.summary.skipped
        );
    }

    #[test]
    fn test_detect_mit_license() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "MIT License\n\nCopyright (c) 2026\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::OpenSource));
    }

    #[test]
    fn test_detect_apache_license() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "Apache License\nVersion 2.0\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::OpenSource));
    }

    #[test]
    fn test_detect_gpl_license() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "GNU General Public License\nVersion 3\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::OpenSource));
    }

    #[test]
    fn test_detect_bsd_license() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "BSD 3-Clause License\n\nCopyright\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::OpenSource));
    }

    #[test]
    fn test_detect_no_license_is_internal() {
        let tmp = TempDir::new().unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::Internal));
    }

    #[test]
    fn test_detect_proprietary_is_internal() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "Copyright 2026 Acme Corp. All rights reserved.\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::Internal));
    }

    #[test]
    fn test_detect_license_md() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE.md"), "# MIT License\n").unwrap();
        assert!(matches!(detect_project_type(tmp.path()), ProjectType::OpenSource));
    }

    #[test]
    fn test_auto_detect_in_scan() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("LICENSE"), "MIT License\n").unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: None,
            project_scope: ProjectScope::Large,
            checks: Some(vec![1]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.project_type, ProjectType::OpenSource);
    }

    #[test]
    fn test_auto_detect_internal_no_license() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: None,
            project_scope: ProjectScope::Large,
            checks: Some(vec![1]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.project_type, ProjectType::Internal);
    }

    #[test]
    fn test_scope_small_skips_medium_rules() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Small,
            checks: Some(vec![11]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(matches!(report.results[0].result, CheckResult::Skip { .. }));
    }

    #[test]
    fn test_scope_medium_runs_small_rules() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Medium,
            checks: Some(vec![1]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(!matches!(report.results[0].result, CheckResult::Skip { .. }));
    }

    #[test]
    fn test_scope_large_skips_nothing_by_scope() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Large,
            checks: Some(vec![1, 11, 89]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 3);
        for entry in &report.results {
            if let CheckResult::Skip { ref reason } = entry.result {
                assert!(!reason.contains("scope"), "Check {} was unexpectedly skipped for scope: {}", entry.id.0, reason);
            }
        }
    }

    #[test]
    fn test_scope_in_report() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Medium,
            checks: Some(vec![1]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.project_scope, ProjectScope::Medium);
    }

    #[test]
    fn test_scope_small_skips_large_rules() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: Some(ProjectType::OpenSource),
            project_scope: ProjectScope::Small,
            checks: Some(vec![89]),
            rules_path: None,
        };
        let report = engine.scan_with_config(tmp.path(), &config).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(matches!(report.results[0].result, CheckResult::Skip { .. }));
        if let CheckResult::Skip { ref reason } = report.results[0].result {
            assert!(reason.contains("scope"));
        }
    }
}
