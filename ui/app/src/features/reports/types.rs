use rsc_compat::prelude::*;

/// Supported report export formats (FR-701).
#[derive(Clone, Debug, PartialEq)]
pub enum ReportFormat {
    Json,
    Markdown,
    Pdf,
}

impl ReportFormat {
    pub fn as_str(&self) -> &str {
        match self {
            ReportFormat::Json => "json",
            ReportFormat::Markdown => "markdown",
            ReportFormat::Pdf => "pdf",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ReportFormat::Json => "JSON",
            ReportFormat::Markdown => "Markdown",
            ReportFormat::Pdf => "PDF",
        }
    }

    /// Parse a format identifier from a string. Returns None for unknown values.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "json" => Some(ReportFormat::Json),
            "markdown" => Some(ReportFormat::Markdown),
            "pdf" => Some(ReportFormat::Pdf),
            _ => None,
        }
    }
}

/// Generated report data from the server.
#[derive(Clone, Debug)]
pub struct ReportData {
    pub scan_id: String,
    pub format: String,
    pub content: String,
    pub generated_at: String,
}

impl ReportData {
    /// Parse a ReportData from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ReportData {
            scan_id: value.get_str("scan_id").unwrap_or_default().into(),
            format: value.get_str("format").unwrap_or_default().into(),
            content: value.get_str("content").unwrap_or_default().into(),
            generated_at: value.get_str("generated_at").unwrap_or_default().into(),
        })
    }
}

/// Comparison between two scan reports (FR-703).
#[derive(Clone, Debug)]
pub struct ReportComparison {
    pub scan_a_id: String,
    pub scan_b_id: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: u32,
}

impl ReportComparison {
    /// Parse a ReportComparison from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let added = value.get("added")
            .and_then(|v| v.as_array())
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.into()))
            .collect();
        let removed = value.get("removed")
            .and_then(|v| v.as_array())
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.into()))
            .collect();
        Some(ReportComparison {
            scan_a_id: value.get_str("scan_a_id").unwrap_or_default().into(),
            scan_b_id: value.get_str("scan_b_id").unwrap_or_default().into(),
            added,
            removed,
            unchanged: value.get_u32("unchanged").unwrap_or(0),
        })
    }
}
