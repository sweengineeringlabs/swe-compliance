use std::fs;
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

static DESIGN_REQ_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)requirements\.md|FR-\d|STK-\d|SRS|1-requirements").unwrap()
});
static PLAN_ARCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)architecture\.md|3-design|architectural").unwrap()
});
static BACKLOG_REQ_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)requirements\.md|requirements\b|FR-\d|STK-\d|SRS|1-requirements|BL-\d").unwrap()
});

/// Check 51: phase_artifact_presence
/// Populated SDLC phase directories contain their expected artifact.
pub struct PhaseArtifactPresence {
    pub def: RuleDef,
}

impl CheckRunner for PhaseArtifactPresence {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let phase_expectations: &[(&str, &[&str])] = &[
            ("docs/1-requirements", &["requirements", "srs"]),
            ("docs/2-planning", &["plan", "implementation"]),
            ("docs/3-design", &["architecture.md"]),
        ];

        // Check which phase dirs exist
        let existing_phases: Vec<_> = phase_expectations.iter()
            .filter(|(dir, _)| ctx.root.join(dir).is_dir())
            .collect();

        if existing_phases.is_empty() {
            return CheckResult::Skip { reason: "No SDLC phase directories exist".to_string() };
        }

        let mut violations = Vec::new();

        for (dir, expected_patterns) in &existing_phases {
            let full_dir = ctx.root.join(dir);

            // Check if any file in this dir matches an expected pattern
            let has_artifact = ctx.files.iter().any(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                if !s.starts_with(*dir) {
                    return false;
                }
                // Only look at direct children (not subdirs)
                let relative = &s[dir.len()..];
                if let Some(rest) = relative.strip_prefix('/') {
                    // Skip files in subdirectories
                    if rest.contains('/') {
                        return false;
                    }
                    let lower = rest.to_lowercase();
                    expected_patterns.iter().any(|pat| lower.contains(pat))
                } else {
                    false
                }
            });

            // Fallback: also check the filesystem directly for the specific expected file
            let has_artifact = has_artifact || {
                if *dir == "docs/3-design" {
                    full_dir.join("architecture.md").exists()
                } else {
                    // Read the directory and check filenames
                    match fs::read_dir(&full_dir) {
                        Ok(entries) => {
                            entries.filter_map(|e| e.ok()).any(|entry| {
                                let name = entry.file_name().to_string_lossy().to_lowercase();
                                expected_patterns.iter().any(|pat| name.contains(pat))
                            })
                        }
                        Err(_) => false,
                    }
                }
            };

            if !has_artifact {
                let expected = expected_patterns.join("' or '");
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(std::path::PathBuf::from(*dir)),
                    message: format!(
                        "Phase directory '{}' exists but is missing expected artifact containing '{}'",
                        dir, expected
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

/// Check 52: design_traces_requirements
/// Design documents reference requirements.
pub struct DesignTracesRequirements {
    pub def: RuleDef,
}

impl CheckRunner for DesignTracesRequirements {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let design_dir = "docs/3-design";
        if !ctx.root.join(design_dir).is_dir() {
            return CheckResult::Skip { reason: "docs/3-design/ does not exist".to_string() };
        }

        // Find qualifying .md files in docs/3-design/ (excluding adr/, compliance/, and README.md)
        let qualifying_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                s.starts_with("docs/3-design/")
                    && s.ends_with(".md")
                    && !s.starts_with("docs/3-design/adr/")
                    && !s.starts_with("docs/3-design/compliance/")
                    && s != "docs/3-design/README.md"
            })
            .collect();

        if qualifying_files.is_empty() {
            return CheckResult::Skip { reason: "No qualifying .md files in docs/3-design/".to_string() };
        }

        let mut violations = Vec::new();

        for file in &qualifying_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !DESIGN_REQ_RE.is_match(&content) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(file.to_path_buf()),
                    message: format!(
                        "Design document '{}' does not reference requirements (expected pattern: requirements.md, FR-N, STK-N, SRS, or 1-requirements)",
                        file.display()
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

/// Check 53: plan_traces_design
/// Planning documents reference architecture.
pub struct PlanTracesDesign {
    pub def: RuleDef,
}

impl CheckRunner for PlanTracesDesign {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let planning_dir = "docs/2-planning";
        if !ctx.root.join(planning_dir).is_dir() {
            return CheckResult::Skip { reason: "docs/2-planning/ does not exist".to_string() };
        }

        // Find qualifying .md files in docs/2-planning/ (excluding README.md)
        let qualifying_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy().replace('\\', "/");
                s.starts_with("docs/2-planning/")
                    && s.ends_with(".md")
                    && s != "docs/2-planning/README.md"
            })
            .collect();

        if qualifying_files.is_empty() {
            return CheckResult::Skip { reason: "No qualifying .md files in docs/2-planning/".to_string() };
        }

        let mut violations = Vec::new();

        for file in &qualifying_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !PLAN_ARCH_RE.is_match(&content) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some(file.to_path_buf()),
                    message: format!(
                        "Planning document '{}' does not reference architecture (expected pattern: architecture.md, 3-design, or architectural)",
                        file.display()
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

/// Check 82: backlog_traces_requirements
/// Backlog documents reference requirements/SRS.
pub struct BacklogTracesRequirements {
    pub def: RuleDef,
}

impl CheckRunner for BacklogTracesRequirements {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let backlog_path = ctx.root.join("docs/2-planning/backlog.md");
        if !backlog_path.exists() {
            return CheckResult::Skip { reason: "docs/2-planning/backlog.md does not exist".to_string() };
        }

        let content = match fs::read_to_string(&backlog_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read backlog.md: {}", e),
                };
            }
        };

        if BACKLOG_REQ_RE.is_match(&content) {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/2-planning/backlog.md".into()),
                    message: "Backlog does not reference requirements (expected: requirements.md, FR-N, STK-N, SRS, 1-requirements, or BL-N)".to_string(),
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
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    fn make_def(id: u8, category: &str, handler: &str) -> RuleDef {
        RuleDef {
            id,
            category: category.to_string(),
            description: "test".to_string(),
            severity: Severity::Warning,
            rule_type: RuleType::Builtin { handler: handler.to_string() },
            project_type: None,
        }
    }

    fn make_ctx(root: &Path, files: Vec<PathBuf>) -> ScanContext {
        ScanContext {
            root: root.to_path_buf(),
            files,
            file_contents: HashMap::new(),
            project_type: ProjectType::OpenSource,
        }
    }

    fn write_file(root: &Path, relative: &str, content: &str) {
        let full = root.join(relative);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&full, content).unwrap();
    }

    // =========================================================================
    // Check 51 — PhaseArtifactPresence
    // =========================================================================

    #[test]
    fn test_phase_artifact_all_present() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/1-requirements/requirements.md", "# Requirements\n");
        write_file(tmp.path(), "docs/2-planning/implementation_plan.md", "# Plan\n");
        write_file(tmp.path(), "docs/3-design/architecture.md", "# Arch\n");

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/1-requirements/requirements.md"),
            PathBuf::from("docs/2-planning/implementation_plan.md"),
            PathBuf::from("docs/3-design/architecture.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_phase_artifact_missing_srs() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/1-requirements")).unwrap();
        write_file(tmp.path(), "docs/1-requirements/README.md", "# Readme\n");

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/1-requirements/README.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_phase_artifact_missing_plan() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/2-planning")).unwrap();
        write_file(tmp.path(), "docs/2-planning/README.md", "# Readme\n");

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/2-planning/README.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_phase_artifact_missing_arch() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs/3-design")).unwrap();
        write_file(tmp.path(), "docs/3-design/README.md", "# Readme\n");

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/3-design/README.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_phase_artifact_no_phase_dirs() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_phase_artifact_partial_phases() {
        let tmp = TempDir::new().unwrap();
        // Only 1-requirements and 3-design exist, both with correct artifacts
        write_file(tmp.path(), "docs/1-requirements/srs.md", "# SRS\n");
        write_file(tmp.path(), "docs/3-design/architecture.md", "# Arch\n");

        let handler = PhaseArtifactPresence { def: make_def(51, "traceability", "phase_artifact_presence") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/1-requirements/srs.md"),
            PathBuf::from("docs/3-design/architecture.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // =========================================================================
    // Check 52 — DesignTracesRequirements
    // =========================================================================

    #[test]
    fn test_design_traces_req_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\nSee FR-001 for details.\n");

        let handler = DesignTracesRequirements { def: make_def(52, "traceability", "design_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/3-design/architecture.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_design_traces_req_fail() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/3-design/architecture.md",
            "# Architecture\n\nThis is the architecture.\n");

        let handler = DesignTracesRequirements { def: make_def(52, "traceability", "design_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/3-design/architecture.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_design_traces_req_skip_no_dir() {
        let tmp = TempDir::new().unwrap();

        let handler = DesignTracesRequirements { def: make_def(52, "traceability", "design_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_design_traces_req_ignores_adr_compliance() {
        let tmp = TempDir::new().unwrap();
        // Only files in adr/ and compliance/ — no qualifying files
        write_file(tmp.path(), "docs/3-design/adr/001-decision.md",
            "# ADR\n\nNo requirements ref.\n");
        write_file(tmp.path(), "docs/3-design/compliance/checklist.md",
            "# Checklist\n\nNo requirements ref.\n");

        let handler = DesignTracesRequirements { def: make_def(52, "traceability", "design_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/3-design/adr/001-decision.md"),
            PathBuf::from("docs/3-design/compliance/checklist.md"),
        ]);
        // Should skip since no qualifying files remain after exclusions
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // =========================================================================
    // Check 53 — PlanTracesDesign
    // =========================================================================

    #[test]
    fn test_plan_traces_design_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/implementation_plan.md",
            "# Plan\n\nSee architecture.md for the design.\n");

        let handler = PlanTracesDesign { def: make_def(53, "traceability", "plan_traces_design") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/2-planning/implementation_plan.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_plan_traces_design_fail() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/implementation_plan.md",
            "# Plan\n\nThis is the plan.\n");

        let handler = PlanTracesDesign { def: make_def(53, "traceability", "plan_traces_design") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/2-planning/implementation_plan.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_plan_traces_design_skip_no_dir() {
        let tmp = TempDir::new().unwrap();

        let handler = PlanTracesDesign { def: make_def(53, "traceability", "plan_traces_design") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    #[test]
    fn test_plan_traces_design_ignores_readme() {
        let tmp = TempDir::new().unwrap();
        // Only README.md — should be excluded, resulting in skip
        write_file(tmp.path(), "docs/2-planning/README.md",
            "# Planning\n\nNo arch ref.\n");

        let handler = PlanTracesDesign { def: make_def(53, "traceability", "plan_traces_design") };
        let ctx = make_ctx(tmp.path(), vec![
            PathBuf::from("docs/2-planning/README.md"),
        ]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // =========================================================================
    // Check 82 — BacklogTracesRequirements
    // =========================================================================

    #[test]
    fn test_backlog_traces_req_pass() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\nDerived from requirements.md and SRS analysis.\n");

        let handler = BacklogTracesRequirements { def: make_def(82, "traceability", "backlog_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_traces_req_pass_fr_ref() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\nBL-01 maps to FR-800.\n");

        let handler = BacklogTracesRequirements { def: make_def(82, "traceability", "backlog_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_backlog_traces_req_fail() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "docs/2-planning/backlog.md",
            "# Backlog\n\nSome tasks to do.\n");

        let handler = BacklogTracesRequirements { def: make_def(82, "traceability", "backlog_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_backlog_traces_req_skip_no_file() {
        let tmp = TempDir::new().unwrap();

        let handler = BacklogTracesRequirements { def: make_def(82, "traceability", "backlog_traces_requirements") };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }
}
