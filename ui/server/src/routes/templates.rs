use std::path::{Path, PathBuf};

use axum::extract::{Path as AxumPath, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// Template metadata.
#[derive(Debug, Serialize)]
pub struct TemplateEntry {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

/// Copy template request.
#[derive(Debug, Deserialize)]
pub struct CopyTemplateRequest {
    pub destination: String,
}

/// GET /api/v1/templates — list template-engine templates (FR-600).
pub async fn list_templates(
    _user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<TemplateEntry>>, AppError> {
    let template_dir = state
        .config
        .template_dir
        .as_ref()
        .ok_or_else(|| {
            AppError::BadRequest(
                "template directory not configured — set SWE_TEMPLATE_DIR".into(),
            )
        })?;

    if !template_dir.exists() {
        return Err(AppError::NotFound(format!(
            "template directory does not exist: {}",
            template_dir.display()
        )));
    }

    let dir = template_dir.clone();
    let entries = tokio::task::spawn_blocking(move || -> Result<Vec<TemplateEntry>, AppError> {
        let mut templates = Vec::new();
        collect_templates(&dir, &dir, &mut templates)?;
        Ok(templates)
    })
    .await
    .map_err(|e| AppError::Internal(format!("{e}")))?;

    entries.map(Json)
}

/// POST /api/v1/templates/{name}/copy — copy template to project directory (FR-602).
pub async fn copy_template(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(name): AxumPath<String>,
    Json(body): Json<CopyTemplateRequest>,
) -> Result<axum::http::StatusCode, AppError> {
    // Path traversal prevention
    if name.contains("..") || body.destination.contains("..") {
        return Err(AppError::BadRequest(
            "paths must not contain path traversal sequences".into(),
        ));
    }

    let template_dir = state
        .config
        .template_dir
        .as_ref()
        .ok_or_else(|| {
            AppError::BadRequest(
                "template directory not configured — set SWE_TEMPLATE_DIR".into(),
            )
        })?;

    let source = template_dir.join(&name);
    if !source.exists() {
        return Err(AppError::NotFound(format!("template not found: {name}")));
    }

    let destination = PathBuf::from(&body.destination);

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        if source.is_file() {
            let dest_file = destination.join(
                source
                    .file_name()
                    .ok_or_else(|| AppError::Internal("invalid source filename".into()))?,
            );
            std::fs::create_dir_all(&destination)
                .map_err(|e| AppError::Internal(format!("failed to create directory: {e}")))?;
            std::fs::copy(&source, &dest_file)
                .map_err(|e| AppError::Internal(format!("failed to copy template: {e}")))?;
        } else if source.is_dir() {
            copy_dir_recursive(&source, &destination)?;
        }
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("{e}")))?
    .map_err(|e: AppError| e)?;

    Ok(axum::http::StatusCode::CREATED)
}

fn collect_templates(
    base: &Path,
    dir: &Path,
    templates: &mut Vec<TemplateEntry>,
) -> Result<(), AppError> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| AppError::Internal(format!("failed to read directory: {e}")))?;

    for entry in entries {
        let entry =
            entry.map_err(|e| AppError::Internal(format!("failed to read entry: {e}")))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|e| AppError::Internal(format!("failed to read metadata: {e}")))?;

        if metadata.is_file() {
            let relative = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .display()
                .to_string();
            templates.push(TemplateEntry {
                name: relative.clone(),
                path: relative,
                size_bytes: metadata.len(),
            });
        } else if metadata.is_dir() {
            collect_templates(base, &path, templates)?;
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), AppError> {
    std::fs::create_dir_all(dst)
        .map_err(|e| AppError::Internal(format!("failed to create directory: {e}")))?;

    for entry in std::fs::read_dir(src)
        .map_err(|e| AppError::Internal(format!("failed to read directory: {e}")))?
    {
        let entry =
            entry.map_err(|e| AppError::Internal(format!("failed to read entry: {e}")))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .map_err(|e| AppError::Internal(format!("failed to copy file: {e}")))?;
        }
    }

    Ok(())
}
