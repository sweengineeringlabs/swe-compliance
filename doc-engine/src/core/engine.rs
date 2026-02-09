use std::collections::HashMap;
use std::path::Path;

use crate::api::traits::ComplianceEngine;
use crate::api::types::{ScanConfig, ScanReport, ScanSummary, CheckEntry};
use crate::spi::traits::FileScanner;
use crate::spi::types::{CheckResult, ScanContext, ScanError};
use super::rules::{self, DEFAULT_RULES};
use super::scanner::FileSystemScanner;

pub struct DocComplianceEngine;

impl ComplianceEngine for DocComplianceEngine {
    fn scan(&self, root: &Path) -> Result<ScanReport, ScanError> {
        self.scan_with_config(root, &ScanConfig::default())
    }

    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError> {
        // Validate root path exists
        if !root.exists() {
            return Err(ScanError::Path(format!("Path '{}' does not exist", root.display())));
        }

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
            project_type: config.project_type.clone(),
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
                    if *rule_pt != config.project_type {
                        results.push(CheckEntry {
                            id: runner.id(),
                            category: runner.category().to_string(),
                            description: runner.description().to_string(),
                            result: CheckResult::Skip {
                                reason: format!(
                                    "Skipped: requires {:?} project type",
                                    rule_pt
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
            project_type: config.project_type.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::ComplianceEngine;
    use crate::spi::types::ProjectType;
    use tempfile::TempDir;

    #[test]
    fn test_nonexistent_path() {
        let engine = DocComplianceEngine;
        let result = engine.scan(std::path::Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::Path(_)));
    }

    #[test]
    fn test_default_config() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let report = engine.scan(tmp.path()).unwrap();
        assert_eq!(report.results.len(), 78);
        assert_eq!(report.summary.total, 78);
    }

    #[test]
    fn test_check_filter() {
        let tmp = TempDir::new().unwrap();
        let engine = DocComplianceEngine;
        let config = ScanConfig {
            project_type: ProjectType::OpenSource,
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
            project_type: ProjectType::Internal,
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
        let report = engine.scan(tmp.path()).unwrap();
        assert_eq!(
            report.summary.total,
            report.summary.passed + report.summary.failed + report.summary.skipped
        );
    }
}
