use std::sync::Arc;

use agent_controller::AgentDescriptor;

use crate::api::types::ChatError;
use crate::core::agents::ChatAgentManager;
use crate::spi::ComplianceChatConfig;

/// Interactive compliance chat agent.
///
/// Sends messages to the active compliance agent and returns responses.
pub struct ComplianceChat {
    llm: Arc<dyn llm_provider::LlmService>,
    config: ComplianceChatConfig,
    manager: ChatAgentManager,
}

impl ComplianceChat {
    /// Create the compliance chat service.
    ///
    /// This is async because the underlying LLM provider may perform network
    /// handshakes during initialisation.
    pub async fn new(config: ComplianceChatConfig) -> Result<Self, ChatError> {
        if !config.enabled {
            return Err(ChatError::NotEnabled(
                "set DOC_ENGINE_AI_ENABLED=true".into(),
            ));
        }
        if !config.has_api_key() {
            return Err(ChatError::NotEnabled(
                "no API key found (set ANTHROPIC_API_KEY or OPENAI_API_KEY)".into(),
            ));
        }

        let llm = Arc::new(
            llm_provider::create_service()
                .await
                .map_err(|e| ChatError::Init(e.to_string()))?,
        );

        let manager = ChatAgentManager::new(llm.clone(), config.clone());
        Ok(Self {
            llm,
            config,
            manager,
        })
    }

    /// Send a chat message to the active compliance agent.
    pub async fn chat(&self, message: &str) -> Result<String, ChatError> {
        let agent = self
            .manager
            .active_agent()
            .ok_or(ChatError::NoAgent)?;

        let response = llm_provider::CompletionBuilder::new(&self.config.model)
            .system(agent.system_prompt())
            .user(message)
            .execute(&*self.llm)
            .await
            .map_err(|e| ChatError::Llm(e.to_string()))?;

        Ok(response.content.unwrap_or_default())
    }
}
