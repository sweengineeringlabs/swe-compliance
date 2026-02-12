use std::sync::Arc;

use agent_controller::AgentDescriptor;
use serde::Deserialize;

use crate::spi::ComplianceChatConfig;
use super::factory::ChatEngineFactory;

// ---------------------------------------------------------------------------
// YAML parsing types (private)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct AgentsYaml {
    agents: Vec<AgentEntry>,
}

#[derive(Deserialize)]
struct AgentEntry {
    name: String,
    description: String,
    trigger_keywords: Vec<String>,
    tools: Vec<String>,
    system_prompt: String,
}

// ---------------------------------------------------------------------------
// ChatAgent — implements AgentDescriptor
// ---------------------------------------------------------------------------

/// Descriptor for a chat agent, parsed from YAML config.
pub struct ChatAgent {
    id: String,
    display_name: String,
    description: String,
    system_prompt: String,
    trigger_keywords: Vec<String>,
    pub tools: Vec<String>,
}

impl AgentDescriptor for ChatAgent {
    fn id(&self) -> &str {
        &self.id
    }
    fn display_name(&self) -> &str {
        &self.display_name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn system_prompt(&self) -> &str {
        &self.system_prompt
    }
    fn trigger_keywords(&self) -> &[String] {
        &self.trigger_keywords
    }
}

impl From<AgentEntry> for ChatAgent {
    fn from(e: AgentEntry) -> Self {
        Self {
            display_name: e.description.clone(),
            id: e.name,
            description: e.description,
            system_prompt: e.system_prompt,
            trigger_keywords: e.trigger_keywords,
            tools: e.tools,
        }
    }
}

// ---------------------------------------------------------------------------
// ChatAgentManager
// ---------------------------------------------------------------------------

/// Manages chat agent descriptors and their engine factory.
pub struct ChatAgentManager {
    registry: agent_controller::AgentRegistry<ChatAgent>,
    factory: ChatEngineFactory,
    active_agent_id: String,
}

impl ChatAgentManager {
    pub fn new(llm: Arc<dyn llm_provider::LlmService>, config: ComplianceChatConfig) -> Self {
        let factory = ChatEngineFactory::new(llm, config.clone());
        let mut registry = agent_controller::AgentRegistry::new();

        for agent in load_default_agents() {
            registry.register(agent);
        }

        Self {
            registry,
            factory,
            active_agent_id: "compliance-auditor".into(),
        }
    }

    /// Return the currently-active agent descriptor (if it exists).
    pub fn active_agent(&self) -> Option<&ChatAgent> {
        self.registry.get(&self.active_agent_id)
    }

    /// List all registered agents.
    pub fn list_agents(&self) -> Vec<&ChatAgent> {
        self.registry.list()
    }

    /// Borrow the engine factory.
    pub fn factory(&self) -> &ChatEngineFactory {
        &self.factory
    }

    /// Switch the active agent by id.
    pub fn switch_agent(&mut self, id: &str) -> bool {
        if self.registry.get(id).is_some() {
            self.active_agent_id = id.to_string();
            true
        } else {
            false
        }
    }
}

/// Parse the embedded YAML into agent descriptors.
fn load_default_agents() -> Vec<ChatAgent> {
    let yaml = include_str!("default_agents.yaml");
    let config: AgentsYaml =
        serde_yml::from_str(yaml).expect("embedded agent YAML must be valid");
    config.agents.into_iter().map(ChatAgent::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_agents() {
        let agents = load_default_agents();
        assert!(!agents.is_empty());
        let auditor = agents.iter().find(|a| a.id() == "compliance-auditor");
        assert!(auditor.is_some(), "compliance-auditor agent must exist");
        let auditor = auditor.unwrap();
        assert!(!auditor.trigger_keywords().is_empty());
    }
}
