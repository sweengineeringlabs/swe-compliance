use std::path::Path;

use axum::extract::{Path as AxumPath, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// Create project request (FR-100).
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub root_path: String,
    pub scope: Option<String>,
    pub project_type: Option<String>,
}

/// Update project request (FR-102).
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub scope: Option<String>,
    pub project_type: Option<String>,
}

/// Project response with optional compliance summary.
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub root_path: String,
    pub scope: String,
    pub project_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_scan_id: Option<String>,
    pub compliance_summary: Option<serde_json::Value>,
}

/// POST /api/v1/projects (FR-100, FR-104, NFR-201).
pub async fn create_project(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<(axum::http::StatusCode, Json<ProjectResponse>), AppError> {
    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".into()));
    }

    // Path traversal prevention (NFR-201)
    let root = &body.root_path;
    if root.contains("..") || root.contains('\0') {
        return Err(AppError::BadRequest(
            "root_path must not contain path traversal sequences".into(),
        ));
    }

    // Validate root_path exists
    if !Path::new(root).exists() {
        return Err(AppError::BadRequest(format!(
            "root_path does not exist: {root}"
        )));
    }

    let scope = body.scope.as_deref().unwrap_or("Small");
    let project_type = body.project_type.as_deref().unwrap_or("OpenSource");

    validate_scope(scope)?;
    validate_project_type(project_type)?;

    let row = state
        .db
        .create_project(&body.name, root, scope, project_type)?;

    Ok((
        axum::http::StatusCode::CREATED,
        Json(project_row_to_response(row, None)),
    ))
}

/// GET /api/v1/projects (FR-101).
pub async fn list_projects(
    _user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectResponse>>, AppError> {
    let rows = state.db.list_projects()?;

    let projects: Vec<ProjectResponse> = rows
        .into_iter()
        .map(|row| {
            let summary = row
                .last_scan_id
                .as_ref()
                .and_then(|scan_id| state.db.get_scan(scan_id).ok())
                .and_then(|scan| scan.report_json)
                .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
                .and_then(|report| report.get("summary").cloned());

            project_row_to_response(row, summary)
        })
        .collect();

    Ok(Json(projects))
}

/// GET /api/v1/projects/{id} (FR-101).
pub async fn get_project(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<ProjectResponse>, AppError> {
    let row = state.db.get_project(&id)?;

    let summary = row
        .last_scan_id
        .as_ref()
        .and_then(|scan_id| state.db.get_scan(scan_id).ok())
        .and_then(|scan| scan.report_json)
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .and_then(|report| report.get("summary").cloned());

    Ok(Json(project_row_to_response(row, summary)))
}

/// PATCH /api/v1/projects/{id} (FR-102).
pub async fn update_project(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    Json(body): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, AppError> {
    if let Some(scope) = &body.scope {
        validate_scope(scope)?;
    }
    if let Some(pt) = &body.project_type {
        validate_project_type(pt)?;
    }

    let row = state.db.update_project(
        &id,
        body.name.as_deref(),
        body.scope.as_deref(),
        body.project_type.as_deref(),
    )?;

    Ok(Json(project_row_to_response(row, None)))
}

/// DELETE /api/v1/projects/{id} — soft delete, retain scan history (FR-103).
pub async fn delete_project(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<axum::http::StatusCode, AppError> {
    state.db.delete_project(&id)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

fn project_row_to_response(
    row: crate::db::ProjectRow,
    compliance_summary: Option<serde_json::Value>,
) -> ProjectResponse {
    ProjectResponse {
        id: row.id,
        name: row.name,
        root_path: row.root_path,
        scope: row.scope,
        project_type: row.project_type,
        created_at: row.created_at,
        updated_at: row.updated_at,
        last_scan_id: row.last_scan_id,
        compliance_summary,
    }
}

fn validate_scope(scope: &str) -> Result<(), AppError> {
    match scope {
        "Small" | "Medium" | "Large" => Ok(()),
        _ => Err(AppError::BadRequest(format!(
            "invalid scope '{scope}' — must be Small, Medium, or Large"
        ))),
    }
}

fn validate_project_type(pt: &str) -> Result<(), AppError> {
    match pt {
        "OpenSource" | "Internal" => Ok(()),
        _ => Err(AppError::BadRequest(format!(
            "invalid project_type '{pt}' — must be OpenSource or Internal"
        ))),
    }
}
