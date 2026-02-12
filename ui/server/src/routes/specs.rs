use std::path::{Path, PathBuf};

use axum::extract::{Path as AxumPath, State};
use axum::Json;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// Spec file metadata.
#[derive(Debug, Serialize)]
pub struct SpecFileEntry {
    pub name: String,
    pub path: String,
    pub extension: String,
    pub domain: String,
    pub size_bytes: u64,
}

/// GET /api/v1/projects/{id}/specs — discover spec files (FR-1000).
pub async fn get_specs(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Vec<SpecFileEntry>>, AppError> {
    let project = state.db.get_project(&id)?;
    let root = PathBuf::from(&project.root_path);

    if !root.exists() {
        return Err(AppError::NotFound(format!(
            "project root does not exist: {}",
            root.display()
        )));
    }

    let result = tokio::task::spawn_blocking(move || -> Result<Vec<SpecFileEntry>, AppError> {
        let mut specs = Vec::new();
        discover_specs(&root, &root, &mut specs)?;
        Ok(specs)
    })
    .await
    .map_err(|e| AppError::Internal(format!("{e}")))?;

    result.map(Json)
}

/// Recursively discover spec files (.spec, .arch, .test, .deploy, .yaml).
fn discover_specs(
    base: &Path,
    dir: &Path,
    specs: &mut Vec<SpecFileEntry>,
) -> Result<(), AppError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };

    let spec_extensions = ["spec", "arch", "test", "deploy"];

    for entry in entries {
        let entry =
            entry.map_err(|e| AppError::Internal(format!("failed to read entry: {e}")))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip hidden directories and common non-spec directories
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if dir_name.starts_with('.') || dir_name == "target" || dir_name == "node_modules" {
                continue;
            }
            discover_specs(base, &path, specs)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            if spec_extensions.contains(&ext) {
                let relative = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .display()
                    .to_string();

                // Infer domain from parent directory name
                let domain = path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let metadata = std::fs::metadata(&path)
                    .map_err(|e| AppError::Internal(format!("metadata error: {e}")))?;

                specs.push(SpecFileEntry {
                    name: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string(),
                    path: relative,
                    extension: ext.to_string(),
                    domain,
                    size_bytes: metadata.len(),
                });
            }
        }
    }

    Ok(())
}
