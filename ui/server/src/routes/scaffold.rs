use std::path::PathBuf;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;

/// Parse SRS request (FR-500).
#[derive(Debug, Deserialize)]
pub struct ParseSrsRequest {
    pub content: String,
}

/// Parsed SRS domain response.
#[derive(Debug, Serialize)]
pub struct ParsedDomain {
    pub section: String,
    pub title: String,
    pub slug: String,
    pub requirements: Vec<ParsedRequirement>,
}

#[derive(Debug, Serialize)]
pub struct ParsedRequirement {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub description: String,
}

/// POST /api/v1/scaffold/parse — parse SRS content (FR-500).
pub async fn parse_srs(
    _user: AuthUser,
    Json(body): Json<ParseSrsRequest>,
) -> Result<Json<Vec<ParsedDomain>>, AppError> {
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("SRS content is required".into()));
    }

    let content = body.content.clone();
    let result = tokio::task::spawn_blocking(move || {
        doc_engine_scaffold::parse_srs(&content)
            .map(|domains| {
                domains
                    .into_iter()
                    .map(|d| ParsedDomain {
                        section: d.section,
                        title: d.title,
                        slug: d.slug,
                        requirements: d
                            .requirements
                            .into_iter()
                            .map(|r| ParsedRequirement {
                                id: r.id,
                                title: r.title,
                                kind: format!("{:?}", r.kind),
                                description: r.description,
                            })
                            .collect(),
                    })
                    .collect::<Vec<_>>()
            })
            .map_err(|e| format!("{e}"))
    })
    .await
    .map_err(|e| AppError::Internal(format!("parse task failed: {e}")))?
    .map_err(|e| AppError::ScaffoldError(e))?;

    Ok(Json(result))
}

/// Execute scaffold request (FR-502, FR-503).
#[derive(Debug, Deserialize)]
pub struct ExecuteScaffoldRequest {
    pub srs_path: String,
    pub output_dir: String,
    pub phases: Option<Vec<String>>,
    pub file_types: Option<Vec<String>>,
    pub force: Option<bool>,
}

/// Scaffold execution response.
#[derive(Debug, Serialize)]
pub struct ScaffoldResponse {
    pub domain_count: usize,
    pub requirement_count: usize,
    pub created: Vec<String>,
    pub skipped: Vec<String>,
}

/// POST /api/v1/scaffold/execute — execute scaffolding (FR-502, FR-503).
pub async fn execute_scaffold(
    _user: AuthUser,
    Json(body): Json<ExecuteScaffoldRequest>,
) -> Result<Json<ScaffoldResponse>, AppError> {
    // Path traversal prevention
    if body.srs_path.contains("..") || body.output_dir.contains("..") {
        return Err(AppError::BadRequest(
            "paths must not contain path traversal sequences".into(),
        ));
    }

    let srs_path = PathBuf::from(&body.srs_path);
    if !srs_path.exists() {
        return Err(AppError::BadRequest(format!(
            "SRS file not found: {}",
            body.srs_path
        )));
    }

    let output_dir = PathBuf::from(&body.output_dir);
    let phases = body.phases.unwrap_or_else(|| {
        vec![
            "requirements".into(),
            "design".into(),
            "testing".into(),
            "deployment".into(),
        ]
    });
    let file_types = body.file_types.unwrap_or_else(|| {
        vec![
            "yaml".into(),
            "spec".into(),
            "arch".into(),
            "test".into(),
            "exec".into(),
            "deploy".into(),
        ]
    });
    let force = body.force.unwrap_or(false);

    let result = tokio::task::spawn_blocking(move || {
        use doc_engine_scaffold::ScaffoldConfig;

        let config = ScaffoldConfig {
            srs_path,
            output_dir,
            force,
            phases,
            file_types,
            features: vec![],
            exclude_features: None,
            command_map_path: None,
        };

        doc_engine_scaffold::scaffold_from_srs(&config)
    })
    .await
    .map_err(|e| AppError::Internal(format!("scaffold task failed: {e}")))?
    .map_err(|e| AppError::ScaffoldError(format!("{e}")))?;

    Ok(Json(ScaffoldResponse {
        domain_count: result.domain_count,
        requirement_count: result.requirement_count,
        created: result.created.iter().map(|p| p.display().to_string()).collect(),
        skipped: result.skipped.iter().map(|p| p.display().to_string()).collect(),
    }))
}
