use std::fs;
use std::path::Path;

use regex::Regex;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

/// Checks 44-45: link_resolution
/// 44: All internal markdown links resolve to existing files (error)
/// 45: All relative links are valid (warning)
pub struct LinkResolution {
    pub def: RuleDef,
}

impl CheckRunner for LinkResolution {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();

        let md_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let s = f.to_string_lossy();
                s.starts_with("docs/") && s.ends_with(".md")
            })
            .collect();

        if md_files.is_empty() {
            return CheckResult::Skip { reason: "No .md files in docs/".to_string() };
        }

        let mut violations = Vec::new();
        for file in &md_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let file_dir = file.parent().unwrap_or(Path::new(""));

            for caps in link_re.captures_iter(&content) {
                let target = &caps[2];

                // Skip external links, anchors, and mailto
                if target.starts_with("http://") || target.starts_with("https://")
                    || target.starts_with('#') || target.starts_with("mailto:")
                {
                    continue;
                }

                // Strip anchor from link
                let target_path = target.split('#').next().unwrap_or(target);
                if target_path.is_empty() {
                    continue;
                }

                // Resolve relative to the file's directory
                let resolved = if target_path.starts_with('/') {
                    ctx.root.join(target_path.trim_start_matches('/'))
                } else {
                    ctx.root.join(file_dir).join(target_path)
                };

                match self.def.id {
                    44 => {
                        // Check internal doc links resolve
                        if target_path.ends_with(".md") && !resolved.exists() {
                            violations.push(Violation {
                                check_id: CheckId(self.def.id),
                                path: Some(file.to_path_buf()),
                                message: format!("Broken link: '{}' does not exist", target),
                                severity: self.def.severity.clone(),
                            });
                        }
                    }
                    45 => {
                        // Check all relative links resolve
                        if !target_path.starts_with('/') && !resolved.exists() {
                            violations.push(Violation {
                                check_id: CheckId(self.def.id),
                                path: Some(file.to_path_buf()),
                                message: format!("Broken relative link: '{}' does not exist", target),
                                severity: self.def.severity.clone(),
                            });
                        }
                    }
                    _ => {}
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
