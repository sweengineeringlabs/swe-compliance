use std::path::Path;

use crate::api::traits::ComplianceEngine;
use crate::core::engine::DocComplianceEngine;
use crate::core::reporter::{TextReporter, JsonReporter};
use crate::spi::traits::Reporter;

// Re-export all public types from SPI
pub use crate::spi::types::{
    CheckId, CheckResult, ProjectType, Severity, Violation, ScanContext, ScanError,
};

// Re-export all public types from API
pub use crate::api::types::{
    ScanConfig, ScanReport, ScanSummary, CheckEntry, RuleSet, RuleDef, RuleType,
};

// Re-export detect_project_type for library consumers
pub use crate::core::engine::detect_project_type;

/// Scan a project directory using default configuration.
///
/// Equivalent to calling [`scan_with_config`] with [`ScanConfig::default()`].
pub fn scan(root: &Path) -> Result<ScanReport, ScanError> {
    scan_with_config(root, &ScanConfig::default())
}

/// Scan a project directory with custom configuration.
///
/// Runs every enabled compliance check against the files found under `root`
/// and returns a [`ScanReport`] with per-check results and a summary.
pub fn scan_with_config(root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError> {
    DocComplianceEngine.scan_with_config(root, config)
}

/// Format a scan report as human-readable text.
///
/// Groups results by category and appends a pass/fail/skip summary line.
pub fn format_report_text(report: &ScanReport) -> String {
    TextReporter.report(report)
}

/// Format a scan report as JSON.
///
/// Produces a pretty-printed JSON string using `serde_json`.
pub fn format_report_json(report: &ScanReport) -> String {
    JsonReporter.report(report)
}
