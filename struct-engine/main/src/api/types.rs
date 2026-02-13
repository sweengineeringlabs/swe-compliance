use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Value types (formerly spi::types)
// ---------------------------------------------------------------------------

/// Newtype wrapping check number 1-44.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckId(pub u8);

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Report output format selection for sinks that support multiple formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// Human-readable plain text grouped by category.
    Text,
    /// Pretty-printed JSON (serde_json).
    Json,
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
    /// Machine-readable rule type tag (e.g. `"file_exists"`, `"builtin:crate_root_exists"`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rule_type: String,
    /// What the rule expected (e.g. a path, pattern, or key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    /// What was actually found (e.g. `"missing"`, an actual value).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
    /// Actionable remediation hint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fix_hint: String,
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
    /// The check was skipped (e.g. not applicable for the project kind).
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

/// The kind of Rust project detected from Cargo.toml.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectKind {
    /// A library crate (has `[lib]` only).
    Library,
    /// A binary crate (has `[[bin]]` only).
    Binary,
    /// Both library and binary targets.
    Both,
    /// A Cargo workspace (has `[workspace]`).
    Workspace,
}

/// Pre-indexed file lookup for efficient check execution.
pub struct FileIndex {
    /// All files (the original flat list).
    pub all: Vec<PathBuf>,
    /// Files grouped by extension (e.g. "rs" → [...]).
    pub by_extension: HashMap<String, Vec<usize>>,
    /// Files grouped by first directory component (e.g. "src" → [...]).
    pub by_top_dir: HashMap<String, Vec<usize>>,
}

impl FileIndex {
    /// Build a FileIndex from a flat list of relative paths.
    pub fn from_files(files: Vec<PathBuf>) -> Self {
        let mut by_extension: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_top_dir: HashMap<String, Vec<usize>> = HashMap::new();

        for (i, path) in files.iter().enumerate() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                by_extension.entry(ext.to_string()).or_default().push(i);
            }
            let normalized = path.to_string_lossy().replace('\\', "/");
            if let Some(first) = normalized.split('/').next() {
                by_top_dir.entry(first.to_string()).or_default().push(i);
            }
        }

        FileIndex { all: files, by_extension, by_top_dir }
    }

    /// Get all files (backward compat).
    pub fn files(&self) -> &[PathBuf] {
        &self.all
    }

    /// Get files by extension.
    pub fn with_extension(&self, ext: &str) -> Vec<&PathBuf> {
        self.by_extension.get(ext)
            .map(|indices| indices.iter().map(|&i| &self.all[i]).collect())
            .unwrap_or_default()
    }

    /// Get files under a top-level directory.
    pub fn under_dir(&self, dir: &str) -> Vec<&PathBuf> {
        self.by_top_dir.get(dir)
            .map(|indices| indices.iter().map(|&i| &self.all[i]).collect())
            .unwrap_or_default()
    }
}

/// Context passed to each CheckRunner during scan.
pub struct ScanContext {
    /// Absolute path to the project root directory.
    pub root: PathBuf,
    /// Pre-indexed file lookup.
    pub file_index: FileIndex,
    /// Cached file contents keyed by path.
    pub file_contents: HashMap<PathBuf, String>,
    /// The project kind used to filter applicable checks.
    pub project_kind: ProjectKind,
    /// Parsed Cargo.toml manifest, if available.
    pub cargo_manifest: Option<CargoManifest>,
}

impl ScanContext {
    /// Get the list of all discovered files (backward compat accessor).
    pub fn files(&self) -> &[PathBuf] {
        self.file_index.files()
    }
}

// ---------------------------------------------------------------------------
// Configuration and report types (formerly api::types)
// ---------------------------------------------------------------------------

/// Configuration for a scan.
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// The project kind used to filter applicable checks.
    /// `None` means auto-detect from Cargo.toml.
    pub project_kind: Option<ProjectKind>,
    /// Optional subset of check IDs to run; `None` runs all checks.
    pub checks: Option<Vec<u8>>,
    /// Optional path to a custom rules TOML file; `None` uses the built-in rules.
    pub rules_path: Option<PathBuf>,
    /// Recursively scan workspace members.
    pub recursive: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        ScanConfig {
            project_kind: None,
            checks: None,
            rules_path: None,
            recursive: false,
        }
    }
}

/// Enriched check entry with metadata.
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
    /// The project kind that was used during this scan.
    pub project_kind: ProjectKind,
    /// Per-member reports for workspace recursive scans.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_reports: Vec<MemberReport>,
}

/// Report for a single workspace member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberReport {
    /// Workspace member name (relative path).
    pub member: String,
    /// Per-check results for this member.
    pub results: Vec<CheckEntry>,
    /// Aggregate pass/fail/skip counts for this member.
    pub summary: ScanSummary,
    /// The project kind detected for this member.
    pub project_kind: ProjectKind,
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
    /// Optional project-kind filter; `None` means the rule applies to all kinds.
    pub project_kind: Option<ProjectKind>,
    /// Optional custom fix hint from TOML; overrides auto-generated hint.
    pub fix_hint: Option<String>,
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
    /// Assert that a TOML key path exists in Cargo.toml.
    CargoKeyExists {
        /// Dotted key path (e.g. `"package.name"`).
        key: String,
    },
    /// Assert that a TOML key value matches a regex pattern.
    CargoKeyMatches {
        /// Dotted key path (e.g. `"package.name"`).
        key: String,
        /// Regex the value must match.
        pattern: String,
    },
}

impl RuleType {
    /// Return a stable machine-readable tag for this rule type.
    pub fn to_tag(&self) -> String {
        match self {
            RuleType::FileExists { .. } => "file_exists".to_string(),
            RuleType::DirExists { .. } => "dir_exists".to_string(),
            RuleType::DirNotExists { .. } => "dir_not_exists".to_string(),
            RuleType::FileContentMatches { .. } => "file_content_matches".to_string(),
            RuleType::FileContentNotMatches { .. } => "file_content_not_matches".to_string(),
            RuleType::GlobContentMatches { .. } => "glob_content_matches".to_string(),
            RuleType::GlobContentNotMatches { .. } => "glob_content_not_matches".to_string(),
            RuleType::GlobNamingMatches { .. } => "glob_naming_matches".to_string(),
            RuleType::GlobNamingNotMatches { .. } => "glob_naming_not_matches".to_string(),
            RuleType::Builtin { handler } => format!("builtin:{}", handler),
            RuleType::CargoKeyExists { .. } => "cargo_key_exists".to_string(),
            RuleType::CargoKeyMatches { .. } => "cargo_key_matches".to_string(),
        }
    }

    /// Return a default remediation hint derived from this rule type.
    pub fn auto_fix_hint(&self) -> String {
        match self {
            RuleType::FileExists { path } => format!("Create the file '{}'", path),
            RuleType::DirExists { path } => format!("Create the directory '{}'", path),
            RuleType::DirNotExists { path, .. } => format!("Remove the directory '{}'", path),
            RuleType::FileContentMatches { path, pattern } => {
                format!("Update '{}' so its content matches pattern '{}'", path, pattern)
            }
            RuleType::FileContentNotMatches { path, pattern } => {
                format!("Remove content matching '{}' from '{}'", pattern, path)
            }
            RuleType::GlobContentMatches { glob, pattern } => {
                format!("Ensure files matching '{}' contain pattern '{}'", glob, pattern)
            }
            RuleType::GlobContentNotMatches { glob, pattern, .. } => {
                format!("Remove content matching '{}' from files matching '{}'", pattern, glob)
            }
            RuleType::GlobNamingMatches { glob, pattern } => {
                format!("Rename files matching '{}' to conform to pattern '{}'", glob, pattern)
            }
            RuleType::GlobNamingNotMatches { glob, pattern, .. } => {
                format!("Rename files matching '{}' so they no longer match pattern '{}'", glob, pattern)
            }
            RuleType::Builtin { handler } => {
                format!("Fix the issue detected by builtin check '{}'", handler)
            }
            RuleType::CargoKeyExists { key } => format!("Add key '{}' to Cargo.toml", key),
            RuleType::CargoKeyMatches { key, pattern } => {
                format!("Update key '{}' in Cargo.toml to match pattern '{}'", key, pattern)
            }
        }
    }
}

/// Parsed Cargo.toml manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoManifest {
    /// Full parsed TOML value.
    #[serde(skip)]
    pub raw: Option<toml::Value>,
    /// Package name from `[package]`.
    pub package_name: Option<String>,
    /// Whether `[lib]` section exists.
    pub has_lib: bool,
    /// The `[lib].path` value if set.
    pub lib_path: Option<String>,
    /// Binary targets from `[[bin]]`.
    pub bins: Vec<BinTarget>,
    /// Test targets from `[[test]]`.
    pub tests: Vec<TestTarget>,
    /// Bench targets from `[[bench]]`.
    pub benches: Vec<BenchTarget>,
    /// Example targets from `[[example]]`.
    pub examples: Vec<ExampleTarget>,
    /// Whether `[workspace]` section exists.
    pub has_workspace: bool,
    /// The `package.edition` value if set.
    pub edition: Option<String>,
    /// Workspace member paths from `[workspace] members = [...]`.
    pub workspace_members: Vec<String>,
}

/// A binary target from `[[bin]]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinTarget {
    /// Target name.
    pub name: String,
    /// Source path.
    pub path: Option<String>,
}

/// A test target from `[[test]]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTarget {
    /// Target name.
    pub name: String,
    /// Source path.
    pub path: Option<String>,
}

/// A bench target from `[[bench]]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchTarget {
    /// Target name.
    pub name: String,
    /// Whether harness is disabled.
    pub harness: Option<bool>,
}

/// An example target from `[[example]]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleTarget {
    /// Target name.
    pub name: String,
    /// Source path.
    pub path: Option<String>,
}
