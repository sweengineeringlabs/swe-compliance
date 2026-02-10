use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
