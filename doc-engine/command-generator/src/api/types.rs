use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Configuration for the command generator.
///
/// Reads LLM provider settings from environment variables.
#[derive(Debug, Clone)]
pub struct CommandGeneratorConfig {
    pub enabled: bool,
    pub model: String,
}

impl CommandGeneratorConfig {
    /// Build configuration from environment variables.
    ///
    /// | Variable | Default |
    /// |----------|---------|
    /// | `DOC_ENGINE_AI_ENABLED` | `true` |
    /// | `LLM_DEFAULT_MODEL` | `claude-sonnet-4-20250514` |
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("DOC_ENGINE_AI_ENABLED")
                .map(|v| v != "0" && v.to_lowercase() != "false")
                .unwrap_or(true),
            model: std::env::var("LLM_DEFAULT_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".into()),
        }
    }

    /// Returns `true` when any recognised API key env-var is non-empty.
    pub fn has_api_key(&self) -> bool {
        std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .or_else(|_| std::env::var("GEMINI_API_KEY"))
            .map(|k| !k.is_empty())
            .unwrap_or(false)
    }
}

impl Default for CommandGeneratorConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Errors produced by the command generator.
#[derive(Debug)]
pub enum CommandGeneratorError {
    /// AI is not enabled or misconfigured.
    NotEnabled(String),
    /// LLM provider initialisation failed.
    Init(String),
    /// LLM completion failed.
    Llm(String),
    /// Serialization/deserialization error.
    Serialization(String),
    /// Invalid argument.
    InvalidArgument(String),
}

impl fmt::Display for CommandGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEnabled(msg) => write!(f, "AI not enabled: {}", msg),
            Self::Init(msg) => write!(f, "AI init failed: {}", msg),
            Self::Llm(msg) => write!(f, "LLM error: {}", msg),
            Self::Serialization(msg) => write!(f, "serialization error: {}", msg),
            Self::InvalidArgument(msg) => write!(f, "invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for CommandGeneratorError {}

/// Context for a single requirement sent to the LLM for command generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementContext {
    /// Requirement identifier, e.g. "FR-100".
    pub id: String,
    /// Requirement title.
    pub title: String,
    /// Verification method: Test, Demonstration, Inspection, Analysis.
    pub verification: String,
    /// Acceptance criteria text.
    pub acceptance: String,
    /// Traceability reference (code paths, stakeholder IDs).
    pub traces_to: String,
    /// Narrative description from the SRS.
    pub description: String,
}

/// Request to generate test commands for a batch of requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCommandsRequest {
    /// Requirements needing commands.
    pub requirements: Vec<RequirementContext>,
    /// Project-level context (workspace layout, crate names, test framework).
    pub project_context: String,
}

/// A requirement the LLM chose to skip (Inspection, Analysis, or unknown).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedRequirement {
    /// Requirement identifier.
    pub id: String,
    /// Reason for skipping.
    pub reason: String,
}

/// Response from the LLM command generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCommandsResponse {
    /// Map of requirement ID to CLI command.
    pub commands: HashMap<String, String>,
    /// Requirements that were skipped (Inspection/Analysis/unknown).
    pub skipped: Vec<SkippedRequirement>,
}
