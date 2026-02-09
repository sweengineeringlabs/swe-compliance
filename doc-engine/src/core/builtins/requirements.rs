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
}
