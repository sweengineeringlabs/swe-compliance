use std::sync::Arc;

use async_trait::async_trait;

use crate::api::types::{AuditResponse, DocEngineAiError, DocEngineAiService};
use crate::core::agents::DocEngineAgentManager;
use crate::core::tools::ComplianceScanTool;
use crate::spi::DocEngineAiConfig;

use agent_controller::AgentDescriptor;
use tool::Tool;

/// Default implementation of `DocEngineAiService`.
///
/// Holds an LLM handle and an agent manager.  For the MVP the `chat()` and
/// `audit()` methods use `CompletionBuilder` one-shot calls so that we avoid
/// needing to wire up event channels.  The full `ChatEngine` infrastructure
/// (factory, engine cache) is in place and ready for Phase 2 streaming.
pub struct DefaultDocEngineAiService {
    llm: Arc<dyn llm_provider::LlmService>,
    config: DocEngineAiConfig,
    manager: DocEngineAgentManager,
}

impl DefaultDocEngineAiService {
    /// Create the AI service.
    ///
    /// This is async because the underlying LLM provider may perform network
    /// handshakes during initialisation.
    pub async fn new(config: DocEngineAiConfig) -> Result<Self, DocEngineAiError> {
        if !config.enabled {
            return Err(DocEngineAiError::NotEnabled(
                "set DOC_ENGINE_AI_ENABLED=true".into(),
            ));
        }
        if !config.has_api_key() {
            return Err(DocEngineAiError::NotEnabled(
                "no API key found (set ANTHROPIC_API_KEY or OPENAI_API_KEY)".into(),
            ));
        }

        let llm = Arc::new(
            llm_provider::create_service()
                .await
                .map_err(|e| DocEngineAiError::Init(e.to_string()))?,
        );

        let manager = DocEngineAgentManager::new(llm.clone(), config.clone());
        Ok(Self {
            llm,
            config,
            manager,
        })
    }
}

#[async_trait]
impl DocEngineAiService for DefaultDocEngineAiService {
    async fn chat(&self, message: &str) -> Result<String, DocEngineAiError> {
        let agent = self
            .manager
            .active_agent()
            .ok_or(DocEngineAiError::NoAgent)?;

        let response = llm_provider::CompletionBuilder::new(&self.config.model)
            .system(agent.system_prompt())
            .user(message)
            .execute(&*self.llm)
            .await
            .map_err(|e| DocEngineAiError::Llm(e.to_string()))?;

        Ok(response.content.unwrap_or_default())
    }

    async fn audit(&self, path: &str, scope: &str) -> Result<AuditResponse, DocEngineAiError> {
        // 1. Run the scan tool directly.
        let scan_tool = ComplianceScanTool::new();
        let args = serde_json::json!({
            "path": path,
            "scope": scope,
            "format": "json"
        });
        let output = scan_tool
            .execute(args)
            .await
            .map_err(|e| DocEngineAiError::Tool(e.to_string()))?;

        // 2. Ask the LLM to analyse the results.
        let agent = self
            .manager
            .active_agent()
            .ok_or(DocEngineAiError::NoAgent)?;

        let analysis_prompt = format!(
            "Analyse the following compliance scan results. \
             Summarise the compliance status, prioritise failures by severity, \
             and list actionable recommendations:\n\n{}",
            serde_json::to_string_pretty(&output.content).unwrap_or_default()
        );

        let response = llm_provider::CompletionBuilder::new(&self.config.model)
            .system(agent.system_prompt())
            .user(&analysis_prompt)
            .execute(&*self.llm)
            .await
            .map_err(|e| DocEngineAiError::Llm(e.to_string()))?;

        let summary = response.content.unwrap_or_default();

        // 3. Extract bullet-point recommendations from the LLM response.
        let recommendations: Vec<String> = summary
            .lines()
            .filter(|l| l.starts_with("- ") || l.starts_with("* "))
            .map(|l| {
                l.trim_start_matches("- ")
                    .trim_start_matches("* ")
                    .to_string()
            })
            .collect();

        Ok(AuditResponse {
            summary,
            scan_results: output.content,
            recommendations,
        })
    }
}
