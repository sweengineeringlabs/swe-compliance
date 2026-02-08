use std::fs;
use std::path::Path;

use regex::Regex;

use crate::api::types::{RuleDef, RuleType};
use crate::spi::traits::CheckRunner;
use crate::spi::types::{CheckId, CheckResult, ScanContext, Violation};

pub struct DeclarativeCheck {
    pub def: RuleDef,
}

impl CheckRunner for DeclarativeCheck {
    fn id(&self) -> CheckId {
        CheckId(self.def.id)
    }

    fn category(&self) -> &str {
        &self.def.category
    }

    fn description(&self) -> &str {
        &self.def.description
    }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        match &self.def.rule_type {
            RuleType::FileExists { path } => self.check_file_exists(ctx, path),
            RuleType::DirExists { path } => self.check_dir_exists(ctx, path),
            RuleType::DirNotExists { path, message } => self.check_dir_not_exists(ctx, path, message),
            RuleType::FileContentMatches { path, pattern } => self.check_file_content_matches(ctx, path, pattern),
            RuleType::FileContentNotMatches { path, pattern } => self.check_file_content_not_matches(ctx, path, pattern),
            RuleType::GlobContentMatches { glob, pattern } => self.check_glob_content_matches(ctx, glob, pattern),
            RuleType::GlobContentNotMatches { glob, pattern, exclude_pattern } => {
                self.check_glob_content_not_matches(ctx, glob, pattern, exclude_pattern.as_deref())
            }
            RuleType::GlobNamingMatches { glob, pattern } => self.check_glob_naming_matches(ctx, glob, pattern),
            RuleType::GlobNamingNotMatches { glob, pattern, exclude_paths } => {
                self.check_glob_naming_not_matches(ctx, glob, pattern, exclude_paths.as_deref())
            }
            RuleType::Builtin { .. } => {
                CheckResult::Skip { reason: "Builtin rules should not use DeclarativeCheck".to_string() }
            }
        }
    }
}

impl DeclarativeCheck {
    fn make_violation(&self, path: Option<&Path>, message: &str) -> Violation {
        Violation {
            check_id: CheckId(self.def.id),
            path: path.map(|p| p.to_path_buf()),
            message: message.to_string(),
            severity: self.def.severity.clone(),
        }
    }

    fn check_file_exists(&self, ctx: &ScanContext, path: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_file() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' does not exist", path),
                )],
            }
        }
    }

    fn check_dir_exists(&self, ctx: &ScanContext, path: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_dir() {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("Directory '{}' does not exist", path),
                )],
            }
        }
    }

    fn check_dir_not_exists(&self, ctx: &ScanContext, path: &str, message: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if full.exists() && full.is_dir() {
            CheckResult::Fail {
                violations: vec![self.make_violation(Some(Path::new(path)), message)],
            }
        } else {
            CheckResult::Pass
        }
    }

    fn check_file_content_matches(&self, ctx: &ScanContext, path: &str, pattern: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if !full.exists() {
            return CheckResult::Skip {
                reason: format!("File '{}' does not exist", path),
            };
        }

        let content = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read '{}': {}", path, e),
                };
            }
        };

        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        if re.is_match(&content) {
            CheckResult::Pass
        } else {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' does not match pattern '{}'", path, pattern),
                )],
            }
        }
    }

    fn check_file_content_not_matches(&self, ctx: &ScanContext, path: &str, pattern: &str) -> CheckResult {
        let full = ctx.root.join(path);
        if !full.exists() {
            return CheckResult::Pass;
        }

        let content = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Cannot read '{}': {}", path, e),
                };
            }
        };

        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        if re.is_match(&content) {
            CheckResult::Fail {
                violations: vec![self.make_violation(
                    Some(Path::new(path)),
                    &format!("File '{}' contains forbidden pattern '{}'", path, pattern),
                )],
            }
        } else {
            CheckResult::Pass
        }
    }

    fn check_glob_content_matches(&self, ctx: &ScanContext, glob: &str, pattern: &str) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let content_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files.iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !content_re.is_match(&content) {
                violations.push(self.make_violation(
                    Some(file),
                    &format!("File does not match pattern '{}'", pattern),
                ));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_content_not_matches(
        &self,
        ctx: &ScanContext,
        glob: &str,
        pattern: &str,
        exclude_pattern: Option<&str>,
    ) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let content_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let exclude_re = match exclude_pattern {
            Some(ep) => match Regex::new(ep) {
                Ok(r) => Some(r),
                Err(e) => {
                    return CheckResult::Skip {
                        reason: format!("Invalid exclude regex '{}': {}", ep, e),
                    };
                }
            },
            None => None,
        };

        let matching_files: Vec<_> = ctx.files.iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let full = ctx.root.join(file);
            let content = match fs::read_to_string(&full) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for line in content.lines() {
                if content_re.is_match(line) {
                    // If exclude_pattern is set, skip lines that also match the exclude
                    if let Some(ref excl) = exclude_re {
                        if excl.is_match(line) {
                            continue;
                        }
                    }
                    violations.push(self.make_violation(
                        Some(file),
                        &format!("File contains forbidden pattern '{}'", pattern),
                    ));
                    break; // one violation per file
                }
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_naming_matches(&self, ctx: &ScanContext, glob: &str, pattern: &str) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let name_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files.iter()
            .filter(|f| glob_re.is_match(&f.to_string_lossy()))
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if !name_re.is_match(&filename) {
                violations.push(self.make_violation(
                    Some(file),
                    &format!("Filename '{}' does not match pattern '{}'", filename, pattern),
                ));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }

    fn check_glob_naming_not_matches(
        &self,
        ctx: &ScanContext,
        glob: &str,
        pattern: &str,
        exclude_paths: Option<&[String]>,
    ) -> CheckResult {
        let glob_re = match glob_to_regex(glob) {
            Some(r) => r,
            None => {
                return CheckResult::Skip {
                    reason: format!("Invalid glob pattern '{}'", glob),
                };
            }
        };

        let name_re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return CheckResult::Skip {
                    reason: format!("Invalid regex '{}': {}", pattern, e),
                };
            }
        };

        let matching_files: Vec<_> = ctx.files.iter()
            .filter(|f| {
                let path_str = f.to_string_lossy();
                if !glob_re.is_match(&path_str) {
                    return false;
                }
                // Exclude paths that start with any prefix in exclude_paths
                if let Some(excl) = exclude_paths {
                    for prefix in excl {
                        if path_str.starts_with(prefix.as_str()) {
                            return false;
                        }
                    }
                }
                true
            })
            .collect();

        if matching_files.is_empty() {
            return CheckResult::Pass;
        }

        let mut violations = Vec::new();
        for file in &matching_files {
            let filename = file.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            if name_re.is_match(&filename) {
                let msg = self.def.rule_type.custom_message()
                    .unwrap_or_else(|| format!("Filename '{}' matches forbidden pattern '{}'", filename, pattern));
                violations.push(self.make_violation(Some(file), &msg));
            }
        }

        if violations.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail { violations }
        }
    }
}

impl RuleType {
    fn custom_message(&self) -> Option<String> {
        match self {
            RuleType::DirNotExists { message, .. } => Some(message.clone()),
            _ => None,
        }
    }
}

/// Convert a glob pattern to a regex. Handles *, **, and ?.
/// Returns None if the resulting regex is invalid.
pub fn glob_to_regex(glob: &str) -> Option<Regex> {
    let mut regex = String::with_capacity(glob.len() * 2);
    regex.push('^');

    let chars: Vec<char> = glob.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    // ** matches any depth (including path separators)
                    if i + 2 < chars.len() && chars[i + 2] == '/' {
                        regex.push_str("(?:.*/)?");
                        i += 3;
                    } else {
                        regex.push_str(".*");
                        i += 2;
                    }
                } else {
                    // * matches any non-separator characters
                    regex.push_str("[^/]*");
                    i += 1;
                }
            }
            '?' => {
                regex.push_str("[^/]");
                i += 1;
            }
            '.' | '+' | '(' | ')' | '{' | '}' | '[' | ']' | '^' | '$' | '|' | '\\' => {
                regex.push('\\');
                regex.push(chars[i]);
                i += 1;
            }
            c => {
                regex.push(c);
                i += 1;
            }
        }
    }

    regex.push('$');
    Regex::new(&regex).ok()
}
