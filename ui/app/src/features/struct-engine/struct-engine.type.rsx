/// A struct engine scan check result (FR-1100).
pub struct StructCheck {
    pub check_id: String,
    pub name: String,
    pub category: String,
    pub status: String,
    pub message: String,
    pub file_path: Option<String>,
}

impl StructCheck {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(StructCheck {
            check_id: value.get_str("check_id").unwrap_or_default().into(),
            name: value.get_str("name").unwrap_or_default().into(),
            category: value.get_str("category").unwrap_or_default().into(),
            status: value.get_str("status").unwrap_or_default().into(),
            message: value.get_str("message").unwrap_or_default().into(),
            file_path: value.get_str("file_path").map(|s| s.into()),
        })
    }

    pub fn status_variant(&self) -> &str {
        match self.status.as_str() {
            "pass" => "success",
            "fail" => "danger",
            "skip" => "secondary",
            "warning" => "warning",
            _ => "secondary",
        }
    }
}

/// Crate node in the project layout tree (FR-1101).
pub struct CrateNode {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub children: Vec<CrateNode>,
    pub modules: Vec<String>,
}

impl CrateNode {
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let children = value.get_array("children").unwrap_or_default().iter().filter_map(|v| CrateNode::from_json(v)).collect();
        let modules = value.get_array("modules").unwrap_or_default().iter().filter_map(|v| v.as_str().map(|s| s.into())).collect();
        Some(CrateNode {
            name: value.get_str("name").unwrap_or_default().into(),
            path: value.get_str("path").unwrap_or_default().into(),
            kind: value.get_str("kind").unwrap_or("lib").into(),
            children,
            modules,
        })
    }

    pub fn total_modules(&self) -> usize {
        self.modules.len() + self.children.iter().map(|c| c.total_modules()).sum::<usize>()
    }
}

/// Project kind classification (FR-1102).
pub enum ProjectKind {
    Application,
    Library,
    Workspace,
    Hybrid,
    Unknown,
}

impl ProjectKind {
    pub fn from_str(s: &str) -> Self {
        match s {
            "application" => ProjectKind::Application,
            "library" => ProjectKind::Library,
            "workspace" => ProjectKind::Workspace,
            "hybrid" => ProjectKind::Hybrid,
            _ => ProjectKind::Unknown,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ProjectKind::Application => "Application",
            ProjectKind::Library => "Library",
            ProjectKind::Workspace => "Workspace",
            ProjectKind::Hybrid => "Hybrid",
            ProjectKind::Unknown => "Unknown",
        }
    }

    pub fn badge_variant(&self) -> &str {
        match self {
            ProjectKind::Application => "primary",
            ProjectKind::Library => "info",
            ProjectKind::Workspace => "success",
            ProjectKind::Hybrid => "warning",
            ProjectKind::Unknown => "secondary",
        }
    }
}
