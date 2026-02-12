use std::sync::Arc;

use agent_controller::{AgentDescriptor, EngineFactory};
use chat_engine::{ChatConfig, ChatEngine, ToolAwareChatEngine};
use tool::ToolRegistry;

use crate::spi::ComplianceChatConfig;
use super::manager::ChatAgent;

/// Factory that creates `ToolAwareChatEngine` instances for chat agents.
///
/// Unlike the audit factory, this registers no tools.
pub struct ChatEngineFactory {
    llm: Arc<dyn llm_provider::LlmService>,
    config: ComplianceChatConfig,
}

impl ChatEngineFactory {
    pub fn new(llm: Arc<dyn llm_provider::LlmService>, config: ComplianceChatConfig) -> Self {
        Self { llm, config }
    }
}

impl EngineFactory<ChatAgent> for ChatEngineFactory {
    type Engine = dyn ChatEngine;

    fn create(&self, descriptor: &ChatAgent) -> Option<Arc<Self::Engine>> {
        let registry = ToolRegistry::new();
        let chat_config = ChatConfig {
            model: self.config.model.clone(),
            temperature: descriptor.temperature().unwrap_or(0.1),
            max_tokens: descriptor.max_tokens().unwrap_or(4096),
            system_prompt: Some(descriptor.system_prompt().to_string()),
            max_history: self.config.history_size,
            enable_summarization: false,
        };

        Some(Arc::new(ToolAwareChatEngine::new(
            self.llm.clone(),
            chat_config,
            Arc::new(registry),
        )))
    }
}
