use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

/// Checks 21-23: snake_lower_case
/// 21: All filenames in docs/ are lowercase
/// 22: All filenames in docs/ use underscores, not hyphens
/// 23: No spaces in docs/ filenames
pub struct SnakeLowerCase {
    pub def: RuleDef,
}

impl CheckRunner for SnakeLowerCase {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let docs_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.starts_with("docs/") && s.ends_with(".md")
            })
            .collect();

        if docs_files.is_empty() {
            return CheckResult::Skip { reason: "No .md files in docs/".to_string() };
        }

        // Exclude ADR files (they use NNN-title.md convention) and phase dir names
        let adr_prefix = "docs/3-design/adr/";
        let phase_prefix_re = Regex::new(r"^\d+-").unwrap();

        let mut violations = Vec::new();
        for file in &docs_files {
            let path_str = file.to_string_lossy();
            // Skip ADR files
            if path_str.starts_with(adr_prefix) {
                continue;
            }

            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            // Skip README.md, CHANGELOG.md etc (uppercase convention files)
            if filename == "README.md" || filename == "CHANGELOG.md"
                || filename == "CONTRIBUTING.md" || filename == "SECURITY.md" {
                continue;
            }

            // Skip phase directory prefix names (e.g., the directory names like 0-overview are fine)
            // We only check the filename component
            match self.def.id {
                21 => {
                    // Check lowercase (excluding extension)
                    let stem = filename.trim_end_matches(".md");
                    if stem != stem.to_lowercase() {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(file.to_path_buf()),
                            message: format!("Filename '{}' contains uppercase characters", filename),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                22 => {
                    // Check no hyphens (underscores only)
                    let stem = filename.trim_end_matches(".md");
                    if stem.contains('-') && !phase_prefix_re.is_match(stem) {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(file.to_path_buf()),
                            message: format!("Filename '{}' contains hyphens; use underscores", filename),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                23 => {
                    // Check no spaces
                    if filename.contains(' ') {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(file.to_path_buf()),
                            message: format!("Filename '{}' contains spaces", filename),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                _ => {}
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 24: guide_naming
/// Guide files follow name_{phase}_guide.md convention
pub struct GuideNaming {
    pub def: RuleDef,
}

impl CheckRunner for GuideNaming {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let guide_re = Regex::new(r"^[a-z_]+_[a-z]+_guide\.md$").unwrap();

        let guide_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.contains("guide/") && s.ends_with(".md")
            })
            .collect();

        if guide_files.is_empty() {
            return CheckResult::Skip { reason: "No guide files found".to_string() };
        }

        let mut violations = Vec::new();
        for file in &guide_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if filename == "README.md" {
                continue;
            }

            if !guide_re.is_match(&filename) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(file.to_path_buf()),
                    message: format!(
                        "Guide file '{}' doesn't follow name_{{phase}}_guide.md convention",
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

/// Check 25: testing_file_placement
/// No *_testing_* files outside 5-testing/
pub struct TestingFilePlacement {
    pub def: RuleDef,
}

impl CheckRunner for TestingFilePlacement {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let testing_re = Regex::new(r"_testing_").unwrap();

        let mut violations = Vec::new();
        for file in &ctx.files {
            let path_str = file.to_string_lossy();
            if !path_str.starts_with("docs/") {
                continue;
            }

            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if testing_re.is_match(&filename) && !path_str.contains("5-testing") {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(file.to_path_buf()),
                    message: format!(
                        "Testing file '{}' found outside 5-testing/",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{RuleDef, RuleType};
    use crate::spi::types::{ProjectType, Severity};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn make_def(id: u8) -> RuleDef {
        RuleDef {
            id,
            category: "naming".to_string(),
            description: "test".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: "test".to_string() },
            project_type: None,
        }
    }

    fn make_ctx(root: &std::path::Path, files: Vec<PathBuf>) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_type: ProjectType::OpenSource,
        }
    }

    // --- SnakeLowerCase (checks 21, 22, 23) ---

    #[test]
    fn test_snake_lowercase_pass() {
        let tmp = TempDir::new().unwrap();
        let handler = SnakeLowerCase { def: make_def(21) };
        let files = vec![PathBuf::from("docs/hello_world.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_snake_lowercase_fail_uppercase() {
        let tmp = TempDir::new().unwrap();
        let handler = SnakeLowerCase { def: make_def(21) };
        let files = vec![PathBuf::from("docs/HelloWorld.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_snake_hyphen_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = SnakeLowerCase { def: make_def(22) };
        let files = vec![PathBuf::from("docs/hello-world.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_snake_space_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = SnakeLowerCase { def: make_def(23) };
        let files = vec![PathBuf::from("docs/hello world.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_snake_skip_readme_adr() {
        let tmp = TempDir::new().unwrap();
        let handler = SnakeLowerCase { def: make_def(21) };
        // README.md (uppercase convention) and ADR files should be skipped
        let files = vec![
            PathBuf::from("docs/README.md"),
            PathBuf::from("docs/3-design/adr/001-use-rust.md"),
        ];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // --- GuideNaming (check 24) ---

    #[test]
    fn test_guide_naming_pass() {
        let tmp = TempDir::new().unwrap();
        let handler = GuideNaming { def: make_def(24) };
        let files = vec![PathBuf::from("docs/4-development/guide/api_development_guide.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_guide_naming_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = GuideNaming { def: make_def(24) };
        let files = vec![PathBuf::from("docs/4-development/guide/bad-name.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_guide_naming_skip_no_guides() {
        let tmp = TempDir::new().unwrap();
        let handler = GuideNaming { def: make_def(24) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- TestingFilePlacement (check 25) ---

    #[test]
    fn test_testing_placement_pass() {
        let tmp = TempDir::new().unwrap();
        let handler = TestingFilePlacement { def: make_def(25) };
        let files = vec![PathBuf::from("docs/5-testing/unit_testing_guide.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_testing_placement_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = TestingFilePlacement { def: make_def(25) };
        let files = vec![PathBuf::from("docs/3-design/unit_testing_plan.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }
}
