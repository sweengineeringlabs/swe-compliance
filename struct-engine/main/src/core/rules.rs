use serde::Deserialize;

use crate::api::traits::CheckRunner;
use crate::api::types::{RuleDef, RuleSet, RuleType, ProjectKind, ScanError, Severity};
use super::builtins;
use super::declarative::DeclarativeCheck;

pub const DEFAULT_RULES: &str = include_str!("../../../config/rules.toml");

/// Return the number of rules defined in the embedded `rules.toml`.
pub fn default_rule_count() -> usize {
    parse_rules(DEFAULT_RULES).expect("embedded rules.toml is invalid").rules.len()
}

/// Intermediate struct for flat TOML deserialization.
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
    project_kind: Option<String>,
    key: Option<String>,
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

fn parse_project_kind(s: &str) -> Result<ProjectKind, ScanError> {
    match s {
        "library" => Ok(ProjectKind::Library),
        "binary" => Ok(ProjectKind::Binary),
        "both" => Ok(ProjectKind::Both),
        "workspace" => Ok(ProjectKind::Workspace),
        other => Err(ScanError::Config(format!("Unknown project_kind: {}", other))),
    }
}

fn convert_raw_rule(raw: RawRule) -> Result<RuleDef, ScanError> {
    let severity = parse_severity(&raw.severity)?;
    let project_kind = raw.project_kind.as_deref().map(parse_project_kind).transpose()?;

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
        "cargo_key_exists" => {
            let key = raw.key.ok_or_else(|| ScanError::Config(
                format!("Rule {}: cargo_key_exists requires 'key'", raw.id)
            ))?;
            RuleType::CargoKeyExists { key }
        }
        "cargo_key_matches" => {
            let key = raw.key.ok_or_else(|| ScanError::Config(
                format!("Rule {}: cargo_key_matches requires 'key'", raw.id)
            ))?;
            let pattern = raw.pattern.ok_or_else(|| ScanError::Config(
                format!("Rule {}: cargo_key_matches requires 'pattern'", raw.id)
            ))?;
            RuleType::CargoKeyMatches { key, pattern }
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
        project_kind,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_exists() {
        let toml = r#"
[[rules]]
id = 1
category = "structure"
description = "Cargo.toml exists at root"
severity = "error"
type = "file_exists"
path = "Cargo.toml"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules.len(), 1);
        assert_eq!(rs.rules[0].id, 1);
        assert!(matches!(rs.rules[0].rule_type, RuleType::FileExists { .. }));
    }

    #[test]
    fn test_parse_cargo_key_exists() {
        let toml = r#"
[[rules]]
id = 9
category = "cargo_metadata"
description = "package.name exists"
severity = "error"
type = "cargo_key_exists"
key = "package.name"
"#;
        let rs = parse_rules(toml).unwrap();
        assert!(matches!(rs.rules[0].rule_type, RuleType::CargoKeyExists { ref key } if key == "package.name"));
    }

    #[test]
    fn test_parse_cargo_key_matches() {
        let toml = r#"
[[rules]]
id = 27
category = "naming"
description = "Package name uses snake_case"
severity = "warning"
type = "cargo_key_matches"
key = "package.name"
pattern = "^[a-z][a-z0-9_]*$"
"#;
        let rs = parse_rules(toml).unwrap();
        assert!(matches!(rs.rules[0].rule_type, RuleType::CargoKeyMatches { .. }));
    }

    #[test]
    fn test_parse_with_project_kind() {
        let toml = r#"
[[rules]]
id = 17
category = "cargo_metadata"
description = "keywords exists (libraries)"
severity = "info"
type = "cargo_key_exists"
key = "package.keywords"
project_kind = "library"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules[0].project_kind, Some(ProjectKind::Library));
    }

    #[test]
    fn test_parse_invalid_toml() {
        let result = parse_rules("not valid toml {{{{");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ScanError::Config(_)));
    }

    #[test]
    fn test_parse_unknown_type() {
        let toml = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "nonexistent_type"
"#;
        let result = parse_rules(toml);
        assert!(matches!(result.unwrap_err(), ScanError::Config(_)));
    }

    #[test]
    fn test_parse_missing_required_field() {
        let toml = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "cargo_key_exists"
"#;
        assert!(matches!(parse_rules(toml).unwrap_err(), ScanError::Config(_)));
    }

    #[test]
    fn test_parse_default_rules_valid() {
        let rs = parse_rules(DEFAULT_RULES).unwrap();
        assert_eq!(rs.rules.len(), default_rule_count());
    }

    #[test]
    fn test_build_registry_declarative() {
        let rules = vec![RuleDef {
            id: 1,
            category: "test".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::FileExists { path: "Cargo.toml".to_string() },
            project_kind: None,
        }];
        let reg = build_registry(&rules).unwrap();
        assert_eq!(reg.len(), 1);
        assert_eq!(reg[0].id().0, 1);
    }

    #[test]
    fn test_build_registry_builtin() {
        let rules = vec![RuleDef {
            id: 3,
            category: "structure".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::Builtin { handler: "crate_root_exists".to_string() },
            project_kind: None,
        }];
        let reg = build_registry(&rules).unwrap();
        assert_eq!(reg.len(), 1);
        assert_eq!(reg[0].id().0, 3);
    }

    #[test]
    fn test_build_registry_unknown_handler() {
        let rules = vec![RuleDef {
            id: 99,
            category: "test".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::Builtin { handler: "nonexistent".to_string() },
            project_kind: None,
        }];
        let result = build_registry(&rules);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_registry_sorted() {
        let rules = vec![
            RuleDef {
                id: 5,
                category: "a".to_string(),
                description: "d".to_string(),
                severity: Severity::Error,
                rule_type: RuleType::FileExists { path: "x".to_string() },
                project_kind: None,
            },
            RuleDef {
                id: 1,
                category: "a".to_string(),
                description: "d".to_string(),
                severity: Severity::Error,
                rule_type: RuleType::DirExists { path: "y".to_string() },
                project_kind: None,
            },
        ];
        let reg = build_registry(&rules).unwrap();
        assert_eq!(reg[0].id().0, 1);
        assert_eq!(reg[1].id().0, 5);
    }

    #[test]
    fn test_build_registry_default_rules() {
        let rs = parse_rules(DEFAULT_RULES).unwrap();
        let reg = build_registry(&rs.rules).unwrap();
        assert_eq!(reg.len(), default_rule_count());
    }
}
