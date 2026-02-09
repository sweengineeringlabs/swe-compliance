use std::collections::HashSet;
use std::fs;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

/// Checks 4-5: module_docs_plural
/// Check 4: All module doc folders use docs/ (plural), not doc/
/// Check 5: No module has both doc/ and docs/
pub struct ModuleDocsPlural {
    pub def: RuleDef,
}

impl CheckRunner for ModuleDocsPlural {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let mut doc_dirs: HashSet<String> = HashSet::new();
        let mut docs_dirs: HashSet<String> = HashSet::new();

        for file in &ctx.files {
            let path_str = file.to_string_lossy();
            let components: Vec<&str> = path_str.split('/').collect();
            for (i, comp) in components.iter().enumerate() {
                if *comp == "doc" && i > 0 {
                    // Found a doc/ directory - record its parent
                    let parent = components[..i].join("/");
                    doc_dirs.insert(parent);
                } else if *comp == "docs" && i > 0 {
                    let parent = components[..i].join("/");
                    docs_dirs.insert(parent);
                }
            }
        }

        match self.def.id {
            4 => {
                // Check that no module uses doc/ (singular)
                if doc_dirs.is_empty() {
                    CheckResult::Pass
                } else {
                    let violations: Vec<Violation> = doc_dirs.iter().map(|parent| {
                        Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(format!("{}/doc", parent).into()),
                            message: format!("Module '{}' uses doc/ (singular); should use docs/", parent),
                            severity: self.def.severity.clone(),
                        }
                    }).collect();
                    CheckResult::Fail { violations }
                }
            }
            5 => {
                // Check no module has both doc/ and docs/
                let both: Vec<_> = doc_dirs.intersection(&docs_dirs).collect();
                if both.is_empty() {
                    CheckResult::Pass
                } else {
                    let violations: Vec<Violation> = both.iter().map(|parent| {
                        Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(parent.as_str().into()),
                            message: format!("Module '{}' has both doc/ and docs/", parent),
                            severity: self.def.severity.clone(),
                        }
                    }).collect();
                    CheckResult::Fail { violations }
                }
            }
            _ => CheckResult::Pass,
        }
    }
}

/// Checks 9-10: sdlc_phase_numbering
/// Check 9: SDLC phase numbering correct (0-7 prefix)
/// Check 10: Phases in correct order
pub struct SdlcPhaseNumbering {
    pub def: RuleDef,
}

impl CheckRunner for SdlcPhaseNumbering {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let docs_path = ctx.root.join("docs");
        if !docs_path.is_dir() {
            return CheckResult::Skip { reason: "docs/ directory does not exist".to_string() };
        }

        let phase_re = Regex::new(r"^(\d+)-").unwrap();
        let mut phase_dirs: Vec<(u8, String)> = Vec::new();

        if let Ok(entries) = fs::read_dir(&docs_path) {
            for entry in entries.flatten() {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if let Some(caps) = phase_re.captures(&name) {
                        if let Ok(num) = caps[1].parse::<u8>() {
                            phase_dirs.push((num, name));
                        }
                    }
                }
            }
        }

        phase_dirs.sort_by_key(|(num, _)| *num);

        match self.def.id {
            9 => {
                // Check numbering: each phase should be 0-7
                let mut violations = Vec::new();
                for (num, name) in &phase_dirs {
                    if *num > 7 {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(format!("docs/{}", name).into()),
                            message: format!("Phase directory '{}' has number > 7", name),
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
            10 => {
                // Check ordering: phases should be in ascending order
                let mut violations = Vec::new();
                for i in 1..phase_dirs.len() {
                    if phase_dirs[i].0 <= phase_dirs[i - 1].0 {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(format!("docs/{}", phase_dirs[i].1).into()),
                            message: format!(
                                "Phase '{}' is out of order (follows '{}')",
                                phase_dirs[i].1, phase_dirs[i - 1].1
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
            _ => CheckResult::Pass,
        }
    }
}

/// Check 8: checklist_completeness
/// Verify every enforceable rule in compliance checklist has a checkbox
pub struct ChecklistCompleteness {
    pub def: RuleDef,
}

impl CheckRunner for ChecklistCompleteness {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let checklist_path = ctx.root.join("docs/3-design/compliance/compliance_checklist.md");
        if !checklist_path.exists() {
            return CheckResult::Skip { reason: "Compliance checklist not found".to_string() };
        }

        let content = match fs::read_to_string(&checklist_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read checklist: {}", e),
                };
            }
        };

        // Count checkboxes (both checked and unchecked)
        let checkbox_re = Regex::new(r"- \[([ xX])\]").unwrap();
        let checkbox_count = checkbox_re.find_iter(&content).count();

        // Simple heuristic: a valid checklist should have many checkboxes
        if checkbox_count >= 10 {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/3-design/compliance/compliance_checklist.md".into()),
                    message: format!(
                        "Checklist has only {} checkboxes; expected comprehensive coverage",
                        checkbox_count
                    ),
                    severity: self.def.severity.clone(),
                }],
            }
        }
    }
}

/// Check 31: open_source_community_files
/// CODE_OF_CONDUCT.md and SUPPORT.md exist (open-source only)
pub struct OpenSourceCommunityFiles {
    pub def: RuleDef,
}

impl CheckRunner for OpenSourceCommunityFiles {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let mut violations = Vec::new();

        for file in &["CODE_OF_CONDUCT.md", "SUPPORT.md"] {
            if !ctx.root.join(file).exists() {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some((*file).into()),
                    message: format!("{} does not exist", file),
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

/// Check 32: open_source_github_templates
/// .github/ISSUE_TEMPLATE/ and PULL_REQUEST_TEMPLATE.md exist (open-source only)
pub struct OpenSourceGithubTemplates {
    pub def: RuleDef,
}

impl CheckRunner for OpenSourceGithubTemplates {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let mut violations = Vec::new();

        if !ctx.root.join(".github/ISSUE_TEMPLATE").is_dir() {
            violations.push(Violation {
                check_id: CheckId(self.def.id),
                path: Some(".github/ISSUE_TEMPLATE".into()),
                message: ".github/ISSUE_TEMPLATE/ directory does not exist".to_string(),
                severity: self.def.severity.clone(),
            });
        }

        if !ctx.root.join(".github/PULL_REQUEST_TEMPLATE.md").exists() {
            violations.push(Violation {
                check_id: CheckId(self.def.id),
                path: Some(".github/PULL_REQUEST_TEMPLATE.md".into()),
                message: ".github/PULL_REQUEST_TEMPLATE.md does not exist".to_string(),
                severity: self.def.severity.clone(),
            });
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 73: templates_populated
/// If docs/templates/ exists, verify it contains >=1 file.
pub struct TemplatesPopulated {
    pub def: RuleDef,
}

impl CheckRunner for TemplatesPopulated {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let templates_dir = ctx.root.join("docs/templates");
        if !templates_dir.is_dir() {
            return CheckResult::Skip { reason: "docs/templates/ does not exist".to_string() };
        }

        let has_files = ctx.files.iter().any(|f| {
            let s = f.to_string_lossy();
            s.starts_with("docs/templates/") && s.ends_with(".md")
        });

        if has_files {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/templates".into()),
                    message: "docs/templates/ exists but contains no template files".to_string(),
                    severity: self.def.severity.clone(),
                }],
            }
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

    fn make_def(id: u8, handler: &str) -> RuleDef {
        RuleDef {
            id,
            category: "structure".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::Builtin { handler: handler.to_string() },
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

    // --- ModuleDocsPlural (checks 4, 5) ---

    #[test]
    fn test_module_docs_plural_pass() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleDocsPlural { def: make_def(4, "module_docs_plural") };
        // No doc/ dirs, only docs/ is fine
        let files = vec![PathBuf::from("modules/auth/docs/README.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_docs_plural_fail_singular() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleDocsPlural { def: make_def(4, "module_docs_plural") };
        let files = vec![PathBuf::from("modules/auth/doc/README.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- SdlcPhaseNumbering (checks 9, 10) ---

    #[test]
    fn test_sdlc_numbering_pass() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("0-overview")).unwrap();
        fs::create_dir_all(docs.join("1-requirements")).unwrap();
        fs::create_dir_all(docs.join("3-design")).unwrap();
        let handler = SdlcPhaseNumbering { def: make_def(9, "sdlc_phase_numbering") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_sdlc_numbering_fail_high() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("9-extra")).unwrap();
        let handler = SdlcPhaseNumbering { def: make_def(9, "sdlc_phase_numbering") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_sdlc_numbering_skip_no_docs() {
        let tmp = TempDir::new().unwrap();
        let handler = SdlcPhaseNumbering { def: make_def(9, "sdlc_phase_numbering") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_sdlc_ordering_pass() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("0-overview")).unwrap();
        fs::create_dir_all(docs.join("1-requirements")).unwrap();
        fs::create_dir_all(docs.join("3-design")).unwrap();
        let handler = SdlcPhaseNumbering { def: make_def(10, "sdlc_phase_numbering") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_sdlc_ordering_fail_duplicate() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("1-requirements")).unwrap();
        fs::create_dir_all(docs.join("1-also_requirements")).unwrap();
        let handler = SdlcPhaseNumbering { def: make_def(10, "sdlc_phase_numbering") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- ChecklistCompleteness (check 8) ---

    #[test]
    fn test_checklist_pass() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("docs/3-design/compliance");
        fs::create_dir_all(&dir).unwrap();
        let content = (0..15).map(|i| format!("- [x] Rule {}", i)).collect::<Vec<_>>().join("\n");
        fs::write(dir.join("compliance_checklist.md"), &content).unwrap();
        let handler = ChecklistCompleteness { def: make_def(8, "checklist_completeness") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_checklist_fail_few_checkboxes() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("docs/3-design/compliance");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("compliance_checklist.md"), "- [x] one\n- [ ] two\n").unwrap();
        let handler = ChecklistCompleteness { def: make_def(8, "checklist_completeness") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_checklist_skip_missing() {
        let tmp = TempDir::new().unwrap();
        let handler = ChecklistCompleteness { def: make_def(8, "checklist_completeness") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- OpenSourceCommunityFiles (check 31) ---

    #[test]
    fn test_community_files_pass() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("CODE_OF_CONDUCT.md"), "conduct").unwrap();
        fs::write(tmp.path().join("SUPPORT.md"), "support").unwrap();
        let handler = OpenSourceCommunityFiles { def: make_def(31, "open_source_community_files") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_community_files_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = OpenSourceCommunityFiles { def: make_def(31, "open_source_community_files") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- OpenSourceGithubTemplates (check 32) ---

    #[test]
    fn test_github_templates_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".github/ISSUE_TEMPLATE")).unwrap();
        fs::write(tmp.path().join(".github/PULL_REQUEST_TEMPLATE.md"), "template").unwrap();
        let handler = OpenSourceGithubTemplates { def: make_def(32, "open_source_github_templates") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_github_templates_fail() {
        let tmp = TempDir::new().unwrap();
        let handler = OpenSourceGithubTemplates { def: make_def(32, "open_source_github_templates") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- TemplatesPopulated (check 73) ---

    #[test]
    fn test_templates_populated_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/templates")).unwrap();
        fs::write(tmp.path().join("docs/templates/check_template.md"), "# Template").unwrap();
        let handler = TemplatesPopulated { def: make_def(73, "templates_populated") };
        let files = vec![PathBuf::from("docs/templates/check_template.md")];
        let ctx = make_ctx(tmp.path(), files);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_templates_populated_skip_no_dir() {
        let tmp = TempDir::new().unwrap();
        let handler = TemplatesPopulated { def: make_def(73, "templates_populated") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }
}
