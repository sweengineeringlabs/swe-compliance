use std::fs;
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::api::traits::CheckRunner;
use crate::api::types::{CheckId, CheckResult, ScanContext, Violation};

static ADR_NAMING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{3}-[a-z0-9_-]+\.md$").unwrap());
static ADR_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{3}-").unwrap());

/// Check 49: adr_naming
/// ADR files follow NNN-title.md naming convention
pub struct AdrNaming {
    pub def: RuleDef,
}

impl CheckRunner for AdrNaming {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let adr_dir = ctx.root.join("docs/3-design/adr");
        if !adr_dir.is_dir() {
            return CheckResult::Skip { reason: "ADR directory does not exist".to_string() };
        }

        let adr_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.starts_with("docs/3-design/adr/") && s.ends_with(".md")
            })
            .collect();

        if adr_files.is_empty() {
            return CheckResult::Skip { reason: "No ADR files found".to_string() };
        }

        let mut violations = Vec::new();
        for file in &adr_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            // Skip README.md and index files
            if filename == "README.md" || filename == "index.md" {
                continue;
            }

            if !ADR_NAMING_RE.is_match(&filename) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(file.to_path_buf()),
                    message: format!(
                        "ADR file '{}' doesn't follow NNN-title.md naming convention",
                        filename
                    ),
                    severity: self.def.severity.clone(),
                });
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 50: adr_index_completeness
/// Cross-reference ADR index against ADR files
pub struct AdrIndexCompleteness {
    pub def: RuleDef,
}

impl CheckRunner for AdrIndexCompleteness {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let adr_dir = ctx.root.join("docs/3-design/adr");
        if !adr_dir.is_dir() {
            return CheckResult::Skip { reason: "ADR directory does not exist".to_string() };
        }

        // Look for an index file
        let index_path = if adr_dir.join("README.md").exists() {
            adr_dir.join("README.md")
        } else if adr_dir.join("index.md").exists() {
            adr_dir.join("index.md")
        } else {
            return CheckResult::Skip { reason: "No ADR index file found".to_string() };
        };

        let index_content = match fs::read_to_string(&index_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read ADR index: {}", e),
                };
            }
        };

        // Collect actual ADR files (excluding index/readme)
        let adr_files: Vec<String> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.starts_with("docs/3-design/adr/") && s.ends_with(".md")
            })
            .filter_map(|f| {
                let filename = f.file_name()?.to_string_lossy().to_string();
                if ADR_PREFIX_RE.is_match(&filename) {
                    Some(filename)
                } else {
                    None
                }
            })
            .collect();

        if adr_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for adr_file in &adr_files {
            if !index_content.contains(adr_file.as_str()) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/3-design/adr".into()),
                    message: format!("ADR '{}' not referenced in index", adr_file),
                    severity: self.def.severity.clone(),
                });
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{RuleDef, RuleType};
    use crate::api::types::{ProjectScope, ProjectType, Severity};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn make_def(id: u8) -> RuleDef {
        RuleDef {
            id,
            category: "adr".to_string(),
            description: "test".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: "test".to_string() },
            project_type: None,
            scope: None,
        }
    }

    fn make_ctx(root: &std::path::Path, files: Vec<PathBuf>) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_type: ProjectType::OpenSource,
            project_scope: ProjectScope::Large,
        }
    }

    // --- AdrNaming (check 49) ---

    #[test]
    fn test_adr_naming_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/3-design/adr")).unwrap();
        fs::write(tmp.path().join("docs/3-design/adr/001-use-rust.md"), "# ADR").unwrap();
        let handler = AdrNaming { def: make_def(49) };
        let files = vec![PathBuf::from("docs/3-design/adr/001-use-rust.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_adr_naming_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/3-design/adr")).unwrap();
        fs::write(tmp.path().join("docs/3-design/adr/bad_name.md"), "# ADR").unwrap();
        let handler = AdrNaming { def: make_def(49) };
        let files = vec![PathBuf::from("docs/3-design/adr/bad_name.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_adr_naming_skip_no_dir() {
        let tmp = TempDir::new().unwrap();
        let handler = AdrNaming { def: make_def(49) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- AdrIndexCompleteness (check 50) ---

    #[test]
    fn test_adr_index_pass() {
        let tmp = TempDir::new().unwrap();
        let adr_dir = tmp.path().join("docs/3-design/adr");
        fs::create_dir_all(&adr_dir).unwrap();
        fs::write(adr_dir.join("README.md"), "# ADR Index\n- [001-use-rust.md](001-use-rust.md)\n").unwrap();
        fs::write(adr_dir.join("001-use-rust.md"), "# ADR").unwrap();
        let handler = AdrIndexCompleteness { def: make_def(50) };
        let files = vec![
            PathBuf::from("docs/3-design/adr/README.md"),
            PathBuf::from("docs/3-design/adr/001-use-rust.md"),
        ];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_adr_index_fail_missing_ref() {
        let tmp = TempDir::new().unwrap();
        let adr_dir = tmp.path().join("docs/3-design/adr");
        fs::create_dir_all(&adr_dir).unwrap();
        fs::write(adr_dir.join("README.md"), "# ADR Index\nNothing listed\n").unwrap();
        fs::write(adr_dir.join("001-use-rust.md"), "# ADR").unwrap();
        let handler = AdrIndexCompleteness { def: make_def(50) };
        let files = vec![
            PathBuf::from("docs/3-design/adr/README.md"),
            PathBuf::from("docs/3-design/adr/001-use-rust.md"),
        ];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_adr_index_skip_no_index() {
        let tmp = TempDir::new().unwrap();
        let adr_dir = tmp.path().join("docs/3-design/adr");
        fs::create_dir_all(&adr_dir).unwrap();
        fs::write(adr_dir.join("001-use-rust.md"), "# ADR").unwrap();
        let handler = AdrIndexCompleteness { def: make_def(50) };
        let files = vec![PathBuf::from("docs/3-design/adr/001-use-rust.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }
}
