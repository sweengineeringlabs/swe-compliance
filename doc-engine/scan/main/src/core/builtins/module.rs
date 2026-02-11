use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::api::traits::CheckRunner;
use crate::api::types::{CheckId, CheckResult, ScanContext, Violation};

static MODULE_W3H_WHAT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*what").unwrap());
static MODULE_W3H_WHY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*why").unwrap());
static MODULE_W3H_HOW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*how").unwrap());

fn module_w3h_re(keyword: &str) -> &'static LazyLock<Regex> {
    match keyword {
        "what" => &MODULE_W3H_WHAT_RE,
        "why" => &MODULE_W3H_WHY_RE,
        "how" => &MODULE_W3H_HOW_RE,
        _ => unreachable!(),
    }
}

/// A discovered module with its relative path and name.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub path: PathBuf,
    pub name: String,
}

/// Manifest files that indicate a directory is a module/package root.
const MANIFEST_FILES: &[&str] = &[
    "Cargo.toml",
    "package.json",
    "pyproject.toml",
    "go.mod",
    "pom.xml",
    "build.gradle",
];

/// Directories to scan for modules beneath the project root.
const MODULE_DIRS: &[&str] = &["modules", "packages", "crates"];

/// Discover modules by scanning known directories for manifest files (FR-800).
/// Skips the project root itself.
pub fn discover_modules(ctx: &ScanContext) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();

    // Scan direct children of module directories
    for dir_name in MODULE_DIRS {
        let dir = ctx.root.join(dir_name);
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                        let sub = entry.path();
                        if has_manifest(&sub) {
                            let rel = sub.strip_prefix(&ctx.root).unwrap_or(&sub);
                            modules.push(ModuleInfo {
                                path: rel.to_path_buf(),
                                name: entry.file_name().to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Also scan direct children of root (for flat module layouts)
    if let Ok(entries) = fs::read_dir(&ctx.root) {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                let name = entry.file_name().to_string_lossy().to_string();
                // Skip known non-module directories
                if name.starts_with('.') || name == "docs" || name == "target"
                    || name == "node_modules" || MODULE_DIRS.contains(&name.as_str()) {
                    continue;
                }
                let sub = entry.path();
                if has_manifest(&sub) {
                    let rel = sub.strip_prefix(&ctx.root).unwrap_or(&sub);
                    modules.push(ModuleInfo {
                        path: rel.to_path_buf(),
                        name,
                    });
                }
            }
        }
    }

    modules.sort_by(|a, b| a.path.cmp(&b.path));
    modules
}

fn has_manifest(dir: &Path) -> bool {
    MANIFEST_FILES.iter().any(|f| dir.join(f).exists())
}

// ---------------------------------------------------------------------------
// Check 77: module_readme_w3h — Module READMEs follow W3H structure
// ---------------------------------------------------------------------------

pub struct ModuleReadmeW3h {
    pub def: RuleDef,
}

impl CheckRunner for ModuleReadmeW3h {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let modules = discover_modules(ctx);
        if modules.is_empty() {
            return CheckResult::Pass; // vacuously true
        }

        let w3h_keywords = ["what", "why", "how"];
        let mut violations = Vec::new();

        for m in &modules {
            let readme = ctx.root.join(&m.path).join("docs/README.md");
            if !readme.exists() {
                continue; // skip modules without docs/README.md
            }

            let content = match fs::read_to_string(&readme) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let mut missing = Vec::new();
            for keyword in &w3h_keywords {
                let re = module_w3h_re(keyword);
                if !re.is_match(&content) {
                    missing.push(*keyword);
                }
            }

            if !missing.is_empty() {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(m.path.join("docs/README.md")),
                    message: format!(
                        "Module '{}' README missing W3H sections: {}",
                        m.name, missing.join(", ")
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

// ---------------------------------------------------------------------------
// Checks 78-79: module_examples_tests — Module examples and integration tests
// ---------------------------------------------------------------------------

pub struct ModuleExamplesTests {
    pub def: RuleDef,
}

impl CheckRunner for ModuleExamplesTests {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let modules = discover_modules(ctx);
        if modules.is_empty() {
            return CheckResult::Pass; // vacuously true
        }

        let mut violations = Vec::new();
        for m in &modules {
            let module_root = ctx.root.join(&m.path);
            match self.def.id {
                78 => {
                    // Check examples/ has >=1 file
                    let examples = module_root.join("examples");
                    if !examples.is_dir() || !dir_has_files(&examples) {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(m.path.join("examples")),
                            message: format!("Module '{}' missing examples/ directory with files", m.name),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                79 => {
                    // Check tests/ has >=1 file
                    let tests = module_root.join("tests");
                    if !tests.is_dir() || !dir_has_files(&tests) {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(m.path.join("tests")),
                            message: format!("Module '{}' missing tests/ directory with files", m.name),
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

// ---------------------------------------------------------------------------
// Check 80: module_toolchain_docs — Modules have toolchain documentation
// ---------------------------------------------------------------------------

pub struct ModuleToolchainDocs {
    pub def: RuleDef,
}

impl CheckRunner for ModuleToolchainDocs {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let modules = discover_modules(ctx);
        if modules.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for m in &modules {
            let toolchain = ctx.root.join(&m.path).join("docs/3-design/toolchain.md");
            if !toolchain.exists() {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(m.path.join("docs/3-design/toolchain.md")),
                    message: format!("Module '{}' missing docs/3-design/toolchain.md", m.name),
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

// ---------------------------------------------------------------------------
// Check 81: module_deployment_docs — Module deployment docs complete (FR-802)
// ---------------------------------------------------------------------------

pub struct ModuleDeploymentDocs {
    pub def: RuleDef,
}

impl CheckRunner for ModuleDeploymentDocs {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let modules = discover_modules(ctx);
        if modules.is_empty() {
            return CheckResult::Pass;
        }

        let required_files = ["README.md", "prerequisites.md", "installation.md"];
        let mut violations = Vec::new();

        for m in &modules {
            let deploy_dir = ctx.root.join(&m.path).join("docs/6-deployment");
            // FR-802: skip modules without docs/6-deployment/
            if !deploy_dir.is_dir() {
                continue;
            }

            for file in &required_files {
                if !deploy_dir.join(file).exists() {
                    violations.push(Violation {
                        check_id: CheckId(self.def.id),
                        path: Some(m.path.join(format!("docs/6-deployment/{}", file))),
                        message: format!(
                            "Module '{}' deployment directory missing {}",
                            m.name, file
                        ),
                        severity: self.def.severity.clone(),
                    });
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

/// Check if a directory contains at least one file (not just subdirs).
fn dir_has_files(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|ft| ft.is_file()) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{RuleDef, RuleType};
    use crate::api::types::{ProjectScope, ProjectType, Severity};
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_def(id: u8, handler: &str) -> RuleDef {
        RuleDef {
            id,
            category: "module".to_string(),
            description: "test".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: handler.to_string() },
            project_type: None,
            scope: None,
            depends_on: vec![],
        }
    }

    fn make_ctx(root: &std::path::Path, files: Vec<PathBuf>) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_type: ProjectType::OpenSource,
            project_scope: ProjectScope::Large,
        }
    }

    // --- discover_modules ---

    #[test]
    fn test_discover_no_modules() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert!(modules.is_empty());
    }

    #[test]
    fn test_discover_single_module() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "auth");
    }

    #[test]
    fn test_discover_multiple_modules() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::create_dir_all(tmp.path().join("crates/core")).unwrap();
        fs::write(tmp.path().join("crates/core/Cargo.toml"), "[package]").unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert_eq!(modules.len(), 2);
    }

    #[test]
    fn test_discover_nested_module_dirs() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("modules/api")).unwrap();
        fs::write(tmp.path().join("modules/api/package.json"), "{}").unwrap();
        fs::create_dir_all(tmp.path().join("packages/util")).unwrap();
        fs::write(tmp.path().join("packages/util/pyproject.toml"), "").unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert_eq!(modules.len(), 2);
    }

    #[test]
    fn test_discover_root_child_module() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("mylib")).unwrap();
        fs::write(tmp.path().join("mylib/Cargo.toml"), "[package]").unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "mylib");
    }

    #[test]
    fn test_discover_skips_docs_dir() {
        let tmp = TempDir::new().unwrap();
        // docs/ with a Cargo.toml should NOT be treated as a module
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/Cargo.toml"), "[package]").unwrap();
        let ctx = make_ctx(tmp.path(), vec![]);
        let modules = discover_modules(&ctx);
        assert!(modules.is_empty());
    }

    // --- ModuleReadmeW3h (check 77) ---

    #[test]
    fn test_module_readme_w3h_pass_no_modules() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleReadmeW3h { def: make_def(77, "module_readme_w3h") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_readme_w3h_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth/docs")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("crates/auth/docs/README.md"),
            "# Auth\n## What\nAuth module\n## Why\nSecurity\n## How\nOAuth\n"
        ).unwrap();
        let handler = ModuleReadmeW3h { def: make_def(77, "module_readme_w3h") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_readme_w3h_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth/docs")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("crates/auth/docs/README.md"),
            "# Auth\nJust text\n"
        ).unwrap();
        let handler = ModuleReadmeW3h { def: make_def(77, "module_readme_w3h") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_module_readme_w3h_skip_no_readme() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        // No docs/README.md — should pass (skip module)
        let handler = ModuleReadmeW3h { def: make_def(77, "module_readme_w3h") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // --- ModuleExamplesTests (checks 78-79) ---

    #[test]
    fn test_module_examples_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth/examples")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("crates/auth/examples/basic.rs"), "fn main() {}").unwrap();
        let handler = ModuleExamplesTests { def: make_def(78, "module_examples_tests") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_examples_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        let handler = ModuleExamplesTests { def: make_def(78, "module_examples_tests") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_module_tests_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth/tests")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("crates/auth/tests/integration.rs"), "#[test] fn t() {}").unwrap();
        let handler = ModuleExamplesTests { def: make_def(79, "module_examples_tests") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_tests_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        let handler = ModuleExamplesTests { def: make_def(79, "module_examples_tests") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_module_examples_pass_no_modules() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleExamplesTests { def: make_def(78, "module_examples_tests") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // --- ModuleToolchainDocs (check 80) ---

    #[test]
    fn test_module_toolchain_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth/docs/3-design")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("crates/auth/docs/3-design/toolchain.md"), "# Toolchain").unwrap();
        let handler = ModuleToolchainDocs { def: make_def(80, "module_toolchain_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_toolchain_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        let handler = ModuleToolchainDocs { def: make_def(80, "module_toolchain_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_module_toolchain_pass_no_modules() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleToolchainDocs { def: make_def(80, "module_toolchain_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // --- ModuleDeploymentDocs (check 81) ---

    #[test]
    fn test_module_deployment_pass() {
        let tmp = TempDir::new().unwrap();
        let deploy = tmp.path().join("crates/auth/docs/6-deployment");
        fs::create_dir_all(&deploy).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(deploy.join("README.md"), "# Deploy").unwrap();
        fs::write(deploy.join("prerequisites.md"), "# Prerequisites").unwrap();
        fs::write(deploy.join("installation.md"), "# Install").unwrap();
        let handler = ModuleDeploymentDocs { def: make_def(81, "module_deployment_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_deployment_fail_missing_files() {
        let tmp = TempDir::new().unwrap();
        let deploy = tmp.path().join("crates/auth/docs/6-deployment");
        fs::create_dir_all(&deploy).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        fs::write(deploy.join("README.md"), "# Deploy").unwrap();
        // missing prerequisites.md and installation.md
        let handler = ModuleDeploymentDocs { def: make_def(81, "module_deployment_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        let result = handler.run(&ctx);
        match result {
            CheckResult::Fail { violations } => assert_eq!(violations.len(), 2),
            other => panic!("Expected Fail, got {:?}", other),
        }
    }

    #[test]
    fn test_module_deployment_skip_no_deploy_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("crates/auth")).unwrap();
        fs::write(tmp.path().join("crates/auth/Cargo.toml"), "[package]").unwrap();
        // No docs/6-deployment/ — FR-802: skip
        let handler = ModuleDeploymentDocs { def: make_def(81, "module_deployment_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_module_deployment_pass_no_modules() {
        let tmp = TempDir::new().unwrap();
        let handler = ModuleDeploymentDocs { def: make_def(81, "module_deployment_docs") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }
}
