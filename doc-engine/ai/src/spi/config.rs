/// Configuration for the doc-engine AI subsystem.
///
/// Reads LLM provider settings from environment variables,
/// following the same env-var conventions as swebash-ai.
#[derive(Debug, Clone)]
pub struct DocEngineAiConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub history_size: usize,
}

impl DocEngineAiConfig {
    /// Build configuration from environment variables.
    ///
    /// | Variable | Default |
    /// |----------|---------|
    /// | `DOC_ENGINE_AI_ENABLED` | `true` |
    /// | `LLM_PROVIDER` | `anthropic` |
    /// | `LLM_DEFAULT_MODEL` | `claude-sonnet-4-20250514` |
    /// | `DOC_ENGINE_AI_HISTORY_SIZE` | `20` |
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("DOC_ENGINE_AI_ENABLED")
                .map(|v| v != "0" && v.to_lowercase() != "false")
                .unwrap_or(true),
            provider: std::env::var("LLM_PROVIDER")
                .unwrap_or_else(|_| "anthropic".into()),
            model: std::env::var("LLM_DEFAULT_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".into()),
            history_size: std::env::var("DOC_ENGINE_AI_HISTORY_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(20),
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

impl Default for DocEngineAiConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
