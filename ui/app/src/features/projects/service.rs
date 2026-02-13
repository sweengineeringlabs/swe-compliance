use crate::util::api::{api_get, api_post, api_patch, api_delete, ApiError};
use crate::features::projects::types::{
    Project, CreateProjectRequest, UpdateProjectRequest,
};

/// API service for project CRUD operations (FR-100..FR-104).
///
/// All functions delegate to the shared API client in `crate::util::api`,
/// which handles JWT injection, error parsing, and response deserialization.
///
/// Reference: docs/1-requirements/project_management/project_management.spec

/// Create a new compliance project.
/// `POST /api/v1/projects` (FR-100)
///
/// Returns the created Project with a server-assigned ID on success,
/// or an ApiError with code "VALIDATION_ERROR" (422) if inputs are invalid.
pub async fn create_project(req: &CreateProjectRequest) -> Result<Project, ApiError> {
    let body = serde_json::to_string(req).unwrap_or_default();
    let response = api_post("/projects", &body).await?;
    let project: Project = serde_json::from_str(&response)
        .ok()
        .ok_or_else(|| ApiError {
            code: "PARSE_ERROR".into(),
            message: "failed to parse project response".into(),
        })?;
    Ok(project)
}

/// List all projects.
/// `GET /api/v1/projects` (FR-101)
///
/// Returns a Vec of all projects with their compliance summaries.
pub async fn list_projects() -> Result<Vec<Project>, ApiError> {
    let response = api_get("/projects").await?;
    let projects: Vec<Project> = serde_json::from_str(&response)
        .ok()
        .ok_or_else(|| ApiError {
            code: "PARSE_ERROR".into(),
            message: "failed to parse projects list response".into(),
        })?;
    Ok(projects)
}

/// Retrieve a single project by ID.
/// `GET /api/v1/projects/{id}` (FR-101)
///
/// Returns the Project if found, or an ApiError with code "HTTP_404" if not found.
pub async fn get_project(id: &str) -> Result<Project, ApiError> {
    let path = format!("/projects/{id}");
    let response = api_get(&path).await?;
    let project: Project = serde_json::from_str(&response)
        .ok()
        .ok_or_else(|| ApiError {
            code: "PARSE_ERROR".into(),
            message: "failed to parse project response".into(),
        })?;
    Ok(project)
}

/// Update an existing project's configuration.
/// `PATCH /api/v1/projects/{id}` (FR-102)
///
/// Only fields present in the UpdateProjectRequest are modified.
/// Returns the updated Project on success.
pub async fn update_project(id: &str, req: &UpdateProjectRequest) -> Result<Project, ApiError> {
    let path = format!("/projects/{id}");
    let body = serde_json::to_string(req).unwrap_or_default();
    let response = api_patch(&path, &body).await?;
    let project: Project = serde_json::from_str(&response)
        .ok()
        .ok_or_else(|| ApiError {
            code: "PARSE_ERROR".into(),
            message: "failed to parse project response".into(),
        })?;
    Ok(project)
}

/// Delete a project.
/// `DELETE /api/v1/projects/{id}` (FR-103)
///
/// The project is removed but associated scan history is retained for audit trail.
/// Returns Ok(()) on success (204 No Content).
pub async fn delete_project(id: &str) -> Result<(), ApiError> {
    let path = format!("/projects/{id}");
    api_delete(&path).await?;
    Ok(())
}
