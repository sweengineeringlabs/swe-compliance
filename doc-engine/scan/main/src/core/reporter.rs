use std::collections::BTreeMap;

use crate::api::types::ScanReport;
use crate::api::traits::Reporter;
use crate::api::types::CheckResult;

pub struct TextReporter;
pub struct JsonReporter;

impl Reporter for TextReporter {
    fn report(&self, report: &ScanReport) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "doc-engine scan results (project type: {:?}, scope: {:?})\n",
            report.project_type, report.project_scope
        ));
        output.push_str(&"=".repeat(60));
        output.push('\n');
        output.push('\n');

        // Group results by category
        let mut by_category: BTreeMap<&str, Vec<_>> = BTreeMap::new();
        for entry in &report.results {
            by_category.entry(&entry.category).or_default().push(entry);
        }

        for (category, entries) in &by_category {
            output.push_str(&format!("## {}\n", category));

            for entry in entries {
                let status = match &entry.result {
                    CheckResult::Pass => "PASS",
                    CheckResult::Fail { .. } => "FAIL",
                    CheckResult::Skip { .. } => "SKIP",
                };

                output.push_str(&format!(
                    "  [{}] {}: {}\n",
                    status, entry.id, entry.description
                ));

                // Show violations for failures
                if let CheckResult::Fail { violations } = &entry.result {
                    for v in violations {
                        let path_str = v.path.as_ref()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if path_str.is_empty() {
                            output.push_str(&format!("    -> {}\n", v.message));
                        } else {
                            output.push_str(&format!("    -> {}: {}\n", path_str, v.message));
                        }
                    }
                }

                // Show reason for skips
                if let CheckResult::Skip { reason } = &entry.result {
                    output.push_str(&format!("    -> {}\n", reason));
                }
            }
            output.push('\n');
        }

        // Summary line
        output.push_str(&format!(
            "{}/{} passed, {} failed, {} skipped\n",
            report.summary.passed,
            report.summary.total,
            report.summary.failed,
            report.summary.skipped,
        ));

        output
    }
}

impl Reporter for JsonReporter {
    fn report(&self, report: &ScanReport) -> String {
        serde_json::to_string_pretty(report).unwrap_or_else(|e| {
            format!("{{\"error\": \"JSON serialization failed: {}\"}}", e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{ScanSummary, CheckEntry};
    use crate::api::types::{CheckId, ProjectScope, ProjectType, Violation, Severity};

    fn make_report(entries: Vec<CheckEntry>) -> ScanReport {
        let total = entries.len() as u8;
        let passed = entries.iter().filter(|e| matches!(e.result, CheckResult::Pass)).count() as u8;
        let failed = entries.iter().filter(|e| matches!(e.result, CheckResult::Fail { .. })).count() as u8;
        let skipped = entries.iter().filter(|e| matches!(e.result, CheckResult::Skip { .. })).count() as u8;
        ScanReport {
            standard: "ISO/IEC/IEEE 15289:2019".to_string(),
            clause: "9.2".to_string(),
            tool: "doc-engine".to_string(),
            tool_version: "0.1.0".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            project_root: "/tmp/test".to_string(),
            results: entries,
            summary: ScanSummary { total, passed, failed, skipped },
            project_type: ProjectType::OpenSource,
            project_scope: ProjectScope::Large,
        }
    }

    #[test]
    fn test_text_pass_only() {
        let report = make_report(vec![
            CheckEntry {
                id: CheckId(1),
                category: "structure".to_string(),
                description: "docs/ exists".to_string(),
                result: CheckResult::Pass,
            },
        ]);
        let text = TextReporter.report(&report);
        assert!(text.contains("[PASS]"));
        assert!(!text.contains("[FAIL]"));
    }

    #[test]
    fn test_text_mixed() {
        let report = make_report(vec![
            CheckEntry {
                id: CheckId(1),
                category: "structure".to_string(),
                description: "check pass".to_string(),
                result: CheckResult::Pass,
            },
            CheckEntry {
                id: CheckId(2),
                category: "structure".to_string(),
                description: "check fail".to_string(),
                result: CheckResult::Fail {
                    violations: vec![Violation {
                        check_id: CheckId(2),
                        path: Some("docs/bad.md".into()),
                        message: "violation msg".to_string(),
                        severity: Severity::Error,
                    }],
                },
            },
            CheckEntry {
                id: CheckId(3),
                category: "structure".to_string(),
                description: "check skip".to_string(),
                result: CheckResult::Skip { reason: "not applicable".to_string() },
            },
        ]);
        let text = TextReporter.report(&report);
        assert!(text.contains("[PASS]"));
        assert!(text.contains("[FAIL]"));
        assert!(text.contains("[SKIP]"));
        assert!(text.contains("violation msg"));
        assert!(text.contains("not applicable"));
    }

    #[test]
    fn test_text_summary() {
        let report = make_report(vec![
            CheckEntry {
                id: CheckId(1),
                category: "a".to_string(),
                description: "d".to_string(),
                result: CheckResult::Pass,
            },
            CheckEntry {
                id: CheckId(2),
                category: "a".to_string(),
                description: "d".to_string(),
                result: CheckResult::Fail { violations: vec![] },
            },
        ]);
        let text = TextReporter.report(&report);
        assert!(text.contains("1/2 passed, 1 failed, 0 skipped"));
    }

    #[test]
    fn test_json_valid() {
        let report = make_report(vec![
            CheckEntry {
                id: CheckId(1),
                category: "a".to_string(),
                description: "d".to_string(),
                result: CheckResult::Pass,
            },
        ]);
        let json = JsonReporter.report(&report);
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(val.is_object());
    }

    #[test]
    fn test_json_roundtrip() {
        let report = make_report(vec![
            CheckEntry {
                id: CheckId(1),
                category: "structure".to_string(),
                description: "docs/ exists".to_string(),
                result: CheckResult::Pass,
            },
            CheckEntry {
                id: CheckId(2),
                category: "structure".to_string(),
                description: "fail check".to_string(),
                result: CheckResult::Fail {
                    violations: vec![Violation {
                        check_id: CheckId(2),
                        path: Some("bad.md".into()),
                        message: "bad".to_string(),
                        severity: Severity::Warning,
                    }],
                },
            },
        ]);
        let json = JsonReporter.report(&report);
        let deserialized: ScanReport = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.results.len(), 2);
        assert_eq!(deserialized.summary.total, 2);
        assert_eq!(deserialized.summary.passed, 1);
        assert_eq!(deserialized.summary.failed, 1);
    }
}
