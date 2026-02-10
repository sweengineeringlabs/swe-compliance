use std::path::PathBuf;
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
