/// A spec file discovered in the project (FR-1000).
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
            path: value.get_str("path").unwrap_or_default().into(),
            name: value.get_str("name").unwrap_or_default().into(),
            kind: value.get_str("kind").unwrap_or_default().into(),
            size: value.get_u64("size").unwrap_or(0),
            modified: value.get_str("modified").unwrap_or_default().into(),
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
pub struct SpecDirectory {
    pub name: String,
    pub path: String,
    pub files: Vec<SpecFile>,
    pub children: Vec<SpecDirectory>,
}

impl SpecDirectory {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let files = value.get_array("files").unwrap_or_default().iter().filter_map(|v| SpecFile::from_json(v)).collect();
        let children = value.get_array("children").unwrap_or_default().iter().filter_map(|v| SpecDirectory::from_json(v)).collect();
        Some(SpecDirectory {
            name: value.get_str("name").unwrap_or_default().into(),
            path: value.get_str("path").unwrap_or_default().into(),
            files,
            children,
        })
    }

    pub fn total_files(&self) -> usize {
        self.files.len() + self.children.iter().map(|c| c.total_files()).sum::<usize>()
    }
}

/// BRD (Business Requirements Document) overview entry (FR-1003).
pub struct BrdEntry {
    pub id: String,
    pub title: String,
    pub status: String,
    pub spec_files: Vec<String>,
}

impl BrdEntry {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(BrdEntry {
            id: value.get_str("id").unwrap_or_default().into(),
            title: value.get_str("title").unwrap_or_default().into(),
            status: value.get_str("status").unwrap_or("unknown").into(),
            spec_files: value.get_array("spec_files").unwrap_or_default().iter().filter_map(|v| v.as_str().map(|s| s.into())).collect(),
        })
    }
}
