use rsc_compat::prelude::*;

/// AI subsystem availability status returned by the health endpoint.
#[derive(Clone, Debug)]
pub struct AiStatus {
    /// Whether the AI backend is enabled and reachable.
    pub enabled: bool,
    /// Name of the configured provider (e.g. "openai", "anthropic").
    /// None when AI is disabled or no provider is set.
    pub provider: Option<String>,
}

impl AiStatus {
    /// Parse an AiStatus from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(AiStatus {
            enabled: value.get_bool("enabled").unwrap_or_default(),
            provider: value.get_str("provider").map(|s| s.into()),
        })
    }

    /// Human-readable label for the current status.
    pub fn label(&self) -> String {
        if self.enabled {
            match &self.provider {
                Some(p) => format!("AI Enabled ({})", p),
                None => "AI Enabled".into(),
            }
        } else {
            "AI Disabled".into()
        }
    }

    /// Badge variant for rsc-ui rendering.
    pub fn badge_variant(&self) -> &str {
        if self.enabled {
            "success"
        } else {
            "warning"
        }
    }
}

/// A single message in a compliance chat conversation.
#[derive(Clone, Debug)]
pub struct ChatMessage {
    /// Role of the message author: "user" or "assistant".
    pub role: String,
    /// Textual content of the message.
    pub content: String,
}

impl ChatMessage {
    /// Parse a ChatMessage from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ChatMessage {
            role: value.get_str("role").unwrap_or_default().into(),
            content: value.get_str("content").unwrap_or_default().into(),
        })
    }

    /// Serialize the message to a JSON string for API submission.
    pub fn to_json(&self) -> String {
        let obj = json!({
            "role": self.role,
            "content": self.content,
        });
        json_stringify(&obj)
    }

    /// Returns true when the message was sent by the user.
    pub fn is_user(&self) -> bool {
        self.role == "user"
    }

    /// Returns true when the message was sent by the assistant.
    pub fn is_assistant(&self) -> bool {
        self.role == "assistant"
    }
}

/// Result of an AI-driven compliance audit scan.
#[derive(Clone, Debug)]
pub struct AuditResult {
    /// High-level summary of the audit findings.
    pub summary: String,
    /// Raw scan results as a JSON value for flexible rendering.
    pub scan_results: JsonValue,
    /// Actionable recommendations produced by the AI.
    pub recommendations: Vec<String>,
}

impl AuditResult {
    /// Parse an AuditResult from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let summary = value.get_str("summary").unwrap_or_default().into();
        let scan_results = value.get("scan_results").cloned().unwrap_or(json!({}));
        let rec_array = value.get("recommendations")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let recommendations = rec_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.into()))
            .collect();
        Some(AuditResult {
            summary,
            scan_results,
            recommendations,
        })
    }
}

/// A single requirement input used when generating compliance commands.
#[derive(Clone, Debug)]
pub struct RequirementInput {
    /// Unique identifier for the requirement (e.g. "REQ-001").
    pub id: String,
    /// Short title describing the requirement.
    pub title: String,
    /// Verification method or criteria.
    pub verification: String,
    /// Acceptance criteria that must be satisfied.
    pub acceptance: String,
    /// Upstream requirement or artifact this traces to.
    pub traces_to: String,
    /// Full description of the requirement.
    pub description: String,
}

impl RequirementInput {
    /// Parse a RequirementInput from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(RequirementInput {
            id: value.get_str("id").unwrap_or_default().into(),
            title: value.get_str("title").unwrap_or_default().into(),
            verification: value.get_str("verification").unwrap_or_default().into(),
            acceptance: value.get_str("acceptance").unwrap_or_default().into(),
            traces_to: value.get_str("traces_to").unwrap_or_default().into(),
            description: value.get_str("description").unwrap_or_default().into(),
        })
    }

    /// Serialize to a JSON value for API submission.
    pub fn to_json_value(&self) -> JsonValue {
        json!({
            "id": self.id,
            "title": self.title,
            "verification": self.verification,
            "acceptance": self.acceptance,
            "traces_to": self.traces_to,
            "description": self.description,
        })
    }
}

/// Request body for the command generation endpoint.
#[derive(Clone, Debug)]
pub struct CommandGenRequest {
    /// List of requirements to generate commands for.
    pub requirements: Vec<RequirementInput>,
    /// Contextual information about the project (language, framework, etc.).
    pub project_context: String,
}

impl CommandGenRequest {
    /// Serialize the request to a JSON string for the API call.
    pub fn to_json(&self) -> String {
        let reqs: Vec<JsonValue> = self.requirements
            .iter()
            .map(|r| r.to_json_value())
            .collect();
        let obj = json!({
            "requirements": reqs,
            "project_context": self.project_context,
        });
        json_stringify(&obj)
    }
}

/// A requirement that was skipped during command generation.
#[derive(Clone, Debug)]
pub struct SkippedReq {
    /// Identifier of the skipped requirement.
    pub id: String,
    /// Reason the requirement could not produce a command.
    pub reason: String,
}

impl SkippedReq {
    /// Parse a SkippedReq from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(SkippedReq {
            id: value.get_str("id").unwrap_or_default().into(),
            reason: value.get_str("reason").unwrap_or_default().into(),
        })
    }
}

/// Result of the AI command generation endpoint.
#[derive(Clone, Debug)]
pub struct CommandGenResult {
    /// Mapping of requirement ID to generated shell command.
    pub commands: Map<String, String>,
    /// Requirements that could not be processed.
    pub skipped: Vec<SkippedReq>,
}

impl CommandGenResult {
    /// Parse a CommandGenResult from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        let empty_map = serde_json::Map::new();
        let commands_obj = value.get("commands")
            .and_then(|v| v.as_object())
            .unwrap_or(&empty_map);
        let mut commands = Map::new();
        for (key, val) in commands_obj.iter() {
            if let Some(cmd) = val.as_str() {
                commands.insert(key.clone(), cmd.into());
            }
        }

        let skipped_arr = value.get("skipped")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let skipped = skipped_arr
            .iter()
            .filter_map(|v| SkippedReq::from_json(v))
            .collect();

        Some(CommandGenResult { commands, skipped })
    }

    /// Total number of successfully generated commands.
    pub fn generated_count(&self) -> usize {
        self.commands.len()
    }

    /// Total number of skipped requirements.
    pub fn skipped_count(&self) -> usize {
        self.skipped.len()
    }
}
