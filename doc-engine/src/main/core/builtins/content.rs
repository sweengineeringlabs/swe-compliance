use std::fs;
use std::sync::LazyLock;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

static TLDR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\*\*TLDR\*\*|## TLDR|## TL;DR").unwrap());
static GLOSSARY_TERM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*\*[^*]+\*\*").unwrap());
static GLOSSARY_VALID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*\*[^*]+\*\*\s*[-—–:]\s+\S").unwrap());
static GLOSSARY_TERM_CAPTURE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*\*([^*]+)\*\*").unwrap());
static GLOSSARY_TERM_DEF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*\*([^*]+)\*\*\s*[-—–:]\s*(.*)").unwrap());
static ACRONYM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Z]{2,}$").unwrap());

/// Checks 35-36: tldr_conditional
/// 35: Docs >=200 lines should have TLDR
/// 36: Docs <200 lines should NOT have TLDR (info only)
pub struct TldrConditional {
    pub def: RuleDef,
}

impl CheckRunner for TldrConditional {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let docs_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.starts_with("docs/") && s.ends_with(".md")
            })
            .collect();

        if docs_files.is_empty() {
            return CheckResult::Skip { reason: "No .md files in docs/".to_string() };
        }

        let mut violations = Vec::new();
        for file in &docs_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let line_count = content.lines().count();
            let has_tldr = TLDR_RE.is_match(&content);

            match self.def.id {
                35 => {
                    // Long docs (>=200 lines) should have TLDR
                    if line_count >= 200 && !has_tldr {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(file.to_path_buf()),
                            message: format!(
                                "File has {} lines but no TLDR section",
                                line_count
                            ),
                            severity: self.def.severity.clone(),
                        });
                    }
                }
                36 => {
                    // Short docs (<200 lines) shouldn't need TLDR
                    if line_count < 200 && has_tldr {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some(file.to_path_buf()),
                            message: format!(
                                "File has only {} lines but has a TLDR section (unnecessary)",
                                line_count
                            ),
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

/// Check 37: glossary_format
/// Validate **Term** - Definition. format in glossary.md
pub struct GlossaryFormat {
    pub def: RuleDef,
}

impl CheckRunner for GlossaryFormat {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let glossary_path = ctx.root.join("docs/glossary.md");
        if !glossary_path.exists() {
            return CheckResult::Skip { reason: "docs/glossary.md not found".to_string() };
        }

        let content = match fs::read_to_string(&glossary_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read glossary: {}", e),
                };
            }
        };

        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            // Only check lines that start with bold text (appear to be term definitions)
            if GLOSSARY_TERM_RE.is_match(line) && !GLOSSARY_VALID_RE.is_match(line) {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/glossary.md".into()),
                    message: format!(
                        "Line {}: Term definition doesn't follow '**Term** - Definition' format",
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

/// Check 38: glossary_alphabetized
/// Glossary terms are in alphabetical order
pub struct GlossaryAlphabetized {
    pub def: RuleDef,
}

impl CheckRunner for GlossaryAlphabetized {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let glossary_path = ctx.root.join("docs/glossary.md");
        if !glossary_path.exists() {
            return CheckResult::Skip { reason: "docs/glossary.md not found".to_string() };
        }

        let content = match fs::read_to_string(&glossary_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read glossary: {}", e),
                };
            }
        };

        let terms: Vec<String> = content.lines()
            .filter_map(|line| {
                GLOSSARY_TERM_CAPTURE_RE.captures(line.trim()).map(|caps| caps[1].to_lowercase())
            })
            .collect();

        if terms.len() < 2 {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for i in 1..terms.len() {
            if terms[i] < terms[i - 1] {
                violations.push(Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("docs/glossary.md".into()),
                    message: format!(
                        "Term '{}' should come before '{}'",
                        terms[i], terms[i - 1]
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

/// Check 39: glossary_acronyms
/// Acronym expansions are present
pub struct GlossaryAcronyms {
    pub def: RuleDef,
}

impl CheckRunner for GlossaryAcronyms {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let glossary_path = ctx.root.join("docs/glossary.md");
        if !glossary_path.exists() {
            return CheckResult::Skip { reason: "docs/glossary.md not found".to_string() };
        }

        let content = match fs::read_to_string(&glossary_path) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read glossary: {}", e),
                };
            }
        };

        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if let Some(caps) = GLOSSARY_TERM_DEF_RE.captures(line.trim()) {
                let term = &caps[1];
                let definition = &caps[2];

                // If term looks like an acronym (all caps, 2+ letters)
                if ACRONYM_RE.is_match(term) {
                    // Check that definition contains expansion (at least some lowercase words)
                    let has_expansion = definition.split_whitespace()
                        .any(|w| w.chars().any(|c| c.is_lowercase()));
                    if !has_expansion {
                        violations.push(Violation {
                            check_id: CheckId(self.def.id),
                            path: Some("docs/glossary.md".into()),
                            message: format!(
                                "Line {}: Acronym '{}' lacks expansion in definition",
                                i + 1, term
                            ),
                            severity: self.def.severity.clone(),
                        });
                    }
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

/// Check 75: readme_line_count
/// Root README.md should be under 100 lines.
pub struct ReadmeLineCount {
    pub def: RuleDef,
}

impl CheckRunner for ReadmeLineCount {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let readme = ctx.root.join("README.md");
        if !readme.exists() {
            return CheckResult::Skip { reason: "README.md not found".to_string() };
        }

        let content = match fs::read_to_string(&readme) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read README.md: {}", e),
                };
            }
        };

        let line_count = content.lines().count();
        if line_count <= 100 {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![Violation {
                    check_id: CheckId(self.def.id),
                    path: Some("README.md".into()),
                    message: format!(
                        "README.md has {} lines; should be under 100 lines",
                        line_count
                    ),
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
    use crate::spi::types::{ProjectScope, ProjectType, Severity};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn make_def(id: u8) -> RuleDef {
        RuleDef {
            id,
            category: "content".to_string(),
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

    // --- TldrConditional (checks 35, 36) ---

    #[test]
    fn test_tldr_long_no_tldr_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        // 200+ lines with no TLDR
        let content = (0..210).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        fs::write(tmp.path().join("docs/long.md"), &content).unwrap();
        let handler = TldrConditional { def: make_def(35) };
        let ctx = make_ctx(tmp.path(), vec![PathBuf::from("docs/long.md")]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_tldr_short_with_tldr_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/short.md"), "# Title\n\n## TLDR\nShort doc\n").unwrap();
        let handler = TldrConditional { def: make_def(36) };
        let ctx = make_ctx(tmp.path(), vec![PathBuf::from("docs/short.md")]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_tldr_long_with_tldr_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        let mut content: Vec<String> = (0..210).map(|i| format!("Line {}", i)).collect();
        content.insert(0, "## TLDR\nSummary here".to_string());
        fs::write(tmp.path().join("docs/long.md"), content.join("\n")).unwrap();
        let handler = TldrConditional { def: make_def(35) };
        let ctx = make_ctx(tmp.path(), vec![PathBuf::from("docs/long.md")]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_tldr_short_no_tldr_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/short.md"), "# Title\nShort content\n").unwrap();
        let handler = TldrConditional { def: make_def(36) };
        let ctx = make_ctx(tmp.path(), vec![PathBuf::from("docs/short.md")]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    // --- GlossaryFormat (check 37) ---

    #[test]
    fn test_glossary_format_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "# Glossary\n\n**API** - Application Programming Interface\n**CLI** - Command Line Interface\n"
        ).unwrap();
        let handler = GlossaryFormat { def: make_def(37) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_glossary_format_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "# Glossary\n\n**API**\n"
        ).unwrap();
        let handler = GlossaryFormat { def: make_def(37) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_glossary_format_skip() {
        let tmp = TempDir::new().unwrap();
        let handler = GlossaryFormat { def: make_def(37) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }

    // --- GlossaryAlphabetized (check 38) ---

    #[test]
    fn test_glossary_alphabetized_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "**API** - Application\n**CLI** - Command\n**SDK** - Software\n"
        ).unwrap();
        let handler = GlossaryAlphabetized { def: make_def(38) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_glossary_alphabetized_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "**SDK** - Software\n**API** - Application\n"
        ).unwrap();
        let handler = GlossaryAlphabetized { def: make_def(38) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- GlossaryAcronyms (check 39) ---

    #[test]
    fn test_glossary_acronyms_pass() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "**API** - Application Programming Interface\n"
        ).unwrap();
        let handler = GlossaryAcronyms { def: make_def(39) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_glossary_acronyms_fail() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/glossary.md"),
            "**API** - API API API\n"
        ).unwrap();
        let handler = GlossaryAcronyms { def: make_def(39) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    // --- ReadmeLineCount (check 75) ---

    #[test]
    fn test_readme_line_count_pass() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# Project\nShort readme.\n").unwrap();
        let handler = ReadmeLineCount { def: make_def(75) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Pass));
    }

    #[test]
    fn test_readme_line_count_fail() {
        let tmp = TempDir::new().unwrap();
        let content: String = (0..105).map(|i| format!("Line {}\n", i)).collect();
        fs::write(tmp.path().join("README.md"), &content).unwrap();
        let handler = ReadmeLineCount { def: make_def(75) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Fail { .. }));
    }

    #[test]
    fn test_readme_line_count_skip() {
        let tmp = TempDir::new().unwrap();
        let handler = ReadmeLineCount { def: make_def(75) };
        let ctx = make_ctx(tmp.path(), vec![]);
        assert!(matches!(handler.run(&ctx), CheckResult::Skip { .. }));
    }
}
