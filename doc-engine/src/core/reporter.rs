use std::collections::BTreeMap;

use crate::api::types::ScanReport;
use crate::spi::traits::Reporter;
use crate::spi::types::CheckResult;

pub struct TextReporter;
pub struct JsonReporter;

impl Reporter for TextReporter {
    fn report(&self, report: &ScanReport) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "doc-engine scan results (project type: {:?})\n",
            report.project_type
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
