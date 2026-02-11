use std::path::Path;

use crate::api::traits::CheckRunner;
use crate::api::types::{RuleDef, CheckId, CheckResult, ProjectKind, ScanContext, Violation};

fn make_violation(def: &RuleDef, path: Option<&Path>, message: &str) -> Violation {
    Violation {
        check_id: CheckId(def.id),
        path: path.map(|p| p.to_path_buf()),
        message: message.to_string(),
        severity: def.severity.clone(),
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
                )],
            }
        }
    }
}
