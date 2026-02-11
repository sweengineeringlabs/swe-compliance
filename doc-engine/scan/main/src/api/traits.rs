use std::path::{Path, PathBuf};

use super::types::{CheckId, CheckResult, ScanConfig, ScanContext, ScanError, ScanReport};

/// Abstracts file system traversal for project scanning.
///
/// Implementations discover files under a project root, test for existence,
/// and read file contents.
pub trait FileScanner {
    /// Recursively list all files under `root`, returning relative paths.
    fn scan_files(&self, root: &Path) -> Vec<PathBuf>;
    /// Check whether a file at the given relative path exists under `root`.
    fn file_exists(&self, root: &Path, relative: &str) -> bool;
    /// Read the entire contents of `path` into a string.
    fn read_file(&self, path: &Path) -> Result<String, ScanError>;
}

/// Executes a single compliance check against a scan context.
///
/// Each check has a unique numeric identifier, a category label, a
/// human-readable description, and a [`run`](CheckRunner::run) method that
/// produces a [`CheckResult`].
pub trait CheckRunner: Send + Sync {
    /// Return the unique numeric identifier for this check.
    fn id(&self) -> CheckId;
    /// Return the category label (e.g. `"structure"`, `"naming"`).
    fn category(&self) -> &str;
    /// Return a short human-readable description of what this check verifies.
    fn description(&self) -> &str;
    /// Execute the check against the given [`ScanContext`] and return the result.
    fn run(&self, ctx: &ScanContext) -> CheckResult;
}

/// Engine for running documentation compliance scans.
///
/// Implementors walk a project directory, execute compliance checks, and
/// produce a [`ScanReport`].
pub trait ComplianceEngine {
    /// Scan a project directory with the supplied [`ScanConfig`].
    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>;
}

/// Formats scan reports for output.
///
/// Implementations convert a [`ScanReport`] into a display string
/// (e.g. plain text, JSON).
pub trait Reporter {
    /// Render the given report as a string.
    fn report(&self, report: &ScanReport) -> String;
}
