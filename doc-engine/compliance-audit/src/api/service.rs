use std::sync::Arc;

use agent_controller::AgentDescriptor;
use tool::Tool;

use crate::api::types::{AuditError, AuditResponse};
use crate::core::agents::AuditAgentManager;
use crate::core::tools::ComplianceScanTool;
use crate::spi::AuditConfig;

/// AI-powered compliance auditor.
///
/// Runs a compliance scan and then asks the LLM to analyse the results.
pub struct ComplianceAuditor {
    llm: Arc<dyn llm_provider::LlmService>,
    config: AuditConfig,
    manager: AuditAgentManager,
}

impl ComplianceAuditor {
    /// Create the compliance auditor.
    ///
    /// This is async because the underlying LLM provider may perform network
    /// handshakes during initialisation.
    pub async fn new(config: AuditConfig) -> Result<Self, AuditError> {
        if !config.enabled {
            return Err(AuditError::NotEnabled(
                "set DOC_ENGINE_AI_ENABLED=true".into(),
            ));
        }
        if !config.has_api_key() {
            return Err(AuditError::NotEnabled(
                "no API key found (set ANTHROPIC_API_KEY or OPENAI_API_KEY)".into(),
            ));
        }

        let llm = Arc::new(
            llm_provider::create_service()
                .await
                .map_err(|e| AuditError::Init(e.to_string()))?,
        );

        let manager = AuditAgentManager::new(llm.clone(), config.clone());
        Ok(Self {
            llm,
            config,
            manager,
        })
    }

    /// Run an AI-powered compliance audit on the given path.
    pub async fn audit(&self, path: &str, scope: &str) -> Result<AuditResponse, AuditError> {
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
            .map_err(|e| AuditError::Tool(e.to_string()))?;

        // 2. Ask the LLM to analyse the results.
        let agent = self
            .manager
            .active_agent()
            .ok_or(AuditError::NoAgent)?;

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
            .map_err(|e| AuditError::Llm(e.to_string()))?;

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
