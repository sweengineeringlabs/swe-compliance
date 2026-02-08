use std::fs;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

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
            let pattern = format!(r"(?i)#{{1,3}}\s+.*{}", keyword);
            let re = Regex::new(&pattern).unwrap();
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
        let phase_re = Regex::new(r"^(\d+-[a-z_]+)$").unwrap();
        let mut phase_dirs: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(ctx.root.join("docs")) {
            for entry in entries.flatten() {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if phase_re.is_match(&name) {
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
        let deep_link_re = Regex::new(r"\]\(docs/\d+-[^)]+\)").unwrap();

        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if deep_link_re.is_match(line) {
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
