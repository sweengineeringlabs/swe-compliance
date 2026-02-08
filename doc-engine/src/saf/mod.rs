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

pub fn scan(root: &Path) -> Result<ScanReport, ScanError> {
    scan_with_config(root, &ScanConfig::default())
}

pub fn scan_with_config(root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError> {
    DocComplianceEngine.scan_with_config(root, config)
}

pub fn format_report_text(report: &ScanReport) -> String {
    TextReporter.report(report)
}

pub fn format_report_json(report: &ScanReport) -> String {
    JsonReporter.report(report)
}
