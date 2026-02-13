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

/// Check 33: Test files use correct suffixes (_test.rs, _int_test.rs, etc.).
pub struct TestFileSuffixes {
    pub def: RuleDef,
}

impl CheckRunner for TestFileSuffixes {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let valid_suffixes = [
            "_int_test.rs",
            "_stress_test.rs",
            "_perf_test.rs",
            "_load_test.rs",
            "_e2e_test.rs",
            "_security_test.rs",
        ];

        let test_files: Vec<_> = ctx.files().iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("tests/") || s.starts_with("tests\\"))
                    && s.ends_with(".rs")
                    && !s.contains("common/")
                    && !s.contains("common\\")
                    && !s.ends_with("mod.rs")
            })
            .collect();

        if test_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &test_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if !valid_suffixes.iter().any(|suffix| filename.ends_with(suffix)) {
                violations.push(make_violation(
                    &self.def,
                    Some(file),
                    &format!(
                        "Test file '{}' does not use a valid suffix (expected one of: {})",
                        filename,
                        valid_suffixes.join(", ")
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

/// Check 34: Test functions use category prefixes (security_, e2e_, etc.).
pub struct TestFnPrefixes {
    pub def: RuleDef,
}

impl CheckRunner for TestFnPrefixes {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let test_fn_re = Regex::new(r"#\[test\]\s*(?:\n\s*)*fn\s+(\w+)").unwrap();
        let valid_prefixes = [
            "security_", "e2e_", "stress_test_", "perf_", "load_",
            "test_", // standard Rust test prefix
        ];

        let test_files: Vec<_> = ctx.files().iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("tests/") || s.starts_with("tests\\"))
                    && s.ends_with(".rs")
            })
            .collect();

        if test_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &test_files {
            let full = ctx.root.join(file);
            let content = match std::fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for cap in test_fn_re.captures_iter(&content) {
                let fn_name = &cap[1];
                if !valid_prefixes.iter().any(|prefix| fn_name.starts_with(prefix)) {
                    violations.push(make_violation(
                        &self.def,
                        Some(file),
                        &format!("Test function '{}' does not use a valid category prefix", fn_name),
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

/// Check 35: Test functions use scenario suffixes (_happy, _error, etc.).
pub struct TestFnSuffixes {
    pub def: RuleDef,
}

impl CheckRunner for TestFnSuffixes {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let test_fn_re = Regex::new(r"#\[test\]\s*(?:\n\s*)*fn\s+(\w+)").unwrap();
        let valid_suffixes = [
            "_happy", "_error", "_edge", "_regression", "_contract",
            "_config", "_state", "_observe",
        ];

        let test_files: Vec<_> = ctx.files().iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("tests/") || s.starts_with("tests\\"))
                    && s.ends_with(".rs")
            })
            .collect();

        if test_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &test_files {
            let full = ctx.root.join(file);
            let content = match std::fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for cap in test_fn_re.captures_iter(&content) {
                let fn_name = &cap[1];
                if !valid_suffixes.iter().any(|suffix| fn_name.ends_with(suffix)) {
                    violations.push(make_violation(
                        &self.def,
                        Some(file),
                        &format!("Test function '{}' does not use a valid scenario suffix", fn_name),
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

/// Check 36: Integration tests in tests/src/ (rustboot).
pub struct IntTestsLocation {
    pub def: RuleDef,
}

impl CheckRunner for IntTestsLocation {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let tests_src = ctx.root.join("tests/src");
        if !tests_src.exists() {
            // Check if there are any test files in tests/ that should be in tests/src/
            let test_files: Vec<_> = ctx.files().iter()
                .filter(|f| {
                    let s = f.to_string_lossy().replace('\\', "/");
                    s.starts_with("tests/") && s.ends_with(".rs")
                        && !s.starts_with("tests/src/")
                        && !s.contains("common/")
                        && !s.ends_with("mod.rs")
                })
                .collect();

            if test_files.is_empty() {
                return CheckResult::Pass;
            }

            let mut violations = Vec::new();
            for file in test_files {
                violations.push(make_violation(
                    &self.def,
                    Some(file),
                    &format!("Integration test '{}' should be in tests/src/", file.display()),
                ));
            }
            return CheckResult::Fail { violations };
        }

        CheckResult::Pass
    }
}

/// Check 37: Unit tests colocated in source files.
pub struct UnitTestsColocated {
    pub def: RuleDef,
}

impl CheckRunner for UnitTestsColocated {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        // Look for source files that have code but no #[cfg(test)] module
        let cfg_test_re = Regex::new(r"#\[cfg\(test\)\]").unwrap();

        let source_files: Vec<_> = ctx.files().iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                (s.starts_with("src/") || s.starts_with("main/src/"))
                    && s.ends_with(".rs")
                    && !s.ends_with("mod.rs")
                    && !s.ends_with("lib.rs")
                    && !s.ends_with("main.rs")
            })
            .collect();

        if source_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &source_files {
            let full = ctx.root.join(file);
            let content = match std::fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Only flag files that have substantial code (>10 lines)
            if content.lines().count() > 10 && !cfg_test_re.is_match(&content) {
                violations.push(make_violation(
                    &self.def,
                    Some(file),
                    &format!("Source file '{}' has no colocated #[cfg(test)] module", file.display()),
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

/// Check 38: No test code in src/ (non-#[cfg(test)]).
pub struct NoTestInSrc {
    pub def: RuleDef,
}

impl CheckRunner for NoTestInSrc {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let test_attr_re = Regex::new(r"#\[test\]").unwrap();
        let cfg_test_re = Regex::new(r"#\[cfg\(test\)\]").unwrap();

        let source_files: Vec<_> = ctx.files().iter()
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

            // Check if #[test] appears outside of #[cfg(test)] blocks
            if test_attr_re.is_match(&content) && !cfg_test_re.is_match(&content) {
                violations.push(make_violation(
                    &self.def,
                    Some(file),
                    &format!("Source file '{}' contains #[test] outside of #[cfg(test)]", file.display()),
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
