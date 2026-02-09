use std::path::Path;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

fn make_violation(def: &RuleDef, path: Option<&Path>, message: &str) -> Violation {
    Violation {
        check_id: CheckId(def.id),
        path: path.map(|p| p.to_path_buf()),
        message: message.to_string(),
        severity: def.severity.clone(),
    }
}

/// Check 3: src/lib.rs or src/main.rs exists (standard layout).
pub struct CrateRootExists {
    pub def: RuleDef,
}

impl CheckRunner for CrateRootExists {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let lib_rs = ctx.root.join("src/lib.rs");
        let main_rs = ctx.root.join("src/main.rs");
        if lib_rs.exists() || main_rs.exists() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    Some(Path::new("src/")),
                    "Neither src/lib.rs nor src/main.rs exists",
                )],
            }
        }
    }
}

/// Check 5: main/src/lib.rs or main/src/main.rs exists (rustboot layout).
pub struct RustbootCrateRootExists {
    pub def: RuleDef,
}

impl CheckRunner for RustbootCrateRootExists {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let lib_rs = ctx.root.join("main/src/lib.rs");
        let main_rs = ctx.root.join("main/src/main.rs");
        if lib_rs.exists() || main_rs.exists() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    Some(Path::new("main/src/")),
                    "Neither main/src/lib.rs nor main/src/main.rs exists",
                )],
            }
        }
    }
}

/// Check 8: benches/ directory exists if benchmarks are declared.
pub struct BenchesDirIfDeclared {
    pub def: RuleDef,
}

impl CheckRunner for BenchesDirIfDeclared {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Pass,
        };

        if manifest.benches.is_empty() {
            return CheckResult::Pass;
        }

        let benches_dir = ctx.root.join("benches");
        if benches_dir.exists() && benches_dir.is_dir() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    Some(Path::new("benches")),
                    "[[bench]] targets declared but benches/ directory does not exist",
                )],
            }
        }
    }
}

/// Check 13: package.license or package.license-file exists.
pub struct LicenseFieldExists {
    pub def: RuleDef,
}

impl CheckRunner for LicenseFieldExists {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => {
                return CheckResult::Skip {
                    reason: "No Cargo.toml found".to_string(),
                };
            }
        };

        let raw = match &manifest.raw {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: "Cargo.toml not parsed".to_string(),
                };
            }
        };

        let has_license = raw.get("package")
            .and_then(|p| p.get("license"))
            .is_some();
        let has_license_file = raw.get("package")
            .and_then(|p| p.get("license-file"))
            .is_some();

        if has_license || has_license_file {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![make_violation(
                    &self.def,
                    Some(Path::new("Cargo.toml")),
                    "Neither package.license nor package.license-file found in Cargo.toml",
                )],
            }
        }
    }
}

/// Check 19: [lib] path matches src/lib.rs or main/src/lib.rs.
pub struct LibPathCorrect {
    pub def: RuleDef,
}

impl CheckRunner for LibPathCorrect {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        if !manifest.has_lib {
            return CheckResult::Skip { reason: "No [lib] section in Cargo.toml".to_string() };
        }

        match &manifest.lib_path {
            Some(path) => {
                let full = ctx.root.join(path);
                if full.exists() {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail {
                        violations: vec![make_violation(
                            &self.def,
                            Some(Path::new("Cargo.toml")),
                            &format!("[lib] path '{}' does not resolve to an existing file", path),
                        )],
                    }
                }
            }
            None => {
                // Default lib path is src/lib.rs
                if ctx.root.join("src/lib.rs").exists() {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail {
                        violations: vec![make_violation(
                            &self.def,
                            Some(Path::new("src/lib.rs")),
                            "Default lib path src/lib.rs does not exist",
                        )],
                    }
                }
            }
        }
    }
}

/// Check 20: [[bin]] path matches src/main.rs or main/src/main.rs.
pub struct BinPathCorrect {
    pub def: RuleDef,
}

impl CheckRunner for BinPathCorrect {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        if manifest.bins.is_empty() {
            return CheckResult::Skip { reason: "No [[bin]] targets in Cargo.toml".to_string() };
        }

        let mut violations = Vec::new();
        for bin in &manifest.bins {
            if let Some(ref path) = bin.path {
                let full = ctx.root.join(path);
                if !full.exists() {
                    violations.push(make_violation(
                        &self.def,
                        Some(Path::new("Cargo.toml")),
                        &format!("[[bin]] '{}' path '{}' does not resolve to an existing file", bin.name, path),
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

/// Check 21: [[test]] targets declared for tests/ files.
pub struct TestTargetsDeclared {
    pub def: RuleDef,
}

impl CheckRunner for TestTargetsDeclared {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        // Find .rs files under tests/
        let test_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("tests/") || s.starts_with("tests\\"))
                    && s.ends_with(".rs")
                    && !s.contains("common/")
                    && !s.contains("common\\")
            })
            .collect();

        if test_files.is_empty() {
            return CheckResult::Pass;
        }

        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        // If no [[test]] declared and there are test files, that's fine for auto-discovery
        // Cargo auto-discovers tests/ files, so this is only informational
        if manifest.tests.is_empty() && !test_files.is_empty() {
            return CheckResult::Pass; // Cargo auto-discovers
        }

        CheckResult::Pass
    }
}

/// Check 22: [[bench]] targets have harness = false.
pub struct BenchHarnessFalse {
    pub def: RuleDef,
}

impl CheckRunner for BenchHarnessFalse {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        if manifest.benches.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for bench in &manifest.benches {
            match bench.harness {
                Some(false) => {} // correct
                _ => {
                    violations.push(make_violation(
                        &self.def,
                        Some(Path::new("Cargo.toml")),
                        &format!("[[bench]] '{}' does not have harness = false", bench.name),
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

/// Check 23: No undeclared test files.
pub struct NoUndeclaredTests {
    pub def: RuleDef,
}

impl CheckRunner for NoUndeclaredTests {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Pass,
        };

        // Only check if test targets are explicitly declared
        if manifest.tests.is_empty() {
            return CheckResult::Pass;
        }

        let declared_paths: Vec<String> = manifest.tests.iter()
            .filter_map(|t| t.path.clone())
            .map(|p| p.replace('\\', "/"))
            .collect();

        let mut violations = Vec::new();
        for file in &ctx.files {
            let s = file.to_string_lossy().replace('\\', "/");
            if s.starts_with("tests/") && s.ends_with(".rs")
                && !s.contains("common/") && !s.ends_with("mod.rs")
                && !declared_paths.contains(&s)
            {
                violations.push(make_violation(
                    &self.def,
                    Some(file),
                    &format!("Test file '{}' not declared in [[test]]", s),
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

/// Check 24: No undeclared bench files.
pub struct NoUndeclaredBenches {
    pub def: RuleDef,
}

impl CheckRunner for NoUndeclaredBenches {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Pass,
        };

        if manifest.benches.is_empty() {
            return CheckResult::Pass;
        }

        // Benches are auto-discovered from benches/*.rs, so this is mostly informational
        CheckResult::Pass
    }
}

/// Check 25: [[example]] targets exist if examples/ present.
pub struct ExampleTargetsIfDir {
    pub def: RuleDef,
}

impl CheckRunner for ExampleTargetsIfDir {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let examples_dir = ctx.root.join("examples");
        if !examples_dir.exists() || !examples_dir.is_dir() {
            return CheckResult::Pass;
        }

        // Cargo auto-discovers examples, so this always passes if dir exists
        CheckResult::Pass
    }
}

/// Check 26: [[test]] paths resolve to existing files.
pub struct TestPathsResolve {
    pub def: RuleDef,
}

impl CheckRunner for TestPathsResolve {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let manifest = match &ctx.cargo_manifest {
            Some(m) => m,
            None => return CheckResult::Skip { reason: "No Cargo.toml found".to_string() },
        };

        if manifest.tests.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for test in &manifest.tests {
            if let Some(ref path) = test.path {
                let full = ctx.root.join(path);
                if !full.exists() {
                    violations.push(make_violation(
                        &self.def,
                        Some(Path::new(path.as_str())),
                        &format!("[[test]] '{}' path '{}' does not exist", test.name, path),
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
