use axum::extract::{Path as AxumPath, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// SRS validation request (FR-901).
#[derive(Debug, Deserialize)]
pub struct ValidateSrsRequest {
    pub content: String,
}

/// SRS validation response.
#[derive(Debug, Serialize)]
pub struct ValidateSrsResponse {
    pub valid: bool,
    pub domain_count: usize,
    pub requirement_count: usize,
    pub domains: Vec<DomainSummary>,
}

#[derive(Debug, Serialize)]
pub struct DomainSummary {
    pub section: String,
    pub title: String,
    pub requirement_count: usize,
}

/// SRS content response.
#[derive(Debug, Serialize)]
pub struct SrsContentResponse {
    pub project_id: String,
    pub content: String,
    pub updated_at: String,
}

/// SRS save request.
#[derive(Debug, Deserialize)]
pub struct SaveSrsRequest {
    pub content: String,
}

/// POST /api/v1/editor/validate — validate SRS content (FR-901).
pub async fn validate_srs(
    _user: AuthUser,
    Json(body): Json<ValidateSrsRequest>,
) -> Result<Json<ValidateSrsResponse>, AppError> {
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("SRS content is required".into()));
    }

    let content = body.content.clone();
    let result = tokio::task::spawn_blocking(move || {
        match doc_engine_scaffold::parse_srs(&content) {
            Ok(domains) => {
                let domain_summaries: Vec<DomainSummary> = domains
                    .iter()
                    .map(|d| DomainSummary {
                        section: d.section.clone(),
                        title: d.title.clone(),
                        requirement_count: d.requirements.len(),
                    })
                    .collect();

                let requirement_count: usize =
                    domains.iter().map(|d| d.requirements.len()).sum();

                ValidateSrsResponse {
                    valid: !domains.is_empty(),
                    domain_count: domains.len(),
                    requirement_count,
                    domains: domain_summaries,
                }
            }
            Err(_) => ValidateSrsResponse {
                valid: false,
                domain_count: 0,
                requirement_count: 0,
                domains: vec![],
            },
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("validation task failed: {e}")))?;

    Ok(Json(result))
}

/// GET /api/v1/projects/{id}/srs — load SRS content (FR-903).
pub async fn get_srs(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<SrsContentResponse>, AppError> {
    // Validate project exists
    let _ = state.db.get_project(&id)?;

    let srs = state.db.get_srs(&id)?;

    match srs {
        Some(row) => Ok(Json(SrsContentResponse {
            project_id: row.project_id,
            content: row.content,
            updated_at: row.updated_at,
        })),
        None => Ok(Json(SrsContentResponse {
            project_id: id,
            content: String::new(),
            updated_at: String::new(),
        })),
    }
}

/// PUT /api/v1/projects/{id}/srs — save SRS content (FR-903).
pub async fn save_srs(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    Json(body): Json<SaveSrsRequest>,
) -> Result<Json<SrsContentResponse>, AppError> {
    // Validate project exists
    let _ = state.db.get_project(&id)?;

    let row = state.db.save_srs(&id, &body.content)?;

    Ok(Json(SrsContentResponse {
        project_id: row.project_id,
        content: row.content,
        updated_at: row.updated_at,
    }))
}
