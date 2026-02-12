use axum::extract::ws::WebSocketUpgrade;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;
use crate::ws::handle_ai_chat_ws;

/// AI status response (FR-805).
#[derive(Debug, Serialize)]
pub struct AiStatusResponse {
    pub enabled: bool,
    pub provider: Option<String>,
}

/// Chat request (FR-800).
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

/// Chat response.
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
}

/// Audit request (FR-802).
#[derive(Debug, Deserialize)]
pub struct AuditRequest {
    pub path: String,
    pub scope: Option<String>,
}

/// Audit response.
#[derive(Debug, Serialize)]
pub struct AuditResponse {
    pub summary: String,
    pub scan_results: serde_json::Value,
    pub recommendations: Vec<String>,
}

/// Command generation request (FR-804).
#[derive(Debug, Deserialize)]
pub struct GenerateCommandsRequest {
    pub requirements: Vec<RequirementInput>,
    pub project_context: String,
}

#[derive(Debug, Deserialize)]
pub struct RequirementInput {
    pub id: String,
    pub title: String,
    pub verification: String,
    pub acceptance: String,
    pub traces_to: String,
    pub description: String,
}

/// Command generation response.
#[derive(Debug, Serialize)]
pub struct GenerateCommandsResponse {
    pub commands: std::collections::HashMap<String, String>,
    pub skipped: Vec<SkippedRequirement>,
}

#[derive(Debug, Serialize)]
pub struct SkippedRequirement {
    pub id: String,
    pub reason: String,
}

/// GET /api/v1/ai/status — check AI availability (FR-805).
pub async fn ai_status(
    _user: AuthUser,
    State(state): State<AppState>,
) -> Json<AiStatusResponse> {
    let enabled = state.config.ai_enabled && has_api_key();
    let provider = if enabled {
        detect_provider()
    } else {
        None
    };

    Json(AiStatusResponse { enabled, provider })
}

/// POST /api/v1/ai/chat — proxy to ComplianceChat (FR-800).
pub async fn ai_chat(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(_body): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    if !state.config.ai_enabled || !has_api_key() {
        return Err(AppError::ServiceUnavailable(
            "AI features are not configured — set DOC_ENGINE_AI_ENABLED=true and provide an API key".into(),
        ));
    }

    #[cfg(feature = "ai")]
    {
        use doc_engine_compliance_chat::saf::ComplianceChat;

        let config = doc_engine_compliance_chat::api::types::ComplianceChatConfig::default();
        let chat = ComplianceChat::new(config)
            .await
            .map_err(|e| AppError::Internal(format!("failed to initialize chat: {e}")))?;

        let response = chat
            .chat(&body.message)
            .await
            .map_err(|e| AppError::Internal(format!("chat error: {e}")))?;

        return Ok(Json(ChatResponse { response }));
    }

    #[cfg(not(feature = "ai"))]
    {
        Err(AppError::ServiceUnavailable(
            "AI features require the server to be compiled with the 'ai' feature flag".into(),
        ))
    }
}

/// WS /api/v1/ai/chat/stream — streaming chat via WebSocket (FR-801).
pub async fn ai_chat_stream_ws(
    _user: AuthUser,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let broadcaster = state.ws_broadcaster.clone();
    ws.on_upgrade(move |socket| handle_ai_chat_ws(socket, broadcaster))
}

/// POST /api/v1/ai/audit — call ComplianceAuditor (FR-802).
pub async fn ai_audit(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<AuditRequest>,
) -> Result<Json<AuditResponse>, AppError> {
    if !state.config.ai_enabled || !has_api_key() {
        return Err(AppError::ServiceUnavailable(
            "AI features are not configured".into(),
        ));
    }

    // Path traversal prevention
    if body.path.contains("..") {
        return Err(AppError::BadRequest(
            "path must not contain traversal sequences".into(),
        ));
    }

    #[cfg(feature = "ai")]
    {
        use doc_engine_compliance_audit::saf::ComplianceAuditor;

        let config = doc_engine_compliance_audit::api::types::AuditConfig::default();
        let auditor = ComplianceAuditor::new(config)
            .await
            .map_err(|e| AppError::Internal(format!("failed to initialize auditor: {e}")))?;

        let scope = body.scope.as_deref().unwrap_or("Small");
        let result = auditor
            .audit(&body.path, scope)
            .await
            .map_err(|e| AppError::Internal(format!("audit error: {e}")))?;

        return Ok(Json(AuditResponse {
            summary: result.summary,
            scan_results: result.scan_results,
            recommendations: result.recommendations,
        }));
    }

    #[cfg(not(feature = "ai"))]
    {
        Err(AppError::ServiceUnavailable(
            "AI features require the server to be compiled with the 'ai' feature flag".into(),
        ))
    }
}

/// POST /api/v1/ai/generate-commands — call CommandGenerator (FR-804).
pub async fn ai_generate_commands(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(_body): Json<GenerateCommandsRequest>,
) -> Result<Json<GenerateCommandsResponse>, AppError> {
    if !state.config.ai_enabled || !has_api_key() {
        return Err(AppError::ServiceUnavailable(
            "AI features are not configured".into(),
        ));
    }

    #[cfg(feature = "ai")]
    {
        use doc_engine_command_generator::api::types::{
            CommandGeneratorConfig, GenerateCommandsRequest as EngineRequest,
            RequirementContext,
        };
        use doc_engine_command_generator::api::service::CommandGenerator;

        let config = CommandGeneratorConfig::from_env();
        let generator = CommandGenerator::new(config)
            .await
            .map_err(|e| AppError::Internal(format!("failed to initialize generator: {e}")))?;

        let engine_req = EngineRequest {
            requirements: body
                .requirements
                .into_iter()
                .map(|r| RequirementContext {
                    id: r.id,
                    title: r.title,
                    verification: r.verification,
                    acceptance: r.acceptance,
                    traces_to: r.traces_to,
                    description: r.description,
                })
                .collect(),
            project_context: body.project_context,
        };

        let result = generator
            .generate_commands(&engine_req)
            .await
            .map_err(|e| AppError::Internal(format!("command generation error: {e}")))?;

        return Ok(Json(GenerateCommandsResponse {
            commands: result.commands,
            skipped: result
                .skipped
                .into_iter()
                .map(|s| SkippedRequirement {
                    id: s.id,
                    reason: s.reason,
                })
                .collect(),
        }));
    }

    #[cfg(not(feature = "ai"))]
    {
        Err(AppError::ServiceUnavailable(
            "AI features require the server to be compiled with the 'ai' feature flag".into(),
        ))
    }
}

fn has_api_key() -> bool {
    std::env::var("ANTHROPIC_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .or_else(|_| std::env::var("GEMINI_API_KEY"))
        .map(|k| !k.is_empty())
        .unwrap_or(false)
}

fn detect_provider() -> Option<String> {
    if std::env::var("ANTHROPIC_API_KEY")
        .map(|k| !k.is_empty())
        .unwrap_or(false)
    {
        Some("anthropic".into())
    } else if std::env::var("OPENAI_API_KEY")
        .map(|k| !k.is_empty())
        .unwrap_or(false)
    {
        Some("openai".into())
    } else if std::env::var("GEMINI_API_KEY")
        .map(|k| !k.is_empty())
        .unwrap_or(false)
    {
        Some("gemini".into())
    } else {
        None
    }
}
