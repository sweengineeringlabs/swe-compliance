use std::fmt;

/// Errors produced by the compliance chat subsystem.
#[derive(Debug)]
pub enum ChatError {
    /// AI is not enabled or misconfigured.
    NotEnabled(String),
    /// LLM provider initialisation failed.
    Init(String),
    /// No active agent available.
    NoAgent,
    /// LLM completion failed.
    Llm(String),
}

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEnabled(msg) => write!(f, "AI not enabled: {}", msg),
            Self::Init(msg) => write!(f, "AI init failed: {}", msg),
            Self::NoAgent => write!(f, "no active agent configured"),
            Self::Llm(msg) => write!(f, "LLM error: {}", msg),
        }
    }
}

impl std::error::Error for ChatError {}
