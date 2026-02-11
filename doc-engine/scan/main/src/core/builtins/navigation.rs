use std::fs;
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::api::traits::CheckRunner;
use crate::api::types::{CheckId, CheckResult, ScanContext, Violation};

static W3H_WHO_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*who").unwrap());
static W3H_WHAT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*what").unwrap());
static W3H_WHY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*why").unwrap());
static W3H_HOW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)#{1,3}\s+.*how").unwrap());
static NAV_PHASE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+-[a-z_]+)$").unwrap());
static DEEP_LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\]\(docs/\d+-[^)]+\)").unwrap());

fn w3h_re(keyword: &str) -> &'static LazyLock<Regex> {
    match keyword {
        "who" => &W3H_WHO_RE,
        "what" => &W3H_WHAT_RE,
        "why" => &W3H_WHY_RE,
        "how" => &W3H_HOW_RE,
        _ => unreachable!(),
    }
}

/// Check 41: w3h_hub
/// Detect W3H (WHO-WHAT-WHY-HOW) structure in docs/README.md
pub struct W3hHub {
    pub def: RuleDef,
}

impl CheckRunner for W3hHub {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let hub_path = ctx.root.join("docs/README.md");
        if !hub_path.exists() {
            return CheckResult::Skip { reason: "docs/README.md not found".to_string() };
        }

        let content = match fs::read_to_string(&hub_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read docs/README.md: {}", e),
                };
            }
        };

        let content_lower = content.to_lowercase();
        let w3h_keywords = ["who", "what", "why", "how"];
        let mut missing = Vec::new();

        for keyword in &w3h_keywords {
            // Look for section headers containing the keyword
            let re = w3h_re(keyword);
            if !re.is_match(&content) && !content_lower.contains(&format!("**{}**", keyword)) {
                missing.push(*keyword);
            }
        }

        if missing.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/README.md".into()),
                    message: format!(
                        "Hub document missing W3H sections: {}",
                        missing.join(", ")
                    ),
                    severity: self.def.severity.clone(),
                }],
            }
        }
    }
}

/// Check 42: hub_links_phases
/// Hub document links to all present SDLC phase directories
pub struct HubLinksPhases {
    pub def: RuleDef,
}

impl CheckRunner for HubLinksPhases {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let hub_path = ctx.root.join("docs/README.md");
        if !hub_path.exists() {
            return CheckResult::Skip { reason: "docs/README.md not found".to_string() };
        }

        let content = match fs::read_to_string(&hub_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read docs/README.md: {}", e),
                };
            }
        };

        // Find all phase directories
        let mut phase_dirs: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(ctx.root.join("docs")) {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if NAV_PHASE_RE.is_match(&name) {
                        phase_dirs.push(name);
                    }
                }
            }
        }

        if phase_dirs.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for dir in &phase_dirs {
            // Check if the hub links to this phase directory
            if !content.contains(dir) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/README.md".into()),
                    message: format!("Hub does not link to phase directory '{}'", dir),
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

/// Check 43: no_deep_links
/// Root README doesn't deep-link into docs/ subdirectories
pub struct NoDeepLinks {
    pub def: RuleDef,
}

impl CheckRunner for NoDeepLinks {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let readme_path = ctx.root.join("README.md");
        if !readme_path.exists() {
            return CheckResult::Skip { reason: "README.md not found".to_string() };
        }

        let content = match fs::read_to_string(&readme_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read README.md: {}", e),
                };
            }
        };

        // Deep links go into docs/ subdirectories (e.g., docs/3-design/architecture.md)
        // Allowed: links to docs/README.md, docs/glossary.md (top-level docs files)
        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if DEEP_LINK_RE.is_match(line) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("README.md".into()),
                    message: format!(
                        "Line {}: Root README deep-links into docs/ subdirectory",
                        i + 1
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

/// Check 74: w3h_extended
/// Check hub documents (architecture.md, developer_guide.md) for W3H sections.
/// Only checks What/Why/How (not Who, since Audience check 33 handles that).
pub struct W3hExtended {
    pub def: RuleDef,
}

impl CheckRunner for W3hExtended {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let hub_files = [
            "docs/3-design/architecture.md",
            "docs/4-development/developer_guide.md",
        ];
        let w3h_keywords = ["what", "why", "how"];
        let mut violations = Vec::new();

        for file_path in &hub_files {
            let full = ctx.root.join(file_path);
            if !full.exists() {
                continue; // skip files that don't exist
            }

            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let mut missing = Vec::new();
            for keyword in &w3h_keywords {
                let re = w3h_re(keyword);
                if !re.is_match(&content) {
                    missing.push(*keyword);
                }
            }

            if !missing.is_empty() {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some((*file_path).into()),
                    message: format!(
                        "Hub document '{}' missing W3H sections: {}",
                        file_path, missing.join(", ")
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
    use crate::api::types::{ProjectScope, ProjectType, Severity};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn make_def(id: u8) -> RuleDef {
        RuleDef {
            id,
            category: "navigation".to_string(),
            description: "test".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: "test".to_string() },
            project_type: None,
            scope: None,
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

    // --- W3hHub (check 41) ---

    #[test]
    fn test_w3h_hub_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/README.md"),
            "# Hub\n## Who\nTeam\n## What\nProduct\n## Why\nReason\n## How\nProcess\n"
        ).unwrap();
        let handler = W3hHub { def: make_def(41) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_w3h_hub_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/README.md"), "# Hub\nJust some text\n").unwrap();
        let handler = W3hHub { def: make_def(41) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_w3h_hub_skip() {
        let tmp = TempDir::new().unwrap();
        let handler = W3hHub { def: make_def(41) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- HubLinksPhases (check 42) ---

    #[test]
    fn test_hub_links_phases_pass() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("0-overview")).unwrap();
        fs::create_dir_all(docs.join("1-requirements")).unwrap();
        fs::write(docs.join("README.md"),
            "# Hub\n- [Overview](0-overview/)\n- [Requirements](1-requirements/)\n"
        ).unwrap();
        let handler = HubLinksPhases { def: make_def(42) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_hub_links_phases_fail() {
        let tmp = TempDir::new().unwrap();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(docs.join("0-overview")).unwrap();
        fs::create_dir_all(docs.join("1-requirements")).unwrap();
        fs::write(docs.join("README.md"), "# Hub\nNo links here\n").unwrap();
        let handler = HubLinksPhases { def: make_def(42) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- NoDeepLinks (check 43) ---

    #[test]
    fn test_no_deep_links_pass() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"),
            "# Project\nSee [docs](docs/README.md) for details.\n"
        ).unwrap();
        let handler = NoDeepLinks { def: make_def(43) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_no_deep_links_fail() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"),
            "# Project\nSee [architecture](docs/3-design/architecture.md) for details.\n"
        ).unwrap();
        let handler = NoDeepLinks { def: make_def(43) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- W3hExtended (check 74) ---

    #[test]
    fn test_w3h_extended_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/3-design")).unwrap();
        fs::write(tmp.path().join("docs/3-design/architecture.md"),
            "# Architecture\n## What\nSystem\n## Why\nReason\n## How\nProcess\n"
        ).unwrap();
        let handler = W3hExtended { def: make_def(74) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_w3h_extended_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/3-design")).unwrap();
        fs::write(tmp.path().join("docs/3-design/architecture.md"),
            "# Architecture\nJust text\n"
        ).unwrap();
        let handler = W3hExtended { def: make_def(74) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_w3h_extended_skip_no_files() {
        let tmp = TempDir::new().unwrap();
        let handler = W3hExtended { def: make_def(74) };
        let ctx = make_ctx(tmp.path(), vec![]);
        // No hub files exist — pass (nothing to check)
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }
}
