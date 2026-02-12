use std::sync::Arc;

use agent_controller::{AgentDescriptor, EngineFactory};
use chat_engine::{ChatConfig, ChatEngine, ToolAwareChatEngine};
use tool::ToolRegistry;

use crate::spi::AuditConfig;
use super::manager::AuditAgent;
use crate::core::tools::ComplianceScanTool;

/// Factory that creates `ToolAwareChatEngine` instances for audit agents.
pub struct AuditEngineFactory {
    llm: Arc<dyn llm_provider::LlmService>,
    config: AuditConfig,
}

impl AuditEngineFactory {
    pub fn new(llm: Arc<dyn llm_provider::LlmService>, config: AuditConfig) -> Self {
        Self { llm, config }
    }

    fn build_tool_registry(&self, agent: &AuditAgent) -> ToolRegistry {
        let mut registry = ToolRegistry::new();
        if agent.tools.contains(&"compliance_scan".to_string()) {
            registry.register(Box::new(ComplianceScanTool::new()));
        }
        registry
    }
}

impl EngineFactory<AuditAgent> for AuditEngineFactory {
    type Engine = dyn ChatEngine;

    fn create(&self, descriptor: &AuditAgent) -> Option<Arc<Self::Engine>> {
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
