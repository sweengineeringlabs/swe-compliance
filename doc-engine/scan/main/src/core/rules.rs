use serde::Deserialize;

use crate::api::traits::CheckRunner;
use crate::api::types::{ProjectScope, ProjectType, RuleDef, RuleSet, RuleType, ScanError, Severity};
use super::builtins;
use super::declarative::DeclarativeCheck;

pub const DEFAULT_RULES: &str = include_str!("../../../config/rules.toml");

/// Return the number of rules defined in the embedded `rules.toml`.
pub fn default_rule_count() -> usize {
    parse_rules(DEFAULT_RULES).expect("embedded rules.toml is invalid").rules.len()
}

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
    scope: Option<String>,
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

fn parse_scope(s: &str) -> Result<ProjectScope, ScanError> {
    match s {
        "small" => Ok(ProjectScope::Small),
        "medium" => Ok(ProjectScope::Medium),
        "large" => Ok(ProjectScope::Large),
        other => Err(ScanError::Config(format!("Unknown scope: {}", other))),
    }
}

fn convert_raw_rule(raw: RawRule) -> Result<RuleDef, ScanError> {
    let severity = parse_severity(&raw.severity)?;
    let project_type = raw.project_type.as_deref().map(parse_project_type).transpose()?;
    let scope = raw.scope.as_deref().map(parse_scope).transpose()?;

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
        scope,
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
description = "Root docs/ folder exists"
severity = "error"
type = "file_exists"
path = "docs/README.md"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules.len(), 1);
        assert_eq!(rs.rules[0].id, 1);
        assert!(matches!(rs.rules[0].rule_type, RuleType::FileExists { .. }));
    }

    #[test]
    fn test_parse_all_declarative_types() {
        let toml = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "file_exists"
path = "x"

[[rules]]
id = 2
category = "a"
description = "d"
severity = "warning"
type = "dir_exists"
path = "x"

[[rules]]
id = 3
category = "a"
description = "d"
severity = "info"
type = "dir_not_exists"
path = "x"
message = "no"

[[rules]]
id = 4
category = "a"
description = "d"
severity = "error"
type = "file_content_matches"
path = "x"
pattern = "y"

[[rules]]
id = 5
category = "a"
description = "d"
severity = "error"
type = "file_content_not_matches"
path = "x"
pattern = "y"

[[rules]]
id = 6
category = "a"
description = "d"
severity = "error"
type = "glob_content_matches"
glob = "**/*.md"
pattern = "y"

[[rules]]
id = 7
category = "a"
description = "d"
severity = "error"
type = "glob_content_not_matches"
glob = "**/*.md"
pattern = "y"

[[rules]]
id = 8
category = "a"
description = "d"
severity = "error"
type = "glob_naming_matches"
glob = "**/*.md"
pattern = "y"

[[rules]]
id = 9
category = "a"
description = "d"
severity = "error"
type = "glob_naming_not_matches"
glob = "**/*.md"
pattern = "y"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules.len(), 9);
    }

    #[test]
    fn test_parse_builtin_rule() {
        let toml = r#"
[[rules]]
id = 4
category = "structure"
description = "Module docs plural"
severity = "error"
type = "builtin"
handler = "module_docs_plural"
"#;
        let rs = parse_rules(toml).unwrap();
        assert!(matches!(rs.rules[0].rule_type, RuleType::Builtin { ref handler } if handler == "module_docs_plural"));
    }

    #[test]
    fn test_parse_with_project_type() {
        let toml = r#"
[[rules]]
id = 31
category = "root_files"
description = "Community files"
severity = "warning"
type = "builtin"
handler = "open_source_community_files"
project_type = "open_source"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules[0].project_type, Some(ProjectType::OpenSource));

        let toml2 = r#"
[[rules]]
id = 31
category = "root_files"
description = "Internal only"
severity = "warning"
type = "builtin"
handler = "open_source_community_files"
project_type = "internal"
"#;
        let rs2 = parse_rules(toml2).unwrap();
        assert_eq!(rs2.rules[0].project_type, Some(ProjectType::Internal));
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
        // file_exists without path
        let toml = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "file_exists"
"#;
        assert!(matches!(parse_rules(toml).unwrap_err(), ScanError::Config(_)));

        // glob_content_matches without pattern
        let toml2 = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "glob_content_matches"
glob = "**/*.md"
"#;
        assert!(matches!(parse_rules(toml2).unwrap_err(), ScanError::Config(_)));

        // builtin without handler
        let toml3 = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "error"
type = "builtin"
"#;
        assert!(matches!(parse_rules(toml3).unwrap_err(), ScanError::Config(_)));
    }

    #[test]
    fn test_parse_unknown_severity() {
        let toml = r#"
[[rules]]
id = 1
category = "a"
description = "d"
severity = "critical"
type = "file_exists"
path = "x"
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
            rule_type: RuleType::FileExists { path: "x".to_string() },
            project_type: None,
            scope: None,
        }];
        let reg = build_registry(&rules).unwrap();
        assert_eq!(reg.len(), 1);
        assert_eq!(reg[0].id().0, 1);
    }

    #[test]
    fn test_build_registry_builtin() {
        let rules = vec![RuleDef {
            id: 4,
            category: "structure".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::Builtin { handler: "module_docs_plural".to_string() },
            project_type: None,
            scope: None,
        }];
        let reg = build_registry(&rules).unwrap();
        assert_eq!(reg.len(), 1);
        assert_eq!(reg[0].id().0, 4);
    }

    #[test]
    fn test_build_registry_unknown_handler() {
        let rules = vec![RuleDef {
            id: 99,
            category: "test".to_string(),
            description: "test".to_string(),
            severity: Severity::Error,
            rule_type: RuleType::Builtin { handler: "nonexistent".to_string() },
            project_type: None,
            scope: None,
        }];
        let result = build_registry(&rules);
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ScanError::Config(_)));
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
                project_type: None,
                scope: None,
            },
            RuleDef {
                id: 1,
                category: "a".to_string(),
                description: "d".to_string(),
                severity: Severity::Error,
                rule_type: RuleType::DirExists { path: "y".to_string() },
                project_type: None,
                scope: None,
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

    #[test]
    fn test_parse_scope_values() {
        assert_eq!(parse_scope("small").unwrap(), ProjectScope::Small);
        assert_eq!(parse_scope("medium").unwrap(), ProjectScope::Medium);
        assert_eq!(parse_scope("large").unwrap(), ProjectScope::Large);
    }

    #[test]
    fn test_parse_scope_invalid() {
        assert!(parse_scope("tiny").is_err());
        assert!(parse_scope("huge").is_err());
    }

    #[test]
    fn test_parse_with_scope() {
        let toml = r#"
[[rules]]
id = 1
category = "structure"
description = "test"
severity = "error"
type = "file_exists"
path = "docs/README.md"
scope = "small"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules[0].scope, Some(ProjectScope::Small));
    }

    #[test]
    fn test_parse_without_scope() {
        let toml = r#"
[[rules]]
id = 1
category = "structure"
description = "test"
severity = "error"
type = "file_exists"
path = "docs/README.md"
"#;
        let rs = parse_rules(toml).unwrap();
        assert_eq!(rs.rules[0].scope, None);
    }

    #[test]
    fn test_parse_invalid_scope() {
        let toml = r#"
[[rules]]
id = 1
category = "structure"
description = "test"
severity = "error"
type = "file_exists"
path = "docs/README.md"
scope = "tiny"
"#;
        assert!(parse_rules(toml).is_err());
    }

    #[test]
    fn test_default_rules_have_scope() {
        let rs = parse_rules(DEFAULT_RULES).unwrap();
        // All rules in the default rules.toml should have a scope
        for rule in &rs.rules {
            assert!(
                rule.scope.is_some(),
                "Rule {} should have a scope annotation", rule.id
            );
        }
    }
}
