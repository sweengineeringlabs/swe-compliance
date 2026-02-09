use std::fs;
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

// SRS 29148 attribute regexes
static SRS_HEADING_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^####\s+((?:FR|NFR)-\d+):\s+.+$").unwrap()
});
static SRS_NEXT_HEADING_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^#{1,4}\s+").unwrap()
});
static ATTR_PRIORITY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*Priority\*\*").unwrap());
static ATTR_STATE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*State\*\*").unwrap());
static ATTR_VERIFICATION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*Verification\*\*").unwrap());
static ATTR_TRACES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Traces\s+to\*\*|\*\*Traceability\*\*").unwrap()
});
static ATTR_ACCEPTANCE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*Acceptance\*\*").unwrap());

// ISO/IEC/IEEE 42010:2022 section regexes
static ARCH_STAKEHOLDERS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(stakeholder|## who\b)").unwrap()
});
static ARCH_CONCERNS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(concern|rationale|## why\b|design.decision)").unwrap()
});
static ARCH_VIEWPOINTS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(viewpoint|## what\b|## how\b|layer.model|layer.architect|system.diagram)").unwrap()
});

// ISO/IEC/IEEE 29119-3:2021 section regexes
static TEST_STRATEGY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(test.strateg|test.scope|test.design|test.approach)").unwrap()
});
static TEST_CASES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(test.categor|test.case|test.plan|test.pyramid)").unwrap()
});
static TEST_COVERAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(coverage.target|exit.criteria|entry.criteria|test.procedure)").unwrap()
});

// ISO/IEC/IEEE 26514:2022 developer guide section regexes
static GUIDE_BUILD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(build|setup|install|getting.started|quick.start|prerequisite)").unwrap()
});
static GUIDE_STRUCTURE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(project.structure|codebase|architecture|directory|layout|key.files)").unwrap()
});
static GUIDE_EXTENSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(adding|extend|contribut|modif|new.check|new.feature|how.to)").unwrap()
});

// Backlog section regexes (template-engine convention)
static BACKLOG_ITEMS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(backlog.item|high.priority|medium.priority|low.priority|## todo\b)").unwrap()
});
static BACKLOG_COMPLETED_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(## completed|## done\b|## finished\b|## resolved\b|\- \[x\])").unwrap()
});
static BACKLOG_BLOCKERS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(## blocker|## impediment|## blocked.by|## depend|## risk)").unwrap()
});

// ISO/IEC 25010:2023 production readiness section regexes
static PR_SECURITY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(##\s+\d*\.?\s*security|security\s*\|)").unwrap()
});
static PR_TEST_COVERAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(##\s+\d*\.?\s*test.coverage|test.coverage\s*\|)").unwrap()
});
static PR_OBSERVABILITY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(##\s+\d*\.?\s*observability|observability\s*\|)").unwrap()
});
static PR_COMPATIBILITY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(##\s+\d*\.?\s*backwards.compat|compatibility\s*\|)").unwrap()
});
static PR_RUNTIME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(##\s+\d*\.?\s*runtime.safety|runtime.safety\s*\|)").unwrap()
});
static PR_VERDICT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(verdict|ready\s*\|\s*not.ready|PASS.*WARN.*FAIL)").unwrap()
});

/// Check 89: srs_29148_attributes
/// Validates that SRS requirement blocks (FR-xxx, NFR-xxx) have the five
/// mandatory ISO/IEC/IEEE 29148:2018 attribute table entries:
/// Priority, State, Verification, Traces to (or Traceability), Acceptance.
pub struct Srs29148Attributes {
    pub def: RuleDef,
}

impl CheckRunner for Srs29148Attributes {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let srs_path = ctx.root.join("docs/1-requirements/requirements.md");
        if !srs_path.exists() {
            return CheckResult::Skip {
                reason: "docs/1-requirements/requirements.md does not exist".to_string(),
            };
        }

        let content = match fs::read_to_string(&srs_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read requirements.md: {}", e),
                };
            }
        };

        let heading_re = &*SRS_HEADING_RE;
        let next_heading_re = &*SRS_NEXT_HEADING_RE;

        // Collect requirement blocks: (id, block_text)
        let lines: Vec<&str> = content.lines().collect();
        let mut blocks: Vec<(String, String)> = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            if let Some(caps) = heading_re.captures(lines[i]) {
                let req_id = caps[1].to_string();
                let mut block_lines = Vec::new();
                i += 1;
                while i < lines.len() {
                    if next_heading_re.is_match(lines[i]) {
                        break;
                    }
                    block_lines.push(lines[i]);
                    i += 1;
                }
                blocks.push((req_id, block_lines.join("\n")));
            } else {
                i += 1;
            }
        }

        if blocks.is_empty() {
            return CheckResult::Skip {
                reason: "No FR/NFR requirement blocks found in SRS".to_string(),
            };
        }

        // Define the 5 mandatory attributes as regex patterns
        let attrs: &[(&str, &Regex)] = &[
            ("Priority", &ATTR_PRIORITY_RE),
            ("State", &ATTR_STATE_RE),
            ("Verification", &ATTR_VERIFICATION_RE),
            ("Traces to", &ATTR_TRACES_RE),
            ("Acceptance", &ATTR_ACCEPTANCE_RE),
        ];

        let mut violations = Vec::new();

        for (req_id, block) in &blocks {
            let missing: Vec<&str> = attrs.iter()
                .filter(|(_, re)| !re.is_match(block))
                .map(|(name, _)| *name)
                .collect();

            if !missing.is_empty() {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/1-requirements/requirements.md".into()),
                    message: format!(
                        "{} missing {} attribute{}",
                        req_id,
                        missing.join(", "),
                        if missing.len() > 1 { "s" } else { "" }
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

use std::path::PathBuf;
use super::module::discover_modules;

/// Shared helper: check a single file against a set of regex categories.
/// Returns the list of missing category names, or None if the file doesn't
/// exist or is empty (caller decides whether that's a skip or pass).
enum FileCheckResult<'a> {
    Missing(Vec<&'a str>),
    FileAbsent,
    FileEmpty,
    #[allow(dead_code)]
    ReadError(String),
}

fn check_file_sections<'a>(
    path: &std::path::Path,
    categories: &'a [(&'a str, &'a Regex)],
) -> FileCheckResult<'a> {
    if !path.exists() {
        return FileCheckResult::FileAbsent;
    }
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return FileCheckResult::ReadError(e.to_string()),
    };
    if content.trim().is_empty() {
        return FileCheckResult::FileEmpty;
    }
    let missing: Vec<&str> = categories.iter()
        .filter(|(_, re)| !re.is_match(&content))
        .map(|(name, _)| *name)
        .collect();
    FileCheckResult::Missing(missing)
}

fn arch_42010_categories() -> Vec<(&'static str, &'static Regex)> {
    vec![
        ("Stakeholders", &*ARCH_STAKEHOLDERS_RE),
        ("Concerns/rationale", &*ARCH_CONCERNS_RE),
        ("Viewpoints/views", &*ARCH_VIEWPOINTS_RE),
    ]
}

fn test_29119_categories() -> Vec<(&'static str, &'static Regex)> {
    vec![
        ("Strategy/scope", &*TEST_STRATEGY_RE),
        ("Test cases/categories", &*TEST_CASES_RE),
        ("Coverage/criteria", &*TEST_COVERAGE_RE),
    ]
}

/// Check 90: arch_42010_sections
/// Validates that docs/3-design/architecture.md (project-level and module-level)
/// has key ISO/IEC/IEEE 42010:2022 sections: stakeholders, concerns, viewpoints.
pub struct Arch42010Sections {
    pub def: RuleDef,
}

impl CheckRunner for Arch42010Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let categories = arch_42010_categories();
        let mut violations = Vec::new();
        let mut any_file_found = false;

        // Project-level
        let project_path = ctx.root.join("docs/3-design/architecture.md");
        match check_file_sections(&project_path, &categories) {
            FileCheckResult::Missing(missing) => {
                any_file_found = true;
                if !missing.is_empty() {
                    violations.push(Violation {
                        check_id: CheckId(self.def.id),
                        path: Some("docs/3-design/architecture.md".into()),
                        message: format!(
                            "Architecture document missing 42010 section{}: {}",
                            if missing.len() > 1 { "s" } else { "" },
                            missing.join(", ")
                        ),
                        severity: self.def.severity.clone(),
                    });
                }
            }
            FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
        }

        // Module-level
        for m in discover_modules(ctx) {
            let rel: PathBuf = m.path.join("docs/3-design/architecture.md");
            let abs = ctx.root.join(&rel);
            match check_file_sections(&abs, &categories) {
                FileCheckResult::Missing(missing) => {
                    any_file_found = true;
                    if !missing.is_empty() {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(rel),
                            message: format!(
                                "Module '{}' architecture missing 42010 section{}: {}",
                                m.name,
                                if missing.len() > 1 { "s" } else { "" },
                                missing.join(", ")
                            ),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
            }
        }

        if !any_file_found {
            return CheckResult::Skip {
                reason: "No architecture.md files found (project or module level)".to_string(),
            };
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 91: test_29119_sections
/// Validates that docs/5-testing/testing_strategy.md (project-level and module-level)
/// has key ISO/IEC/IEEE 29119-3:2021 sections: strategy, cases, coverage.
pub struct Test29119Sections {
    pub def: RuleDef,
}

impl CheckRunner for Test29119Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let categories = test_29119_categories();
        let mut violations = Vec::new();
        let mut any_file_found = false;

        // Project-level
        let project_path = ctx.root.join("docs/5-testing/testing_strategy.md");
        match check_file_sections(&project_path, &categories) {
            FileCheckResult::Missing(missing) => {
                any_file_found = true;
                if !missing.is_empty() {
                    violations.push(Violation {
                        check_id: CheckId(self.def.id),
                        path: Some("docs/5-testing/testing_strategy.md".into()),
                        message: format!(
                            "Testing strategy missing 29119-3 section{}: {}",
                            if missing.len() > 1 { "s" } else { "" },
                            missing.join(", ")
                        ),
                        severity: self.def.severity.clone(),
                    });
                }
            }
            FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
        }

        // Module-level
        for m in discover_modules(ctx) {
            let rel: PathBuf = m.path.join("docs/5-testing/testing_strategy.md");
            let abs = ctx.root.join(&rel);
            match check_file_sections(&abs, &categories) {
                FileCheckResult::Missing(missing) => {
                    any_file_found = true;
                    if !missing.is_empty() {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(rel),
                            message: format!(
                                "Module '{}' testing strategy missing 29119-3 section{}: {}",
                                m.name,
                                if missing.len() > 1 { "s" } else { "" },
                                missing.join(", ")
                            ),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
            }
        }

        if !any_file_found {
            return CheckResult::Skip {
                reason: "No testing_strategy.md files found (project or module level)".to_string(),
            };
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 92: prod_readiness_exists
/// Validates that docs/6-deployment/production_readiness.md exists.
pub struct ProdReadinessExists {
    pub def: RuleDef,
}

impl CheckRunner for ProdReadinessExists {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let path = ctx.root.join("docs/6-deployment/production_readiness.md");
        if path.exists() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/6-deployment/production_readiness.md".into()),
                    message: "Production readiness document does not exist".to_string(),
                    severity: self.def.severity.clone(),
                }],
            }
        }
    }
}

fn prod_readiness_25010_categories() -> Vec<(&'static str, &'static Regex)> {
    vec![
        ("Security", &*PR_SECURITY_RE),
        ("Test Coverage", &*PR_TEST_COVERAGE_RE),
        ("Observability", &*PR_OBSERVABILITY_RE),
        ("Backwards Compatibility", &*PR_COMPATIBILITY_RE),
        ("Runtime Safety", &*PR_RUNTIME_RE),
        ("Verdict", &*PR_VERDICT_RE),
    ]
}

/// Check 93: prod_readiness_25010_sections
/// Validates that the production readiness document has key ISO/IEC 25010:2023
/// quality sections: security, test coverage, observability, backwards
/// compatibility, runtime safety, and a verdict/scoring section.
pub struct ProdReadiness25010Sections {
    pub def: RuleDef,
}

impl CheckRunner for ProdReadiness25010Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let categories = prod_readiness_25010_categories();

        let project_path = ctx.root.join("docs/6-deployment/production_readiness.md");
        match check_file_sections(&project_path, &categories) {
            FileCheckResult::Missing(missing) => {
                if missing.is_empty() {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail {
                        violations: vec![Violation {
                            check_id: CheckId(self.def.id),
                            path: Some("docs/6-deployment/production_readiness.md".into()),
                            message: format!(
                                "Production readiness missing 25010 section{}: {}",
                                if missing.len() > 1 { "s" } else { "" },
                                missing.join(", ")
                            ),
                            severity: self.def.severity.clone(),
                        }],
                    }
                }
            }
            FileCheckResult::FileAbsent | FileCheckResult::FileEmpty | FileCheckResult::ReadError(_) => {
                CheckResult::Skip {
                    reason: "docs/6-deployment/production_readiness.md not found".to_string(),
                }
            }
        }
    }
}

fn dev_guide_26514_categories() -> Vec<(&'static str, &'static Regex)> {
    vec![
        ("Build/setup", &*GUIDE_BUILD_RE),
        ("Project structure", &*GUIDE_STRUCTURE_RE),
        ("Extension/contribution", &*GUIDE_EXTENSION_RE),
    ]
}

/// Check 94: dev_guide_26514_sections
/// Validates that docs/4-development/developer_guide.md (project-level and
/// module-level) has key ISO/IEC/IEEE 26514:2022 sections: build/setup,
/// project structure, extension/contribution.
pub struct DevGuide26514Sections {
    pub def: RuleDef,
}

impl CheckRunner for DevGuide26514Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let categories = dev_guide_26514_categories();
        let mut violations = Vec::new();
        let mut any_file_found = false;

        // Project-level
        let project_path = ctx.root.join("docs/4-development/developer_guide.md");
        match check_file_sections(&project_path, &categories) {
            FileCheckResult::Missing(missing) => {
                any_file_found = true;
                if !missing.is_empty() {
                    violations.push(Violation {
                        check_id: CheckId(self.def.id),
                        path: Some("docs/4-development/developer_guide.md".into()),
                        message: format!(
                            "Developer guide missing 26514 section{}: {}",
                            if missing.len() > 1 { "s" } else { "" },
                            missing.join(", ")
                        ),
                        severity: self.def.severity.clone(),
                    });
                }
            }
            FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
        }

        // Module-level
        for m in discover_modules(ctx) {
            let rel: PathBuf = m.path.join("docs/4-development/developer_guide.md");
            let abs = ctx.root.join(&rel);
            match check_file_sections(&abs, &categories) {
                FileCheckResult::Missing(missing) => {
                    any_file_found = true;
                    if !missing.is_empty() {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(rel),
                            message: format!(
                                "Module '{}' developer guide missing 26514 section{}: {}",
                                m.name,
                                if missing.len() > 1 { "s" } else { "" },
                                missing.join(", ")
                            ),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                FileCheckResult::FileEmpty | FileCheckResult::FileAbsent | FileCheckResult::ReadError(_) => {}
            }
        }

        if !any_file_found {
            return CheckResult::Skip {
                reason: "No developer_guide.md files found (project or module level)".to_string(),
            };
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

fn backlog_sections_categories() -> Vec<(&'static str, &'static Regex)> {
    vec![
        ("Backlog items/priorities", &*BACKLOG_ITEMS_RE),
        ("Completed", &*BACKLOG_COMPLETED_RE),
        ("Blockers", &*BACKLOG_BLOCKERS_RE),
    ]
}

/// Check 95: backlog_sections
/// Validates that docs/2-planning/backlog.md has key sections per the
/// template-engine backlog convention: backlog items with priorities,
/// completed items, and blockers.
pub struct BacklogSections {
    pub def: RuleDef,
}

impl CheckRunner for BacklogSections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let categories = backlog_sections_categories();

        let project_path = ctx.root.join("docs/2-planning/backlog.md");
        match check_file_sections(&project_path, &categories) {
            FileCheckResult::Missing(missing) => {
                if missing.is_empty() {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail {
                        violations: vec![Violation {
                            check_id: CheckId(self.def.id),
                            path: Some("docs/2-planning/backlog.md".into()),
                            message: format!(
                                "Backlog missing section{}: {}",
                                if missing.len() > 1 { "s" } else { "" },
                                missing.join(", ")
                            ),
                            severity: self.def.severity.clone(),
                        }],
                    }
                }
            }
            FileCheckResult::FileAbsent | FileCheckResult::FileEmpty | FileCheckResult::ReadError(_) => {
                CheckResult::Skip {
                    reason: "docs/2-planning/backlog.md not found".to_string(),
                }
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
    use std::path::Path;
    use tempfile::TempDir;

    fn make_def() -> RuleDef {
        RuleDef {
            id: 89,
            category: "requirements".to_string(),
            description: "SRS requirements have ISO/IEC/IEEE 29148:2018 attribute tables".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: "srs_29148_attributes".to_string() },
            project_type: None,
        }
    }

    fn make_ctx(root: &Path) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            files: vec![],
            file_contents: HashMap::new(),
            project_type: ProjectType::OpenSource,
        }
    }

    fn write_file(root: &Path, relative: &str, content: &str) {
        let full = root.join(relative);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&full, content).unwrap();
    }

    fn srs_with_block(block: &str) -> String {
        format!("# Requirements\n\n**Audience**: Developers\n\n{}", block)
    }

    fn complete_fr_block(id: &str, title: &str) -> String {
        format!(
            "#### {}: {}\n\n\
             | Attribute | Value |\n\
             |-----------|-------|\n\
             | **Priority** | Must |\n\
             | **State** | Approved |\n\
             | **Verification** | Test |\n\
             | **Traces to** | STK-01 |\n\
             | **Acceptance** | System meets criteria |\n\n\
             The system shall do the thing.\n",
            id, title
        )
    }

    // =========================================================================
    // Pass cases
    // =========================================================================

    #[test]
    fn test_pass_fr_all_attributes() {
        let tmp = TempDir::new().unwrap();
        let block = complete_fr_block("FR-100", "Sample requirement");
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(&block));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_pass_nfr_all_attributes() {
        let tmp = TempDir::new().unwrap();
        let block = complete_fr_block("NFR-200", "Performance requirement");
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(&block));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_pass_traceability_alternative() {
        let tmp = TempDir::new().unwrap();
        let block = "#### FR-100: Sample requirement\n\n\
             | Attribute | Value |\n\
             |-----------|-------|\n\
             | **Priority** | Must |\n\
             | **State** | Approved |\n\
             | **Verification** | Test |\n\
             | **Traceability** | STK-01 |\n\
             | **Acceptance** | System meets criteria |\n";
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(block));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_pass_multiple_complete_blocks() {
        let tmp = TempDir::new().unwrap();
        let block1 = complete_fr_block("FR-100", "First requirement");
        let block2 = complete_fr_block("FR-101", "Second requirement");
        let block3 = complete_fr_block("NFR-200", "Non-functional requirement");
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(&format!("{}\n{}\n{}", block1, block2, block3)));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // =========================================================================
    // Fail cases
    // =========================================================================

    #[test]
    fn test_fail_missing_priority() {
        let tmp = TempDir::new().unwrap();
        let block = "#### FR-100: Sample requirement\n\n\
             | Attribute | Value |\n\
             |-----------|-------|\n\
             | **State** | Approved |\n\
             | **Verification** | Test |\n\
             | **Traces to** | STK-01 |\n\
             | **Acceptance** | System meets criteria |\n";
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(block));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("FR-100"));
                assert!(violations[0].message.contains("Priority"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_fail_missing_multiple_attributes() {
        let tmp = TempDir::new().unwrap();
        let block = "#### FR-100: Sample requirement\n\n\
             | Attribute | Value |\n\
             |-----------|-------|\n\
             | **State** | Approved |\n\
             | **Acceptance** | System meets criteria |\n";
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(block));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("FR-100"));
                assert!(violations[0].message.contains("Priority"));
                assert!(violations[0].message.contains("Verification"));
                assert!(violations[0].message.contains("Traces to"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_fail_mixed_blocks() {
        let tmp = TempDir::new().unwrap();
        let complete = complete_fr_block("FR-100", "Complete requirement");
        let incomplete = "#### FR-101: Incomplete requirement\n\n\
             | Attribute | Value |\n\
             |-----------|-------|\n\
             | **Priority** | Must |\n";
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            &srs_with_block(&format!("{}\n{}", complete, incomplete)));

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("FR-101"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    // =========================================================================
    // Skip cases
    // =========================================================================

    #[test]
    fn test_skip_no_srs_file() {
        let tmp = TempDir::new().unwrap();

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_skip_no_fr_nfr_blocks() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            "# Requirements\n\n**Audience**: Developers\n\nSome general text.\n");

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_skip_only_stk_blocks() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/1-requirements/requirements.md",
            "# Requirements\n\n**Audience**: Developers\n\n\
             #### STK-01: Stakeholder requirement\n\n\
             | ID | Requirement | Source |\n\
             |----|-------------|--------|\n\
             | STK-01 | The tool shall audit | Compliance |\n");

        let handler = Srs29148Attributes { def: make_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // =========================================================================
    // Check 90: Arch42010Sections
    // =========================================================================

    fn make_arch_def() -> RuleDef {
        RuleDef {
            id: 90,
            category: "requirements".to_string(),
            description: "Architecture document has ISO/IEC/IEEE 42010:2022 sections".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "arch_42010_sections".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_arch_42010_pass_explicit_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n**Audience**: Developers\n\n\
             ## Stakeholders\nDevelopers, Architects\n\n\
             ## Concerns\nModularity, performance\n\n\
             ## Viewpoints\nStructural, behavioral\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_arch_42010_pass_w3h_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n**Audience**: Developers\n\n\
             ## Who\nStakeholders\n\n\
             ## Why\nDesign rationale\n\n\
             ## What\nSystem overview\n\n\
             ## How\nImplementation\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_arch_42010_fail_missing_stakeholders() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n**Audience**: Developers\n\n\
             ## Concerns\nModularity\n\n\
             ## Viewpoints\nStructural\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Stakeholders"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_arch_42010_fail_missing_all() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n**Audience**: Developers\n\nSome generic content.\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Stakeholders"));
                assert!(violations[0].message.contains("Concerns"));
                assert!(violations[0].message.contains("Viewpoints"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_arch_42010_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_arch_42010_skip_empty_file() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md", "  \n  \n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // =========================================================================
    // Check 91: Test29119Sections
    // =========================================================================

    fn make_test_def() -> RuleDef {
        RuleDef {
            id: 91,
            category: "requirements".to_string(),
            description: "Testing strategy has ISO/IEC/IEEE 29119-3:2021 sections".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "test_29119_sections".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_29119_pass_explicit_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing Strategy\n\n**Audience**: Developers\n\n\
             ## Test Strategy\nRisk-based testing.\n\n\
             ## Test Categories\nUnit, integration, E2E.\n\n\
             ## Coverage Targets\n80% line coverage.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_29119_pass_pyramid_and_coverage() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing Strategy\n\n**Audience**: Developers\n\n\
             ## Test Design\nRequirements-based.\n\n\
             ## Test Pyramid\nUnit > Integration > E2E.\n\n\
             ## Coverage Targets\n80%.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_29119_fail_missing_coverage() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing Strategy\n\n**Audience**: Developers\n\n\
             ## Test Strategy\nRisk-based.\n\n\
             ## Test Categories\nUnit tests.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Coverage"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_29119_fail_missing_all() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing Strategy\n\n**Audience**: Developers\n\nGeneric content only.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Strategy"));
                assert!(violations[0].message.contains("Test cases"));
                assert!(violations[0].message.contains("Coverage"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_29119_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_29119_skip_empty_file() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md", "  \n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // =========================================================================
    // Check 90: Module-level architecture
    // =========================================================================

    #[test]
    fn test_arch_42010_pass_module_level() {
        let tmp = TempDir::new().unwrap();
        // Module with architecture doc that has 42010 sections
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/3-design/architecture.md",
            "# Auth Architecture\n\n\
             ## Stakeholders\nAuth team\n\n\
             ## Concerns\nSecurity, latency\n\n\
             ## Viewpoints\nStructural\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_arch_42010_fail_module_missing_sections() {
        let tmp = TempDir::new().unwrap();
        // Project-level passes, module-level fails
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n## Stakeholders\nTeam\n\n## Concerns\nPerf\n\n## Viewpoints\nStructural\n");
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/3-design/architecture.md",
            "# Auth Architecture\n\nJust some text.\n");

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("auth"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_arch_42010_pass_no_module_arch() {
        let tmp = TempDir::new().unwrap();
        // Project-level passes, module has no architecture.md (skip module)
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\n## Stakeholders\nTeam\n\n## Concerns\nPerf\n\n## Viewpoints\nStructural\n");
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();

        let handler = Arch42010Sections { def: make_arch_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // =========================================================================
    // Check 91: Module-level testing strategy
    // =========================================================================

    #[test]
    fn test_29119_pass_module_level() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/5-testing/testing_strategy.md",
            "# Auth Testing\n\n\
             ## Test Strategy\nUnit-first.\n\n\
             ## Test Cases\nLogin, logout.\n\n\
             ## Coverage Targets\n90%.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_29119_fail_module_missing_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing\n\n## Test Strategy\nOk.\n\n## Test Pyramid\nOk.\n\n## Coverage Targets\nOk.\n");
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/5-testing/testing_strategy.md",
            "# Auth Testing\n\nNo sections here.\n");

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("auth"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    // =========================================================================
    // Check 92: ProdReadinessExists
    // =========================================================================

    fn make_prod_exists_def() -> RuleDef {
        RuleDef {
            id: 92,
            category: "requirements".to_string(),
            description: "Production readiness document exists".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "prod_readiness_exists".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_prod_readiness_exists_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md",
            "# Production Readiness\n\n**Audience**: Developers\n");

        let handler = ProdReadinessExists { def: make_prod_exists_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_prod_readiness_exists_fail() {
        let tmp = TempDir::new().unwrap();

        let handler = ProdReadinessExists { def: make_prod_exists_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("does not exist"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    // =========================================================================
    // Check 93: ProdReadiness25010Sections
    // =========================================================================

    fn make_prod_25010_def() -> RuleDef {
        RuleDef {
            id: 93,
            category: "requirements".to_string(),
            description: "Production readiness has ISO/IEC 25010:2023 quality sections".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "prod_readiness_25010_sections".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_prod_25010_pass_all_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md",
            "# Production Readiness\n\n**Audience**: Developers\n\n\
             ## Verdict: READY\n\n\
             ## 6. Runtime Safety\n\nNo panics.\n\n\
             ## 11. Security\n\nNo secrets.\n\n\
             ## 12. Test Coverage\n\n252 tests.\n\n\
             ## 13. Observability\n\nStructured logs.\n\n\
             ## 14. Backwards Compatibility\n\nSemver.\n");

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_prod_25010_pass_table_format() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md",
            "# Production Readiness\n\n\
             | Area | Status |\n|------|--------|\n\
             | Security | PASS |\n\
             | Test Coverage | PASS |\n\
             | Observability | PASS |\n\
             | Compatibility | PASS |\n\
             | Runtime Safety | PASS |\n\n\
             PASS | WARN | FAIL scoring.\n");

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_prod_25010_fail_missing_security() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md",
            "# Production Readiness\n\n\
             ## Verdict: READY\n\n\
             ## 6. Runtime Safety\n\nOk.\n\n\
             ## 12. Test Coverage\n\nOk.\n\n\
             ## 13. Observability\n\nOk.\n\n\
             ## 14. Backwards Compatibility\n\nOk.\n");

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Security"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_prod_25010_fail_missing_all() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md",
            "# Production Readiness\n\n**Audience**: Developers\n\nGeneric content only.\n");

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Security"));
                assert!(violations[0].message.contains("Test Coverage"));
                assert!(violations[0].message.contains("Observability"));
                assert!(violations[0].message.contains("Runtime Safety"));
                assert!(violations[0].message.contains("Verdict"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_prod_25010_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_prod_25010_skip_empty_file() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/6-deployment/production_readiness.md", "  \n");

        let handler = ProdReadiness25010Sections { def: make_prod_25010_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_29119_pass_no_module_testing() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/5-testing/testing_strategy.md",
            "# Testing\n\n## Test Strategy\nOk.\n\n## Test Pyramid\nOk.\n\n## Coverage Targets\nOk.\n");
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();

        let handler = Test29119Sections { def: make_test_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // =========================================================================
    // Check 94: DevGuide26514Sections
    // =========================================================================

    fn make_guide_def() -> RuleDef {
        RuleDef {
            id: 94,
            category: "requirements".to_string(),
            description: "Developer guide has ISO/IEC/IEEE 26514:2022 sections".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "dev_guide_26514_sections".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_dev_guide_26514_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/4-development/developer_guide.md",
            "# Developer Guide\n\n**Audience**: Developers\n\n\
             ## Build & Test\nRun `cargo build`.\n\n\
             ## Project Structure\nSee src/ for layout.\n\n\
             ## Adding New Features\nExtend the codebase.\n");

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_dev_guide_26514_fail_missing_structure() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/4-development/developer_guide.md",
            "# Developer Guide\n\n**Audience**: Developers\n\n\
             ## Build & Test\nRun `cargo build`.\n\n\
             ## Adding New Features\nCreate new modules.\n");

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Project structure"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_dev_guide_26514_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_dev_guide_26514_skip_empty() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/4-development/developer_guide.md", "  \n");

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_dev_guide_26514_pass_module_level() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/4-development/developer_guide.md",
            "# Auth Developer Guide\n\n\
             ## Getting Started\nInstall deps.\n\n\
             ## Codebase\nSrc layout.\n\n\
             ## Contributing\nOpen a PR.\n");

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_dev_guide_26514_fail_module_missing_sections() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/4-development/developer_guide.md",
            "# Developer Guide\n\n## Build\nOk.\n\n## Project Structure\nOk.\n\n## Adding\nOk.\n");
        std::fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        std::fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        write_file(tmp.path(), "crates/auth/docs/4-development/developer_guide.md",
            "# Auth Guide\n\nNo sections here.\n");

        let handler = DevGuide26514Sections { def: make_guide_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("auth"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    // =========================================================================
    // Check 95: BacklogSections
    // =========================================================================

    fn make_backlog_def() -> RuleDef {
        RuleDef {
            id: 95,
            category: "requirements".to_string(),
            description: "Backlog has required sections".to_string(),
            severity: Severity::Info,
            rule_type: RuleType::Builtin { handler: "backlog_sections".to_string() },
            project_type: None,
        }
    }

    #[test]
    fn test_backlog_sections_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n## Backlog Items\n\n\
             ### High Priority\n\n- [ ] Task one\n\n\
             ### Medium Priority\n\n- [ ] Task two\n\n\
             ## Completed\n\n- [x] Done task — 2026-01-01\n\n\
             ## Blockers\n\n| Blocker | Impact |\n|---------|--------|\n| None | — |\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_sections_pass_checkbox_completed() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n## Backlog Items\n\n\
             ### High Priority\n\n- [ ] Task one\n\n\
             - [x] Finished task\n\n\
             ## Blockers\n\nNo blockers.\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_sections_fail_missing_blockers() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n## Backlog Items\n\n\
             ### High Priority\n\n- [ ] Task one\n\n\
             ## Completed\n\n- [x] Done task\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Blockers"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_backlog_sections_fail_missing_all() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n**Audience**: Developers\n\nGeneric content only.\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Backlog items"));
                assert!(violations[0].message.contains("Completed"));
                assert!(violations[0].message.contains("Blockers"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_backlog_sections_fail_missing_completed() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n## Backlog Items\n\n\
             ### High Priority\n\n- [ ] Task one\n\n\
             ## Blockers\n\nNo blockers.\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Completed"));
                assert!(!violations[0].message.contains("Backlog items"));
                assert!(!violations[0].message.contains("Blockers"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_backlog_sections_fail_missing_items() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\n## Completed\n\n- [x] Setup done\n\n\
             ## Blockers\n\n| Blocker | Status |\n|---------|--------|\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        match handler.run(&ctx) {
            CheckResult::Fail { violations } => {
                assert_eq!(violations.len(), 1);
                assert!(violations[0].message.contains("Backlog items"));
                assert!(!violations[0].message.contains("Completed"));
            }
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_backlog_sections_pass_alternative_keywords() {
        let tmp = TempDir::new().unwrap();
        // Uses "## Todo", "## Done", "## Risk" instead of template-standard names
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Sprint Backlog\n\n## Todo\n\n\
             ### Low Priority\n\n- [ ] Refactor module\n\n\
             ## Done\n\n- [x] Initial release\n\n\
             ## Risk\n\nNo open risks.\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_sections_pass_with_status_and_overview() {
        let tmp = TempDir::new().unwrap();
        // Full template structure including optional sections
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Project Backlog\n\n\
             ## Status: In Progress\n\n\
             ## Overview\n\nTracking all work items.\n\n\
             ## Current Sprint\n\n\
             | Task | Priority | Status |\n|------|----------|--------|\n\
             | Auth | P0 | In Progress |\n\n\
             ## Backlog Items\n\n\
             ### High Priority\n\n- [ ] Auth module\n\n\
             ### Medium Priority\n\n- [ ] Logging\n\n\
             ## Completed\n\n- [x] Project setup — 2026-01-01\n\n\
             ## Blockers\n\n\
             | Blocker | Impact | Owner | Status |\n\
             |---------|--------|-------|--------|\n\
             | CI setup | High | Team | Open |\n\n\
             ## Notes\n\n- Review weekly.\n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_sections_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_backlog_sections_skip_empty() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md", "  \n");

        let handler = BacklogSections { def: make_backlog_def() };
        let ctx = make_ctx(tmp.path());
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }
}
