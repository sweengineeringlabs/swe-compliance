use std::collections::HashMap;
use std::sync::Arc;

use crate::api::types::{
    CommandGeneratorConfig, CommandGeneratorError,
    GenerateCommandsRequest, GenerateCommandsResponse, SkippedRequirement,
};

/// LLM-powered test command generator.
///
/// Generates CLI commands to verify SRS requirements by sending batches
/// of requirement contexts to an LLM.
pub struct CommandGenerator {
    llm: Arc<dyn llm_provider::LlmService>,
    config: CommandGeneratorConfig,
}

impl CommandGenerator {
    /// Create the command generator.
    ///
    /// This is async because the underlying LLM provider may perform network
    /// handshakes during initialisation.
    pub async fn new(config: CommandGeneratorConfig) -> Result<Self, CommandGeneratorError> {
        if !config.enabled {
            return Err(CommandGeneratorError::NotEnabled(
                "set DOC_ENGINE_AI_ENABLED=true".into(),
            ));
        }
        if !config.has_api_key() {
            return Err(CommandGeneratorError::NotEnabled(
                "no API key found (set ANTHROPIC_API_KEY or OPENAI_API_KEY)".into(),
            ));
        }

        let llm = Arc::new(
            llm_provider::create_service()
                .await
                .map_err(|e| CommandGeneratorError::Init(e.to_string()))?,
        );

        Ok(Self { llm, config })
    }

    /// Generate test commands for requirements that are missing them.
    pub async fn generate_commands(
        &self,
        request: &GenerateCommandsRequest,
    ) -> Result<GenerateCommandsResponse, CommandGeneratorError> {
        if request.requirements.is_empty() {
            return Ok(GenerateCommandsResponse {
                commands: HashMap::new(),
                skipped: Vec::new(),
            });
        }

        let prompt = build_generate_commands_prompt(request);

        let system_prompt = "\
You are a test automation engineer for a Rust workspace. \
You generate precise CLI commands to verify software requirements. \
You respond ONLY with a JSON object — no commentary, no markdown fences.";

        let response = llm_provider::CompletionBuilder::new(&self.config.model)
            .system(system_prompt)
            .user(&prompt)
            .execute(&*self.llm)
            .await
            .map_err(|e| CommandGeneratorError::Llm(e.to_string()))?;

        let raw = response.content.unwrap_or_default();
        parse_generate_commands_response(&raw)
    }
}

/// Build the batch prompt that lists all requirements for the LLM.
pub fn build_generate_commands_prompt(request: &GenerateCommandsRequest) -> String {
    let mut prompt = String::with_capacity(4096);

    prompt.push_str("# Project Context\n\n");
    prompt.push_str(&request.project_context);
    prompt.push_str("\n\n");

    prompt.push_str("# Requirements\n\n");
    prompt.push_str("For each requirement below, generate a CLI command to verify it.\n\n");
    prompt.push_str("Rules by verification method:\n");
    prompt.push_str("- **Test** → `cargo test -p <crate> <filter>` (infer crate from traces_to path)\n");
    prompt.push_str("- **Demonstration** → `cargo run` / `cargo build` command\n");
    prompt.push_str("- **Inspection** → output `INSPECTION: <what to inspect>`\n");
    prompt.push_str("- **Analysis** → output `ANALYSIS: <description>`\n");
    prompt.push_str("- Unknown/unclear → output `SKIP: <reason>`\n\n");

    prompt.push_str("Response format: a single JSON object with a \"commands\" key:\n");
    prompt.push_str("```\n{\"commands\": {\"FR-xxx\": \"cargo test ...\", \"FR-yyy\": \"INSPECTION: ...\"}}\n```\n\n");

    for req in &request.requirements {
        prompt.push_str(&format!("## {}: {}\n", req.id, req.title));
        prompt.push_str(&format!("- Verification: {}\n", req.verification));
        prompt.push_str(&format!("- Acceptance: {}\n", req.acceptance));
        prompt.push_str(&format!("- Traces to: {}\n", req.traces_to));
        if !req.description.is_empty() {
            prompt.push_str(&format!("- Description: {}\n", req.description));
        }
        prompt.push('\n');
    }

    prompt
}

/// Parse the LLM JSON response into commands and skipped entries.
pub fn parse_generate_commands_response(
    raw: &str,
) -> Result<GenerateCommandsResponse, CommandGeneratorError> {
    let json_str = extract_json_block(raw);

    let parsed: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
        CommandGeneratorError::Serialization(format!(
            "failed to parse LLM response as JSON: {}: input was: {}",
            e,
            &raw[..raw.len().min(200)]
        ))
    })?;

    let commands_obj = parsed
        .get("commands")
        .and_then(|v| v.as_object())
        .ok_or_else(|| {
            CommandGeneratorError::Serialization(
                "LLM response missing \"commands\" object".to_string(),
            )
        })?;

    let mut commands = HashMap::new();
    let mut skipped = Vec::new();

    for (id, val) in commands_obj {
        let cmd = val.as_str().unwrap_or_default().to_string();
        if cmd.starts_with("INSPECTION:") || cmd.starts_with("ANALYSIS:") || cmd.starts_with("SKIP:") {
            skipped.push(SkippedRequirement {
                id: id.clone(),
                reason: cmd,
            });
        } else {
            commands.insert(id.clone(), cmd);
        }
    }

    Ok(GenerateCommandsResponse { commands, skipped })
}

/// Extract a JSON block from LLM output, stripping optional code fences.
pub fn extract_json_block(raw: &str) -> &str {
    let trimmed = raw.trim();

    // Strip ```json ... ``` or ``` ... ```
    if let Some(rest) = trimmed.strip_prefix("```json") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
    }
    if let Some(rest) = trimmed.strip_prefix("```") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
    }

    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::RequirementContext;

    fn sample_requirements() -> Vec<RequirementContext> {
        vec![
            RequirementContext {
                id: "FR-100".to_string(),
                title: "Default rules embedded".to_string(),
                verification: "Test".to_string(),
                acceptance: "Engine loads embedded rules".to_string(),
                traces_to: "scan/src/rules.rs".to_string(),
                description: "The binary shall embed rules.".to_string(),
            },
            RequirementContext {
                id: "FR-200".to_string(),
                title: "Recursive scanning".to_string(),
                verification: "Demonstration".to_string(),
                acceptance: "Scanner finds nested files".to_string(),
                traces_to: "scan/src/discovery.rs".to_string(),
                description: String::new(),
            },
            RequirementContext {
                id: "NFR-100".to_string(),
                title: "Architecture compliance".to_string(),
                verification: "Inspection".to_string(),
                acceptance: "Crate structure follows SEA".to_string(),
                traces_to: "Cargo.toml".to_string(),
                description: String::new(),
            },
        ]
    }

    #[test]
    fn test_build_prompt_includes_all_requirements() {
        let request = GenerateCommandsRequest {
            requirements: sample_requirements(),
            project_context: "Rust workspace with crates: scan, scaffold, ai, cli".to_string(),
        };
        let prompt = build_generate_commands_prompt(&request);

        assert!(prompt.contains("FR-100"));
        assert!(prompt.contains("FR-200"));
        assert!(prompt.contains("NFR-100"));
        assert!(prompt.contains("scan/src/rules.rs"));
        assert!(prompt.contains("scan/src/discovery.rs"));
        assert!(prompt.contains("Rust workspace with crates: scan, scaffold, ai, cli"));
        assert!(prompt.contains("cargo test"));
    }

    #[test]
    fn test_parse_valid_json_response() {
        let raw = r#"{"commands": {"FR-100": "cargo test -p doc-engine-scan rules", "FR-200": "cargo run -- scan .", "NFR-100": "INSPECTION: review crate layout"}}"#;
        let resp = parse_generate_commands_response(raw).unwrap();

        assert_eq!(resp.commands.len(), 2);
        assert_eq!(resp.commands["FR-100"], "cargo test -p doc-engine-scan rules");
        assert_eq!(resp.commands["FR-200"], "cargo run -- scan .");
        assert_eq!(resp.skipped.len(), 1);
        assert_eq!(resp.skipped[0].id, "NFR-100");
        assert!(resp.skipped[0].reason.starts_with("INSPECTION:"));
    }

    #[test]
    fn test_parse_json_with_code_fence() {
        let raw = "```json\n{\"commands\": {\"FR-100\": \"cargo test -p scan rules\"}}\n```";
        let resp = parse_generate_commands_response(raw).unwrap();
        assert_eq!(resp.commands.len(), 1);
        assert_eq!(resp.commands["FR-100"], "cargo test -p scan rules");
    }

    #[test]
    fn test_parse_malformed_json_returns_error() {
        let raw = "This is not JSON at all.";
        let result = parse_generate_commands_response(raw);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("serialization error"));
    }

    #[test]
    fn test_extract_json_block_plain() {
        assert_eq!(extract_json_block(r#"{"a": 1}"#), r#"{"a": 1}"#);
    }

    #[test]
    fn test_extract_json_block_fenced() {
        let input = "```json\n{\"a\": 1}\n```";
        assert_eq!(extract_json_block(input), "{\"a\": 1}");
    }

    #[test]
    fn test_extract_json_block_bare_fence() {
        let input = "```\n{\"a\": 1}\n```";
        assert_eq!(extract_json_block(input), "{\"a\": 1}");
    }

    #[test]
    fn test_parse_response_separates_skip_entries() {
        let raw = r#"{"commands": {"FR-300": "SKIP: cannot determine test", "FR-301": "ANALYSIS: review docs"}}"#;
        let resp = parse_generate_commands_response(raw).unwrap();
        assert!(resp.commands.is_empty());
        assert_eq!(resp.skipped.len(), 2);
    }
}
