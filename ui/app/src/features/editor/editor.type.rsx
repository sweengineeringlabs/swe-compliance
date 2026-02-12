/// Validation result for SRS content (FR-901).
pub struct ValidationResult {
    pub valid: bool,
    pub domain_count: u32,
    pub requirement_count: u32,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ValidationResult {
            valid: value.get_bool("valid")?,
            domain_count: value.get_u32("domain_count").unwrap_or(0),
            requirement_count: value.get_u32("requirement_count").unwrap_or(0),
            errors: value.get_array("errors").unwrap_or_default().iter().filter_map(|v| ValidationError::from_json(v)).collect(),
            warnings: value.get_array("warnings").unwrap_or_default().iter().filter_map(|v| v.as_str().map(|s| s.into())).collect(),
        })
    }
}

pub struct ValidationError {
    pub line: u32,
    pub message: String,
}

impl ValidationError {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ValidationError {
            line: value.get_u32("line")?,
            message: value.get_str("message")?.into(),
        })
    }
}

/// Saved SRS document content for a project (FR-903).
pub struct SrsDocument {
    pub project_id: String,
    pub content: String,
    pub updated_at: String,
}

impl SrsDocument {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(SrsDocument {
            project_id: value.get_str("project_id")?.into(),
            content: value.get_str("content")?.into(),
            updated_at: value.get_str("updated_at").unwrap_or_default().into(),
        })
    }
}

/// A suggested FR ID for auto-completion (FR-902).
pub struct FrIdSuggestion {
    pub id: String,
    pub title: String,
    pub category: String,
}
