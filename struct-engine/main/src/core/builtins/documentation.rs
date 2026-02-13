use std::path::Path;

use crate::api::traits::CheckRunner;
use crate::api::types::{RuleDef, CheckId, CheckResult, ProjectKind, ScanContext, Violation};

fn make_violation(
    def: &RuleDef,
    path: Option<&Path>,
    message: &str,
    expected: Option<&str>,
    actual: Option<&str>,
    fix_hint: Option<&str>,
) -> Violation {
    Violation {
        check_id: CheckId(def.id),
        path: path.map(|p| p.to_path_buf()),
        message: message.to_string(),
        severity: def.severity.clone(),
        rule_type: def.rule_type.to_tag(),
        expected: expected.map(String::from),
        actual: actual.map(String::from),
        fix_hint: fix_hint.map(String::from)
            .unwrap_or_else(|| def.fix_hint.clone()
                .unwrap_or_else(|| def.rule_type.auto_fix_hint())),
    }
}

/// Check 40: doc/ or docs/ directory exists (if library).
pub struct DocDirExists {
    pub def: RuleDef,
}

impl CheckRunner for DocDirExists {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        // Only check for libraries
        if ctx.project_kind != ProjectKind::Library && ctx.project_kind != ProjectKind::Both {
            return CheckResult::Skip {
                reason: "Only applicable to library projects".to_string(),
            };
        }

        let doc = ctx.root.join("doc");
        let docs = ctx.root.join("docs");
        if (doc.exists() && doc.is_dir()) || (docs.exists() && docs.is_dir()) {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    None,
                    "Neither doc/ nor docs/ directory exists (recommended for libraries)",
                    Some("doc/ or docs/"),
                    Some("missing"),
                    Some("Create a docs/ directory for project documentation"),
                )],
            }
        }
    }
}

/// Check 41: examples/ directory exists (if library).
pub struct ExamplesDirLib {
    pub def: RuleDef,
}

impl CheckRunner for ExamplesDirLib {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        // Only check for libraries
        if ctx.project_kind != ProjectKind::Library && ctx.project_kind != ProjectKind::Both {
            return CheckResult::Skip {
                reason: "Only applicable to library projects".to_string(),
            };
        }

        let examples = ctx.root.join("examples");
        if examples.exists() && examples.is_dir() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    None,
                    "examples/ directory does not exist (recommended for libraries)",
                    Some("examples/"),
                    Some("missing"),
                    Some("Create an examples/ directory with usage examples"),
                )],
            }
        }
    }
}
