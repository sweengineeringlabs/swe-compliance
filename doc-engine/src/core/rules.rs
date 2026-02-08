use serde::Deserialize;

use crate::api::types::{RuleDef, RuleSet, RuleType};
use crate::spi::traits::CheckRunner;
use crate::spi::types::{ProjectType, ScanError, Severity};
use super::builtins;
use super::declarative::DeclarativeCheck;

pub const DEFAULT_RULES: &str = include_str!("../../rules.toml");

/// Intermediate struct for flat TOML deserialization.
/// The `type` field determines which sibling fields are relevant.
#[derive(Debug, Deserialize)]
struct RawRule {
    id: u8,
    category: String,
    description: String,
    severity: String,
    #[serde(rename = "type")]
    rule_type: String,
    path: Option<String>,
    pattern: Option<String>,
    glob: Option<String>,
    handler: Option<String>,
    exclude_paths: Option<Vec<String>>,
    exclude_pattern: Option<String>,
    message: Option<String>,
    project_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawRuleFile {
    rules: Vec<RawRule>,
}

fn parse_severity(s: &str) -> Result<Severity, ScanError> {
    match s {
        "error" => Ok(Severity::Error),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        other => Err(ScanError::Config(format!("Unknown severity: {}", other))),
    }
}

fn parse_project_type(s: &str) -> Result<ProjectType, ScanError> {
    match s {
        "open_source" => Ok(ProjectType::OpenSource),
        "internal" => Ok(ProjectType::Internal),
        other => Err(ScanError::Config(format!("Unknown project_type: {}", other))),
    }
}

fn convert_raw_rule(raw: RawRule) -> Result<RuleDef, ScanError> {
    let severity = parse_severity(&raw.severity)?;
    let project_type = raw.project_type.as_deref().map(parse_project_type).transpose()?;

    let rule_type = match raw.rule_type.as_str() {
        "file_exists" => {
            let path = raw.path.ok_or_else(|| ScanError::Config(
                format!("Rule {}: file_exists requires 'path'", raw.id)
            ))?;
            RuleType::FileExists { path }
        }
        "dir_exists" => {
            let path = raw.path.ok_or_else(|| ScanError::Config(
                format!("Rule {}: dir_exists requires 'path'", raw.id)
            ))?;
            RuleType::DirExists { path }
        }
        "dir_not_exists" => {
            let path = raw.path.ok_or_else(|| ScanError::Config(
                format!("Rule {}: dir_not_exists requires 'path'", raw.id)
            ))?;
            let message = raw.message.unwrap_or_else(|| format!("{} should not exist", path));
            RuleType::DirNotExists { path, message }
        }
        "file_content_matches" => {
            let path = raw.path.ok_or_else(|| ScanError::Config(
                format!("Rule {}: file_content_matches requires 'path'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: file_content_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::FileContentMatches { path, pattern }
        }
        "file_content_not_matches" => {
            let path = raw.path.ok_or_else(|| ScanError::Config(
                format!("Rule {}: file_content_not_matches requires 'path'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: file_content_not_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::FileContentNotMatches { path, pattern }
        }
        "glob_content_matches" => {
            let glob = raw.glob.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_content_matches requires 'glob'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_content_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::GlobContentMatches { glob, pattern }
        }
        "glob_content_not_matches" => {
            let glob = raw.glob.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_content_not_matches requires 'glob'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_content_not_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::GlobContentNotMatches { glob, pattern, exclude_pattern: raw.exclude_pattern }
        }
        "glob_naming_matches" => {
            let glob = raw.glob.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_naming_matches requires 'glob'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_naming_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::GlobNamingMatches { glob, pattern }
        }
        "glob_naming_not_matches" => {
            let glob = raw.glob.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_naming_not_matches requires 'glob'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: glob_naming_not_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::GlobNamingNotMatches { glob, pattern, exclude_paths: raw.exclude_paths }
        }
        "builtin" => {
            let handler = raw.handler.ok_or_else(|| ScanError::Config(
                format!("Rule {}: builtin requires 'handler'", raw.id)
            ))?;
            RuleType::Builtin { handler }
        }
        other => {
            return Err(ScanError::Config(format!("Rule {}: unknown type '{}'", raw.id, other)));
        }
    };

    Ok(RuleDef {
        id: raw.id,
        category: raw.category,
        description: raw.description,
        severity,
        rule_type,
        project_type,
    })
}

pub fn parse_rules(toml_str: &str) -> Result<RuleSet, ScanError> {
    let raw: RawRuleFile = toml::from_str(toml_str)
        .map_err(|e| ScanError::Config(format!("TOML parse error: {}", e)))?;

    let mut rules = Vec::with_capacity(raw.rules.len());
    for raw_rule in raw.rules {
        rules.push(convert_raw_rule(raw_rule)?);
    }

    Ok(RuleSet { rules })
}

pub fn build_registry(rules: &[RuleDef]) -> Result<Vec<Box<dyn CheckRunner>>, ScanError> {
    let mut runners: Vec<Box<dyn CheckRunner>> = Vec::with_capacity(rules.len());

    for def in rules {
        let runner: Box<dyn CheckRunner> = match &def.rule_type {
            RuleType::Builtin { handler } => {
                builtins::get_handler(handler, def).ok_or_else(|| {
                    ScanError::Config(format!("Rule {}: unknown builtin handler '{}'", def.id, handler))
                })?
            }
            _ => Box::new(DeclarativeCheck { def: def.clone() }),
        };
        runners.push(runner);
    }

    runners.sort_by_key(|r| r.id().0);
    Ok(runners)
}
