use std::fs;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

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

        let heading_re = Regex::new(r"^####\s+((?:FR|NFR)-\d+):\s+.+$").unwrap();
        let next_heading_re = Regex::new(r"^#{1,4}\s+").unwrap();

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
        let attrs: &[(&str, Regex)] = &[
            ("Priority", Regex::new(r"\*\*Priority\*\*").unwrap()),
            ("State", Regex::new(r"\*\*State\*\*").unwrap()),
            ("Verification", Regex::new(r"\*\*Verification\*\*").unwrap()),
            ("Traces to", Regex::new(r"\*\*Traces\s+to\*\*|\*\*Traceability\*\*").unwrap()),
            ("Acceptance", Regex::new(r"\*\*Acceptance\*\*").unwrap()),
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

/// Check 90: arch_42010_sections
/// Validates that docs/3-design/architecture.md has key ISO/IEC/IEEE 42010:2022
/// sections: stakeholder identification, architectural concerns, and viewpoints.
pub struct Arch42010Sections {
    pub def: RuleDef,
}

impl CheckRunner for Arch42010Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let arch_path = ctx.root.join("docs/3-design/architecture.md");
        if !arch_path.exists() {
            return CheckResult::Skip {
                reason: "docs/3-design/architecture.md does not exist".to_string(),
            };
        }

        let content = match fs::read_to_string(&arch_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read architecture.md: {}", e),
                };
            }
        };

        if content.trim().is_empty() {
            return CheckResult::Skip {
                reason: "architecture.md is empty".to_string(),
            };
        }

        let categories: &[(&str, Regex)] = &[
            ("Stakeholders", Regex::new(r"(?i)(stakeholder|## who\b)").unwrap()),
            ("Concerns/rationale", Regex::new(r"(?i)(concern|rationale|## why\b|design.decision)").unwrap()),
            ("Viewpoints/views", Regex::new(r"(?i)(viewpoint|## what\b|## how\b|layer.model|layer.architect|system.diagram)").unwrap()),
        ];

        let missing: Vec<&str> = categories.iter()
            .filter(|(_, re)| !re.is_match(&content))
            .map(|(name, _)| *name)
            .collect();

        if missing.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/3-design/architecture.md".into()),
                    message: format!(
                        "Architecture document missing 42010 section{}: {}",
                        if missing.len() > 1 { "s" } else { "" },
                        missing.join(", ")
                    ),
                    severity: self.def.severity.clone(),
                }],
            }
        }
    }
}

/// Check 91: test_29119_sections
/// Validates that docs/5-testing/testing_strategy.md has key ISO/IEC/IEEE 29119-3:2021
/// sections: test strategy/scope, test categories/cases, and coverage/criteria.
pub struct Test29119Sections {
    pub def: RuleDef,
}

impl CheckRunner for Test29119Sections {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let test_path = ctx.root.join("docs/5-testing/testing_strategy.md");
        if !test_path.exists() {
            return CheckResult::Skip {
                reason: "docs/5-testing/testing_strategy.md does not exist".to_string(),
            };
        }

        let content = match fs::read_to_string(&test_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read testing_strategy.md: {}", e),
                };
            }
        };

        if content.trim().is_empty() {
            return CheckResult::Skip {
                reason: "testing_strategy.md is empty".to_string(),
            };
        }

        let categories: &[(&str, Regex)] = &[
            ("Strategy/scope", Regex::new(r"(?i)(test.strateg|test.scope|test.design|test.approach)").unwrap()),
            ("Test cases/categories", Regex::new(r"(?i)(test.categor|test.case|test.plan|test.pyramid)").unwrap()),
            ("Coverage/criteria", Regex::new(r"(?i)(coverage.target|exit.criteria|entry.criteria|test.procedure)").unwrap()),
        ];

        let missing: Vec<&str> = categories.iter()
            .filter(|(_, re)| !re.is_match(&content))
            .map(|(name, _)| *name)
            .collect();

        if missing.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/5-testing/testing_strategy.md".into()),
                    message: format!(
                        "Testing strategy missing 29119-3 section{}: {}",
                        if missing.len() > 1 { "s" } else { "" },
                        missing.join(", ")
                    ),
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
}
