use rsc_compat::prelude::*;

/// Validation result for SRS content (FR-901).
#[derive(Clone)]
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
            valid: value.get("valid")?.as_bool().unwrap_or_default(),
            domain_count: value.get("domain_count")?.as_u64().unwrap_or(0) as u32,
            requirement_count: value.get("requirement_count")?.as_u64().unwrap_or(0) as u32,
            errors: value
                .get("errors")
                .and_then(|v| v.as_array())
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| ValidationError::from_json(v))
                .collect(),
            warnings: value
                .get("warnings")
                .and_then(|v| v.as_array())
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.into()))
                .collect(),
        })
    }
}

#[derive(Clone)]
pub struct ValidationError {
    pub line: u32,
    pub message: String,
}

impl ValidationError {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ValidationError {
            line: value.get("line")?.as_u64().unwrap_or_default() as u32,
            message: value.get("message")?.as_str().unwrap_or_default().into(),
        })
    }
}

/// Saved SRS document content for a project (FR-903).
#[derive(Clone)]
pub struct SrsDocument {
    pub project_id: String,
    pub content: String,
    pub updated_at: String,
}

impl SrsDocument {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(SrsDocument {
            project_id: value.get("project_id")?.as_str().unwrap_or_default().into(),
            content: value.get("content")?.as_str().unwrap_or_default().into(),
            updated_at: value.get("updated_at")?.as_str().unwrap_or_default().into(),
        })
    }
}

/// A suggested FR ID for auto-completion (FR-902).
#[derive(Clone)]
pub struct FrIdSuggestion {
    pub id: String,
    pub title: String,
    pub category: String,
}
