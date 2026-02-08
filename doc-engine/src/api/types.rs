use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::spi::types::{CheckId, CheckResult, ProjectType, Severity};

/// Configuration for a scan
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub project_type: ProjectType,
    pub checks: Option<Vec<u8>>,
    pub rules_path: Option<PathBuf>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            project_type: ProjectType::OpenSource,
            checks: None,
            rules_path: None,
        }
    }
}

/// Enriched check entry with metadata per DR-01
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEntry {
    pub id: CheckId,
    pub category: String,
    pub description: String,
    pub result: CheckResult,
}

/// Summary of scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub total: u8,
    pub passed: u8,
    pub failed: u8,
    pub skipped: u8,
}

/// Complete scan report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub results: Vec<CheckEntry>,
    pub summary: ScanSummary,
    pub project_type: ProjectType,
}

/// Parsed rule set from TOML
#[derive(Debug, Clone)]
pub struct RuleSet {
    pub rules: Vec<RuleDef>,
}

/// A single rule definition parsed from TOML
#[derive(Debug, Clone)]
pub struct RuleDef {
    pub id: u8,
    pub category: String,
    pub description: String,
    pub severity: Severity,
    pub rule_type: RuleType,
    pub project_type: Option<ProjectType>,
}

/// The type of a rule — declarative or builtin
#[derive(Debug, Clone)]
pub enum RuleType {
    FileExists { path: String },
    DirExists { path: String },
    DirNotExists { path: String, message: String },
    FileContentMatches { path: String, pattern: String },
    FileContentNotMatches { path: String, pattern: String },
    GlobContentMatches { glob: String, pattern: String },
    GlobContentNotMatches { glob: String, pattern: String, exclude_pattern: Option<String> },
    GlobNamingMatches { glob: String, pattern: String },
    GlobNamingNotMatches { glob: String, pattern: String, exclude_paths: Option<Vec<String>> },
    Builtin { handler: String },
}
