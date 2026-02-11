use std::path::Path;

use regex::Regex;

use crate::api::traits::CheckRunner;
use crate::api::types::{RuleDef, CheckId, CheckResult, ScanContext, Violation};

fn make_violation(def: &RuleDef, path: Option<&Path>, message: &str) -> Violation {
    Violation {
        check_id: CheckId(def.id),
        path: path.map(|p| p.to_path_buf()),
        message: message.to_string(),
        severity: def.severity.clone(),
    }
}

/// Check 31: Module file names match mod declarations.
pub struct ModuleNamesMatch {
    pub def: RuleDef,
}

impl CheckRunner for ModuleNamesMatch {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let mod_decl_re = Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+(\w+)\s*;").unwrap();

        let source_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("src/") || s.starts_with("main/src/"))
                    && s.ends_with(".rs")
            })
            .collect();

        let mut violations = Vec::new();

        for file in &source_files {
            let full = ctx.root.join(file);
            let content = match std::fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let parent = file.parent().unwrap_or(Path::new(""));

            for cap in mod_decl_re.captures_iter(&content) {
                let mod_name = &cap[1];

                // Module should be either <mod_name>.rs or <mod_name>/mod.rs
                let file_path = parent.join(format!("{}.rs", mod_name));
                let dir_path = parent.join(mod_name).join("mod.rs");

                let file_exists = ctx.files.iter().any(|f| {
                    let s = f.to_string_lossy().replace('\\', "/");
                    let fp = file_path.to_string_lossy().replace('\\', "/");
                    let dp = dir_path.to_string_lossy().replace('\\', "/");
                    s == fp || s == dp
                });

                if !file_exists {
                    violations.push(make_violation(
                        &self.def,
                        Some(file),
                        &format!(
                            "Module '{}' declared in '{}' but neither '{}.rs' nor '{}/mod.rs' found",
                            mod_name,
                            file.display(),
                            mod_name,
                            mod_name
                        ),
                    ));
                }
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

/// Check 32: Binary names use hyphens or underscores.
pub struct BinNamesValid {
    pub def: RuleDef,
}

impl CheckRunner for BinNamesValid {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        if manifest.bins.is_empty() {
            return CheckResult::Pass;
        }

        let valid_re = Regex::new(r"^[a-z][a-z0-9_-]*$").unwrap();

        let mut violations = Vec::new();
        for bin in &manifest.bins {
            if !valid_re.is_match(&bin.name) {
                violations.push(make_violation(
                    &self.def,
                    Some(Path::new("Cargo.toml")),
                    &format!(
                        "Binary name '{}' should use only lowercase letters, digits, hyphens, or underscores",
                        bin.name
                    ),
                ));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}
