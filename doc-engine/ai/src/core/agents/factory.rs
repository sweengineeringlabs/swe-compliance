use std::sync::Arc;

use agent_controller::{AgentDescriptor, EngineFactory};
use chat_engine::{ChatConfig, ChatEngine, ToolAwareChatEngine};
use tool::ToolRegistry;

use crate::spi::DocEngineAiConfig;
use super::manager::DocEngineAgent;
use crate::core::tools::ComplianceScanTool;

/// Factory that creates `ToolAwareChatEngine` instances for doc-engine agents.
///
/// Implements rustratify's `EngineFactory` trait so it can be used with
/// `EngineCache` for lazy, cached engine creation.
pub struct DocEngineFactory {
    llm: Arc<dyn llm_provider::LlmService>,
    config: DocEngineAiConfig,
}

impl DocEngineFactory {
    pub fn new(llm: Arc<dyn llm_provider::LlmService>, config: DocEngineAiConfig) -> Self {
        Self { llm, config }
    }

    fn build_tool_registry(&self, agent: &DocEngineAgent) -> ToolRegistry {
        let mut registry = ToolRegistry::new();
        if agent.tools.contains(&"compliance_scan".to_string()) {
            registry.register(Box::new(ComplianceScanTool::new()));
        }
        registry
    }
}

impl EngineFactory<DocEngineAgent> for DocEngineFactory {
    type Engine = dyn ChatEngine;

    fn create(&self, descriptor: &DocEngineAgent) -> Option<Arc<Self::Engine>> {
        let registry = self.build_tool_registry(descriptor);
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
