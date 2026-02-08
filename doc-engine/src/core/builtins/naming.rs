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
