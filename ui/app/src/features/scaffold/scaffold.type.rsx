/// A single requirement parsed from an SRS domain section.
pub struct ParsedRequirement {
    /// Unique identifier for the requirement (e.g. "REQ-AUTH-001").
    pub id: String,
    /// Human-readable title of the requirement.
    pub title: String,
    /// Kind of requirement: functional, non-functional, constraint.
    pub kind: String,
    /// Full description text from the SRS.
    pub description: String,
}

/// A domain section parsed from the SRS document, containing its requirements.
pub struct ParsedDomain {
    /// Section number or heading prefix (e.g. "3.1").
    pub section: String,
    /// Domain title (e.g. "Authentication & Authorization").
    pub title: String,
    /// URL-safe slug derived from the title.
    pub slug: String,
    /// Requirements belonging to this domain.
    pub requirements: Vec<ParsedRequirement>,
}

impl ParsedDomain {
    /// Returns the number of requirements in this domain.
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }
}

/// Request payload to execute the scaffold generation.
pub struct ScaffoldRequest {
    /// Path to the SRS markdown file or raw content identifier.
    pub srs_path: String,
    /// Output directory for generated SDLC artifacts.
    pub output_dir: String,
    /// SDLC phases to include in generation.
    pub phases: Vec<String>,
    /// File types to generate (yaml, spec, arch, test, exec, deploy).
    pub file_types: Vec<String>,
    /// Whether to overwrite existing files.
    pub force: bool,
}

/// Result returned after scaffold execution completes.
pub struct ScaffoldResult {
    /// Number of domains processed.
    pub domain_count: u32,
    /// Number of requirements processed.
    pub requirement_count: u32,
    /// List of file paths that were created.
    pub created: Vec<String>,
    /// List of file paths that were skipped (already existed).
    pub skipped: Vec<String>,
}

impl ScaffoldResult {
    /// Total number of files that were processed (created + skipped).
    pub fn total_files(&self) -> usize {
        self.created.len() + self.skipped.len()
    }

    /// Returns the creation rate as a percentage.
    pub fn created_percentage(&self) -> f64 {
        let total = self.total_files();
        if total == 0 {
            0.0
        } else {
            (self.created.len() as f64 / total as f64) * 100.0
        }
    }
}
