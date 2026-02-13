use crate::util::api::{api_get, api_post};
use crate::features::templates::templates_type::{TemplateEntry, ChecklistItem, TemplateCopyResult};

/// List available templates (FR-600).
pub async fn list_templates() -> Result<Vec<TemplateEntry>, String> {
    let response = api_get("/templates").await.map_err(|e| e.message)?;
    let arr = json_parse_array(&response).map_err(|e| format!("parse error: {e}"))?;
    Ok(arr.iter().filter_map(|v| TemplateEntry::from_json(v)).collect())
}

/// Get template details with file listing (FR-601).
pub async fn get_template(name: &str) -> Result<TemplateEntry, String> {
    let response = api_get(&format!("/templates/{}", name)).await.map_err(|e| e.message)?;
    let value = json_parse_obj(&response).map_err(|e| format!("parse error: {e}"))?;
    TemplateEntry::from_json(&value).ok_or("invalid template data".into())
}

/// Copy template to project directory (FR-602).
pub async fn copy_template(name: &str, project_id: &str, destination: &str) -> Result<TemplateCopyResult, String> {
    let body = json_stringify(&json!({ "project_id": project_id, "destination": destination }));
    let response = api_post(&format!("/templates/{}/copy", name), &body).await.map_err(|e| e.message)?;
    let value = json_parse_obj(&response).map_err(|e| format!("parse error: {e}"))?;
    TemplateCopyResult::from_json(&value).ok_or("invalid copy result".into())
}

/// Fetch compliance checklist for a template (FR-603).
pub async fn get_checklist(template_name: &str) -> Result<Vec<ChecklistItem>, String> {
    let response = api_get(&format!("/templates/{}/checklist", template_name)).await.map_err(|e| e.message)?;
    let arr = json_parse_array(&response).map_err(|e| format!("parse error: {e}"))?;
    Ok(arr.iter().filter_map(|v| ChecklistItem::from_json(v)).collect())
}
