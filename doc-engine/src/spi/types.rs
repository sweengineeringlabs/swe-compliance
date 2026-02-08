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

/// Severity level of a check violation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A single violation found by a check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub check_id: CheckId,
    pub path: Option<PathBuf>,
    pub message: String,
    pub severity: Severity,
}

/// Outcome of running a single check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum CheckResult {
    Pass,
    Fail { violations: Vec<Violation> },
    Skip { reason: String },
}

/// Error type for scan operations
#[derive(Debug)]
pub enum ScanError {
    Io(io::Error),
    Path(String),
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

/// Project type for filtering checks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    OpenSource,
    Internal,
}

/// Context passed to each CheckRunner during scan
pub struct ScanContext {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
    pub file_contents: HashMap<PathBuf, String>,
    pub project_type: ProjectType,
}
