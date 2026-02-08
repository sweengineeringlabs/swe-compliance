use std::fs;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

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
        let tldr_re = Regex::new(r"(?i)\*\*TLDR\*\*|## TLDR|## TL;DR").unwrap();

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
            let has_tldr = tldr_re.is_match(&content);

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

        let term_re = Regex::new(r"^\*\*[^*]+\*\*").unwrap();
        let valid_re = Regex::new(r"^\*\*[^*]+\*\*\s*[-—–:]\s+\S").unwrap();

        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            // Only check lines that start with bold text (appear to be term definitions)
            if term_re.is_match(line) && !valid_re.is_match(line) {
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

        let term_re = Regex::new(r"^\*\*([^*]+)\*\*").unwrap();
        let terms: Vec<String> = content.lines()
            .filter_map(|line| {
                term_re.captures(line.trim()).map(|caps| caps[1].to_lowercase())
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

        let term_re = Regex::new(r"^\*\*([^*]+)\*\*\s*[-—–:]\s*(.*)").unwrap();
        let acronym_re = Regex::new(r"^[A-Z]{2,}$").unwrap();

        let mut violations = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if let Some(caps) = term_re.captures(line.trim()) {
                let term = &caps[1];
                let definition = &caps[2];

                // If term looks like an acronym (all caps, 2+ letters)
                if acronym_re.is_match(term) {
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
