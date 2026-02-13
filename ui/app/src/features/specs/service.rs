use rsc_compat::prelude::*;
use crate::util::api::api_get;
use crate::features::specs::types::{SpecFile, SpecDirectory, BrdEntry};

/// List spec files for a project (FR-1000).
pub async fn list_specs(project_id: &str) -> Result<Vec<SpecFile>, String> {
    let response = api_get(&format!("/projects/{}/specs", project_id)).await.map_err(|e| e.message)?;
    let parsed: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    let arr = parsed.as_array().ok_or_else(|| "expected array".to_string())?;
    Ok(arr.iter().filter_map(|v| SpecFile::from_json(v)).collect())
}

/// Get spec file content (FR-1002).
pub async fn get_spec_content(project_id: &str, file_path: &str) -> Result<String, String> {
    let response = api_get(&format!("/projects/{}/specs/content?path={}", project_id, file_path)).await.map_err(|e| e.message)?;
    Ok(response)
}

/// Get spec directory tree (FR-1001).
pub async fn get_spec_tree(project_id: &str) -> Result<SpecDirectory, String> {
    let response = api_get(&format!("/projects/{}/specs/tree", project_id)).await.map_err(|e| e.message)?;
    let value: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    SpecDirectory::from_json(&value).ok_or("invalid spec tree".into())
}

/// List BRD entries for a project (FR-1003).
pub async fn list_brd_entries(project_id: &str) -> Result<Vec<BrdEntry>, String> {
    let response = api_get(&format!("/projects/{}/brd", project_id)).await.map_err(|e| e.message)?;
    let parsed: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    let arr = parsed.as_array().ok_or_else(|| "expected array".to_string())?;
    Ok(arr.iter().filter_map(|v| BrdEntry::from_json(v)).collect())
}
