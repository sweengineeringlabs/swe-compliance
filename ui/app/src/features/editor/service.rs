use rsc_compat::prelude::*;
use crate::util::api::{api_get, api_post, api_put};
use crate::features::editor::types::{ValidationResult, SrsDocument};

/// Validate SRS content (FR-901).
pub async fn validate_srs(content: &str) -> Result<ValidationResult, String> {
    let body = json_stringify(&json!({ "content": content }));
    let response = api_post("/editor/validate", &body).await.map_err(|e| e.message)?;
    let value: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    ValidationResult::from_json(&value).ok_or("invalid validation result".into())
}

/// Load SRS document for a project (FR-903).
pub async fn load_srs(project_id: &str) -> Result<SrsDocument, String> {
    let response = api_get(&format!("/projects/{}/srs", project_id)).await.map_err(|e| e.message)?;
    let value: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    SrsDocument::from_json(&value).ok_or("no SRS document found".into())
}

/// Save SRS document for a project (FR-903).
pub async fn save_srs(project_id: &str, content: &str) -> Result<SrsDocument, String> {
    let body = json_stringify(&json!({ "content": content }));
    let response = api_put(&format!("/projects/{}/srs", project_id), &body).await.map_err(|e| e.message)?;
    let value: JsonValue = serde_json::from_str(&response).map_err(|e| format!("parse error: {e}"))?;
    SrsDocument::from_json(&value).ok_or("save failed".into())
}
