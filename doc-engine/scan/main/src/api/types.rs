use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Core value types (formerly spi::types)
// ---------------------------------------------------------------------------

/// Newtype wrapping check number 1-65
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckId(pub u8);

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Severity level of a check violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// A blocking error that must be fixed before release.
    Error,
    /// A non-blocking issue that should be addressed.
    Warning,
    /// An informational note with no compliance impact.
    Info,
}

/// A single violation found by a check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// The check that produced this violation.
    pub check_id: CheckId,
    /// The file path related to the violation, if applicable.
    pub path: Option<PathBuf>,
    /// Human-readable description of the violation.
    pub message: String,
    /// Severity level of this violation.
    pub severity: Severity,
}

/// Outcome of running a single check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum CheckResult {
    /// The check passed with no violations.
    Pass,
    /// The check failed; contains one or more violations.
    Fail {
        /// The violations that caused the check to fail.
        violations: Vec<Violation>,
    },
    /// The check was skipped (e.g. not applicable for the project type).
    Skip {
        /// Explanation of why the check was skipped.
        reason: String,
    },
}

/// Error type for scan operations.
#[derive(Debug)]
pub enum ScanError {
    /// An I/O error encountered during scanning.
    Io(io::Error),
    /// The supplied project path is invalid or does not exist.
    Path(String),
    /// A configuration or rule parsing error.
    Config(String),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::Io(e) => write!(f, "IO error: {}", e),
            ScanError::Path(s) => write!(f, "Path error: {}", s),
            ScanError::Config(s) => write!(f, "Config error: {}", s),
        }
    }
}

impl std::error::Error for ScanError {}

impl From<io::Error> for ScanError {
    fn from(e: io::Error) -> Self {
        ScanError::Io(e)
    }
}

/// Project type for filtering checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    /// An open-source project with community-facing requirements.
    OpenSource,
    /// An internal/proprietary project.
    Internal,
}

/// Project scope tier for filtering checks by project size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectScope {
    /// Small project (5-10 modules): minimal structure.
    Small,
    /// Medium project (10-20 modules): full structure with security, ADRs.
    Medium,
    /// Large project (20+ modules): complete SDLC with ISO compliance.
    Large,
}

/// Context passed to each CheckRunner during scan.
pub struct ScanContext {
    /// Absolute path to the project root directory.
    pub root: PathBuf,
    /// Relative paths of all files discovered under `root`.
    pub files: Vec<PathBuf>,
    /// Cached file contents keyed by path.
    pub file_contents: HashMap<PathBuf, String>,
    /// The project type used to filter applicable checks.
    pub project_type: ProjectType,
    /// The project scope tier used to filter checks by project size.
    pub project_scope: ProjectScope,
    /// Optional module name filter from CLI `--module`; passed to module check handlers.
    pub module_filter: Option<Vec<String>>,
}

/// Returns the current UTC time as an ISO 8601 string (e.g. "2026-02-10T14:30:00Z").
///
/// Uses Howard Hinnant's civil_from_days algorithm on `std::time::SystemTime`.
/// No external date/time crate required.
pub fn iso8601_now() -> String {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    let day_secs = secs % 86400;
    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;
    let seconds = day_secs % 60;

    // Howard Hinnant's civil_from_days algorithm
    let z = (secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, minutes, seconds
    )
}

// ---------------------------------------------------------------------------
// Configuration and report types (formerly api::types)
// ---------------------------------------------------------------------------

/// Configuration for a scan.
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// The project type used to filter applicable checks.
    /// `None` means auto-detect from the LICENSE file at the project root.
    pub project_type: Option<ProjectType>,
    /// The project scope tier for filtering checks by project size.
    pub project_scope: ProjectScope,
    /// Optional subset of check IDs to run; `None` runs all checks.
    pub checks: Option<Vec<u8>>,
    /// Optional path to a custom rules TOML file; `None` uses the built-in rules.
    pub rules_path: Option<PathBuf>,
    /// Optional phase/category filter; `None` runs all categories.
    pub phases: Option<Vec<String>>,
    /// Optional module name filter; `None` checks all discovered modules.
    pub module_filter: Option<Vec<String>>,
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

/// Complete scan report (ISO/IEC/IEEE 15289:2019 clause 9.2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    /// ISO standard identifier.
    pub standard: String,
    /// Clause reference within the standard.
    pub clause: String,
    /// Tool name that produced this report.
    pub tool: String,
    /// Semantic version of the tool.
    pub tool_version: String,
    /// ISO 8601 UTC timestamp of report generation.
    pub timestamp: String,
    /// Canonicalized absolute path to the scanned project root.
    pub project_root: String,
    /// Per-check results in execution order.
    pub results: Vec<CheckEntry>,
    /// Aggregate pass/fail/skip counts.
    pub summary: ScanSummary,
    /// The project type that was used during this scan.
    pub project_type: ProjectType,
    /// The project scope tier that was used during this scan.
    pub project_scope: ProjectScope,
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
    /// Optional scope tier; `None` means the rule applies at all scope levels.
    pub scope: Option<ProjectScope>,
    /// Parent check IDs that must pass before this check runs.
    pub depends_on: Vec<u8>,
    /// Optional per-rule module name filter from rules.toml.
    pub module_filter: Option<Vec<String>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso8601_now_format() {
        let ts = iso8601_now();
        // Must match YYYY-MM-DDTHH:MM:SSZ
        assert!(ts.ends_with('Z'), "timestamp must end with Z: {}", ts);
        assert_eq!(ts.len(), 20, "ISO 8601 UTC is exactly 20 chars: {}", ts);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_project_scope_ordering() {
        assert!(ProjectScope::Small < ProjectScope::Medium);
        assert!(ProjectScope::Medium < ProjectScope::Large);
        assert!(ProjectScope::Small < ProjectScope::Large);
    }

    #[test]
    fn test_project_scope_equality() {
        assert_eq!(ProjectScope::Small, ProjectScope::Small);
        assert_eq!(ProjectScope::Medium, ProjectScope::Medium);
        assert_eq!(ProjectScope::Large, ProjectScope::Large);
        assert_ne!(ProjectScope::Small, ProjectScope::Large);
    }

    #[test]
    fn test_project_scope_serde_roundtrip() {
        let json = serde_json::to_string(&ProjectScope::Medium).unwrap();
        assert_eq!(json, "\"medium\"");
        let deserialized: ProjectScope = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ProjectScope::Medium);
    }
}
