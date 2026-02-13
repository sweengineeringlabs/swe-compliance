/// A template entry from the template engine.
pub struct TemplateEntry {
    /// Display name of the template.
    pub name: String,
    /// Human-readable description of what the template generates.
    pub description: String,
    /// Classification category (e.g. "security", "testing", "deployment").
    pub category: String,
    /// Number of files included in the template.
    pub file_count: u32,
    /// Searchable tags associated with the template.
    pub tags: Vec<String>,
}

impl TemplateEntry {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(TemplateEntry {
            name: value.get_str("name").unwrap_or_default().into(),
            description: value.get_str("description").unwrap_or_default().into(),
            category: value.get_str("category").unwrap_or_default().into(),
            file_count: value.get_u32("file_count").unwrap_or(0),
            tags: value
                .get_array("tags")
                .unwrap_or_default()
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.into()))
                .collect(),
        })
    }
}

/// A compliance checklist item associated with a template.
pub struct ChecklistItem {
    /// Unique identifier for the checklist item.
    pub id: String,
    /// Human-readable label describing the compliance requirement.
    pub label: String,
    /// Whether this item has been satisfied.
    pub checked: bool,
}

impl ChecklistItem {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ChecklistItem {
            id: value.get_str("id").unwrap_or_default().into(),
            label: value.get_str("label").unwrap_or_default().into(),
            checked: value.get_bool("checked").unwrap_or(false),
        })
    }
}

/// Result of copying a template to a project directory.
pub struct TemplateCopyResult {
    /// Name of the template that was copied.
    pub template_name: String,
    /// Target directory the template was copied into.
    pub destination: String,
    /// Number of files that were written to disk.
    pub files_copied: u32,
}

impl TemplateCopyResult {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(TemplateCopyResult {
            template_name: value.get_str("template_name").unwrap_or_default().into(),
            destination: value.get_str("destination").unwrap_or_default().into(),
            files_copied: value.get_u32("files_copied").unwrap_or(0),
        })
    }
}
