use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::spi::types::{CheckId, CheckResult, ProjectType, Severity};

/// Configuration for a scan.
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// The project type used to filter applicable checks.
    pub project_type: ProjectType,
    /// Optional subset of check IDs to run; `None` runs all checks.
    pub checks: Option<Vec<u8>>,
    /// Optional path to a custom rules TOML file; `None` uses the built-in rules.
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

/// Enriched check entry with metadata per DR-01.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEntry {
    /// Unique numeric identifier for the check.
    pub id: CheckId,
    /// Category label (e.g. `"structure"`, `"naming"`).
    pub category: String,
    /// Human-readable description of what this check verifies.
    pub description: String,
    /// The outcome of running this check.
    pub result: CheckResult,
}

/// Summary of scan results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    /// Total number of checks executed.
    pub total: u8,
    /// Number of checks that passed.
    pub passed: u8,
    /// Number of checks that failed.
    pub failed: u8,
    /// Number of checks that were skipped.
    pub skipped: u8,
}

/// Complete scan report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    /// Per-check results in execution order.
    pub results: Vec<CheckEntry>,
    /// Aggregate pass/fail/skip counts.
    pub summary: ScanSummary,
    /// The project type that was used during this scan.
    pub project_type: ProjectType,
}

/// Parsed rule set from TOML.
#[derive(Debug, Clone)]
pub struct RuleSet {
    /// The ordered list of rule definitions.
    pub rules: Vec<RuleDef>,
}

/// A single rule definition parsed from TOML.
#[derive(Debug, Clone)]
pub struct RuleDef {
    /// Numeric rule identifier (1-based).
    pub id: u8,
    /// Category label for grouping (e.g. `"structure"`).
    pub category: String,
    /// Human-readable description of the rule.
    pub description: String,
    /// Severity assigned to violations of this rule.
    pub severity: Severity,
    /// The type of check to perform (declarative or builtin).
    pub rule_type: RuleType,
    /// Optional project-type filter; `None` means the rule applies to all types.
    pub project_type: Option<ProjectType>,
}

/// The type of a rule -- declarative or builtin.
#[derive(Debug, Clone)]
pub enum RuleType {
    /// Assert that a file exists at the given path.
    FileExists {
        /// Relative path to the expected file.
        path: String,
    },
    /// Assert that a directory exists at the given path.
    DirExists {
        /// Relative path to the expected directory.
        path: String,
    },
    /// Assert that a directory does NOT exist.
    DirNotExists {
        /// Relative path to the directory that should be absent.
        path: String,
        /// Custom violation message.
        message: String,
    },
    /// Assert that a file's content matches a regex pattern.
    FileContentMatches {
        /// Relative path to the file.
        path: String,
        /// Regex that must match somewhere in the file.
        pattern: String,
    },
    /// Assert that a file's content does NOT match a regex pattern.
    FileContentNotMatches {
        /// Relative path to the file.
        path: String,
        /// Regex that must not match anywhere in the file.
        pattern: String,
    },
    /// Assert that all files matching a glob have content matching a regex.
    GlobContentMatches {
        /// Glob pattern to select files.
        glob: String,
        /// Regex that must match in each selected file.
        pattern: String,
    },
    /// Assert that no file matching a glob contains content matching a regex.
    GlobContentNotMatches {
        /// Glob pattern to select files.
        glob: String,
        /// Regex that must not match in any selected file.
        pattern: String,
        /// Optional regex; lines matching this pattern are excluded from the check.
        exclude_pattern: Option<String>,
    },
    /// Assert that filenames matching a glob also match a naming regex.
    GlobNamingMatches {
        /// Glob pattern to select files.
        glob: String,
        /// Regex that each filename must match.
        pattern: String,
    },
    /// Assert that filenames matching a glob do NOT match a forbidden regex.
    GlobNamingNotMatches {
        /// Glob pattern to select files.
        glob: String,
        /// Regex that filenames must not match.
        pattern: String,
        /// Optional path prefixes to exclude from the check.
        exclude_paths: Option<Vec<String>>,
    },
    /// A check implemented in Rust code rather than declaratively.
    Builtin {
        /// Name of the builtin handler function.
        handler: String,
    },
}
