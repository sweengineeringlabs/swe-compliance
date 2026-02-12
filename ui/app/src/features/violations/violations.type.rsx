/// A single violation extracted from a completed scan report.
/// Maps directly to the server-side `ViolationEntry` returned by
/// `GET /api/v1/scans/{id}/violations?format=json`.
pub struct ViolationEntry {
    pub check_id: u32,
    pub category: String,
    pub description: String,
    pub severity: String,
    pub file_path: Option<String>,
    pub message: String,
}

/// Client-side filter state applied to the violations list.
pub struct ViolationFilter {
    /// Selected category slug, or None for "all categories".
    pub category: Option<String>,
    /// Active severity levels (e.g. ["Error", "Warning"]).
    pub severities: Vec<String>,
    /// Free-text search applied to description and message fields.
    pub search: String,
}

impl Default for ViolationFilter {
    fn default() -> Self {
        Self {
            category: None,
            severities: vec![
                "Error".into(),
                "Warning".into(),
                "Info".into(),
            ],
            search: String::new(),
        }
    }
}

/// Column by which the violations table can be sorted.
#[derive(Clone, PartialEq)]
pub enum SortField {
    CheckId,
    Severity,
    Category,
    FilePath,
}

impl Default for SortField {
    fn default() -> Self {
        SortField::Severity
    }
}

/// Sort direction.
#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

/// The 18 doc-engine compliance categories.
pub const CATEGORIES: &[&str] = &[
    "adr",
    "backlog",
    "content",
    "cross_ref",
    "deployment",
    "design",
    "development",
    "ideation",
    "module",
    "naming",
    "navigation",
    "operations",
    "planning",
    "requirements",
    "root_files",
    "structure",
    "testing",
    "traceability",
];

/// Map a category slug to a human-readable label.
pub fn category_label(slug: &str) -> String {
    slug.replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{rest}", rest = chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Numeric weight for severity ordering (lower = more severe).
pub fn severity_weight(severity: &str) -> u8 {
    match severity {
        "Error" => 0,
        "Warning" => 1,
        "Info" => 2,
        _ => 3,
    }
}
