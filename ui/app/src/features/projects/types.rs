use serde::{Serialize, Deserialize};

/// Types for the Projects feature (FR-100..FR-104).
///
/// These types mirror the API models defined in the backend `api/projects.rs`
/// and are used throughout the projects feature for type-safe data handling.
///
/// Reference: docs/1-requirements/project_management/project_management.spec

/// Project scope determines which compliance rule subset applies.
/// Maps to the `scope` field in API requests and responses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectScope {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}

impl std::fmt::Display for ProjectScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl ProjectScope {
    /// Returns the display label for the scope value.
    pub fn label(&self) -> &'static str {
        match self {
            ProjectScope::Small => "Small",
            ProjectScope::Medium => "Medium",
            ProjectScope::Large => "Large",
        }
    }

    /// Returns the serialized API value for the scope.
    pub fn value(&self) -> &'static str {
        match self {
            ProjectScope::Small => "small",
            ProjectScope::Medium => "medium",
            ProjectScope::Large => "large",
        }
    }

    /// Parses a scope string from the API into the enum variant.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "small" => Some(ProjectScope::Small),
            "medium" => Some(ProjectScope::Medium),
            "large" => Some(ProjectScope::Large),
            _ => None,
        }
    }

    /// Returns all scope variants for populating select dropdowns.
    pub fn all() -> Vec<ProjectScope> {
        vec![ProjectScope::Small, ProjectScope::Medium, ProjectScope::Large]
    }
}

/// Project type determines the compliance profile applied during scans.
/// Maps to the `project_type` field in API requests and responses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectType {
    #[serde(rename = "open_source")]
    OpenSource,
    #[serde(rename = "internal")]
    Internal,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl ProjectType {
    /// Returns the display label for the project type.
    pub fn label(&self) -> &'static str {
        match self {
            ProjectType::OpenSource => "Open Source",
            ProjectType::Internal => "Internal",
        }
    }

    /// Returns the serialized API value for the project type.
    pub fn value(&self) -> &'static str {
        match self {
            ProjectType::OpenSource => "open_source",
            ProjectType::Internal => "internal",
        }
    }

    /// Parses a project type string from the API into the enum variant.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "open_source" => Some(ProjectType::OpenSource),
            "internal" => Some(ProjectType::Internal),
            _ => None,
        }
    }

    /// Returns all project type variants for populating select dropdowns.
    pub fn all() -> Vec<ProjectType> {
        vec![ProjectType::OpenSource, ProjectType::Internal]
    }
}

/// Compliance scan summary for a project.
/// Returned as part of the Project object from `GET /api/v1/projects`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
}

impl ComplianceSummary {
    /// Computes the compliance percentage as passed / total * 100.
    /// Returns 0.0 when total is zero to avoid division by zero.
    pub fn compliance_percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.passed as f64 / self.total as f64) * 100.0
    }
}

/// A compliance project managed by the system.
/// Maps to the JSON object returned by `GET /api/v1/projects` (FR-101).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub root_path: String,
    pub scope: ProjectScope,
    pub project_type: ProjectType,
    pub created_at: String,
    pub updated_at: String,
    pub last_scan_id: Option<String>,
    pub compliance_summary: Option<ComplianceSummary>,
}

/// Request payload for creating a new project via `POST /api/v1/projects` (FR-100).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub root_path: String,
    pub scope: ProjectScope,
    pub project_type: ProjectType,
}

/// Request payload for updating a project via `PATCH /api/v1/projects/{id}` (FR-102).
/// All fields are optional; only provided fields are updated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub scope: Option<ProjectScope>,
    pub project_type: Option<ProjectType>,
}
