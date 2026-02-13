use rsc_compat::prelude::*;

/// A spec file discovered in the project (FR-1000).
#[derive(Clone, Default)]
pub struct SpecFile {
    pub path: String,
    pub name: String,
    pub kind: String,
    pub size: u64,
    pub modified: String,
}

impl SpecFile {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(SpecFile {
            path: value.get("path")?.as_str().unwrap_or_default().into(),
            name: value.get("name")?.as_str().unwrap_or_default().into(),
            kind: value.get("kind")?.as_str().unwrap_or_default().into(),
            size: value.get("size")?.as_u64().unwrap_or(0),
            modified: value.get("modified")?.as_str().unwrap_or_default().into(),
        })
    }

    pub fn kind_label(&self) -> &str {
        match self.kind.as_str() {
            "spec" => "Specification",
            "arch" => "Architecture",
            "test" => "Test Plan",
            "deploy" => "Deployment",
            _ => "Other",
        }
    }

    pub fn kind_variant(&self) -> &str {
        match self.kind.as_str() {
            "spec" => "primary",
            "arch" => "info",
            "test" => "warning",
            "deploy" => "success",
            _ => "secondary",
        }
    }
}

/// A directory node in the spec tree (FR-1001).
#[derive(Clone, Default)]
pub struct SpecDirectory {
    pub name: String,
    pub path: String,
    pub files: Vec<SpecFile>,
    pub children: Vec<SpecDirectory>,
}

impl SpecDirectory {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let files = value
            .get("files")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| SpecFile::from_json(v))
            .collect();
        let children = value
            .get("children")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| SpecDirectory::from_json(v))
            .collect();
        Some(SpecDirectory {
            name: value.get("name")?.as_str().unwrap_or_default().into(),
            path: value.get("path")?.as_str().unwrap_or_default().into(),
            files,
            children,
        })
    }

    pub fn total_files(&self) -> usize {
        self.files.len() + self.children.iter().map(|c| c.total_files()).sum::<usize>()
    }
}

/// BRD (Business Requirements Document) overview entry (FR-1003).
#[derive(Clone)]
pub struct BrdEntry {
    pub id: String,
    pub title: String,
    pub status: String,
    pub spec_files: Vec<String>,
}

impl BrdEntry {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(BrdEntry {
            id: value.get("id")?.as_str().unwrap_or_default().into(),
            title: value.get("title")?.as_str().unwrap_or_default().into(),
            status: value.get("status")?.as_str().unwrap_or("unknown").into(),
            spec_files: value
                .get("spec_files")
                .and_then(|v| v.as_array())
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.into()))
                .collect(),
        })
    }
}
