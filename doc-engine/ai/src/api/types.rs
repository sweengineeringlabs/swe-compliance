use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Response from an AI-powered compliance audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResponse {
    /// LLM-generated summary of compliance status.
    pub summary: String,
    /// Raw scan results as JSON.
    pub scan_results: serde_json::Value,
    /// Extracted actionable recommendations.
    pub recommendations: Vec<String>,
}

/// Errors produced by the AI subsystem.
#[derive(Debug)]
pub enum DocEngineAiError {
    /// AI is not enabled or misconfigured.
    NotEnabled(String),
    /// LLM provider initialisation failed.
    Init(String),
    /// No active agent available.
    NoAgent,
    /// LLM completion failed.
    Llm(String),
    /// Tool execution failed.
    Tool(String),
    /// Compliance scan error.
    Scan(String),
    /// Serialization/deserialization error.
    Serialization(String),
    /// Invalid argument.
    InvalidArgument(String),
}

impl fmt::Display for DocEngineAiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEnabled(msg) => write!(f, "AI not enabled: {}", msg),
            Self::Init(msg) => write!(f, "AI init failed: {}", msg),
            Self::NoAgent => write!(f, "no active agent configured"),
            Self::Llm(msg) => write!(f, "LLM error: {}", msg),
            Self::Tool(msg) => write!(f, "tool error: {}", msg),
            Self::Scan(msg) => write!(f, "scan error: {}", msg),
            Self::Serialization(msg) => write!(f, "serialization error: {}", msg),
            Self::InvalidArgument(msg) => write!(f, "invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for DocEngineAiError {}

/// High-level AI service trait for doc-engine.
#[async_trait]
pub trait DocEngineAiService: Send + Sync {
    /// Send a chat message to the active compliance agent.
    async fn chat(&self, message: &str) -> Result<String, DocEngineAiError>;

    /// Run an AI-powered compliance audit on the given path.
    async fn audit(&self, path: &str, scope: &str) -> Result<AuditResponse, DocEngineAiError>;
}
