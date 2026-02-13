use rsc_compat::prelude::*;
use crate::util::api::{api_get, api_post, ApiError};
use crate::features::ai::types::{AiStatus, ChatMessage, AuditResult, CommandGenResult};

/// Check AI subsystem availability and provider info.
/// Maps to: GET /api/v1/ai/status (FR-805)
pub async fn get_ai_status() -> Result<AiStatus, ApiError> {
    let response = api_get("/ai/status").await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse AI status response".into(),
    })?;
    AiStatus::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "AI status response missing required fields".into(),
    })
}

/// Send a chat message to the AI compliance assistant.
/// Returns the assistant reply as a ChatMessage.
/// Maps to: POST /api/v1/ai/chat (FR-800)
pub async fn send_chat(message: &str) -> Result<ChatMessage, ApiError> {
    let body = json_stringify(&json!({ "message": message }));
    let response = api_post("/ai/chat", &body).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse chat response".into(),
    })?;
    ChatMessage::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "chat response missing required fields".into(),
    })
}

/// Run an AI-driven compliance audit on the given path and scope.
/// Returns structured audit findings, scan results, and recommendations.
/// Maps to: POST /api/v1/ai/audit (FR-802)
pub async fn run_audit(path: &str, scope: &str) -> Result<AuditResult, ApiError> {
    let body = json_stringify(&json!({ "path": path, "scope": scope }));
    let response = api_post("/ai/audit", &body).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse audit response".into(),
    })?;
    AuditResult::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "audit response missing required fields".into(),
    })
}

/// Generate compliance shell commands from a set of requirements.
/// Returns a mapping of requirement IDs to commands plus any skipped entries.
/// Maps to: POST /api/v1/ai/generate-commands (FR-804)
pub async fn generate_commands(requirements_json: &str, project_context: &str) -> Result<CommandGenResult, ApiError> {
    let body = json_stringify(&json!({
        "requirements": requirements_json,
        "project_context": project_context,
    }));
    let response = api_post("/ai/generate-commands", &body).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse command generation response".into(),
    })?;
    CommandGenResult::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "command generation response missing required fields".into(),
    })
}
