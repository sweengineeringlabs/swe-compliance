use std::fmt;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Serialize, Deserialize};

/// The SDLC role kind of a requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReqKind {
    /// A functional requirement (FR-xxx).
    Functional,
    /// A non-functional requirement (NFR-xxx).
    NonFunctional,
}

/// A single FR-xxx or NFR-xxx block extracted from the SRS.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SrsRequirement {
    /// Requirement identifier, e.g. "FR-100".
    pub id: String,
    /// Requirement title, e.g. "Default rules embedded in binary".
    pub title: String,
    /// Whether this is functional or non-functional.
    pub kind: ReqKind,
    /// MoSCoW priority: Must / Should / Could / Won't.
    pub priority: Option<String>,
    /// Lifecycle state: Proposed / Approved / Implemented / Verified.
    pub state: Option<String>,
    /// Verification method: Test / Inspection / Analysis / Demonstration.
    pub verification: Option<String>,
    /// Traceability reference to stakeholder requirements or code paths.
    pub traces_to: Option<String>,
    /// Acceptance criteria text.
    pub acceptance: Option<String>,
    /// Narrative description text after the attribute table.
    pub description: String,
}

/// A domain derived from a `### X.Y Title` section heading in the SRS.
#[derive(Debug, Clone)]
pub struct SrsDomain {
    /// Section number, e.g. "4.1".
    pub section: String,
    /// Section title, e.g. "Rule Loading".
    pub title: String,
    /// Slugified title, e.g. "rule_loading".
    pub slug: String,
    /// Requirements found in this section.
    pub requirements: Vec<SrsRequirement>,
}

/// Configuration for the scaffold operation.
pub struct ScaffoldConfig {
    /// Path to the SRS markdown file.
    pub srs_path: PathBuf,
    /// Output directory (spec files are placed under this root).
    pub output_dir: PathBuf,
    /// Overwrite existing files when true.
    pub force: bool,
    /// Optional phase filter: only generate files for these SDLC phases.
    /// Valid values: "requirements", "design", "testing", "deployment".
    /// When empty/None, all phases are generated.
    pub phases: Vec<String>,
    /// Optional file-type filter: only generate files of these types.
    /// Valid values: "yaml", "spec", "arch", "test", "exec", "deploy".
    /// When empty, all file types are generated.
    pub file_types: Vec<String>,
}

/// Result of a scaffold operation (ISO/IEC/IEEE 15289:2019 clause 9).
#[derive(Debug, Serialize, Deserialize)]
pub struct ScaffoldResult {
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
    /// Canonicalized absolute path to the SRS source file.
    pub srs_source: String,
    /// Phase filter applied (empty = all phases).
    pub phases: Vec<String>,
    /// Whether `--force` was set during scaffold.
    pub force: bool,
    /// Number of domains processed.
    pub domain_count: usize,
    /// Total number of requirements extracted.
    pub requirement_count: usize,
    /// Files that were created.
    pub created: Vec<PathBuf>,
    /// Files that were skipped (already existed and --force not set).
    pub skipped: Vec<PathBuf>,
}

/// Error type for scaffold operations.
#[derive(Debug)]
pub enum ScaffoldError {
    /// An I/O error encountered during scaffolding.
    Io(io::Error),
    /// The supplied path is invalid or does not exist.
    Path(String),
    /// A configuration or parsing error.
    Parse(String),
}

impl fmt::Display for ScaffoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaffoldError::Io(e) => write!(f, "IO error: {}", e),
            ScaffoldError::Path(s) => write!(f, "Path error: {}", s),
            ScaffoldError::Parse(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for ScaffoldError {}

impl From<io::Error> for ScaffoldError {
    fn from(e: io::Error) -> Self {
        ScaffoldError::Io(e)
    }
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
