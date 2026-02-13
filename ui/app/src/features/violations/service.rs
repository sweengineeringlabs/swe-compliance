use rsc_compat::prelude::*;
use crate::util::api::api_get;
use crate::features::violations::types::ViolationEntry;

/// Fetch violations for a scan (FR-404).
pub async fn fetch_violations(scan_id: &str) -> Result<Vec<ViolationEntry>, String> {
    let response = api_get(&format!("/scans/{scan_id}/violations")).await
        .map_err(|e| e.message)?;
    let parsed = json_parse(&response).ok_or_else(|| "failed to parse violations response".to_string())?;
    let array = parsed.as_array().ok_or_else(|| "expected array of violations".to_string())?;

    let mut violations = Vec::new();
    for item in array {
        let entry = ViolationEntry {
            check_id: item.get("check_id").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            category: item.get("category").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            description: item.get("description").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            severity: item.get("severity").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            file_path: item.get("file_path").and_then(|v| v.as_str()).map(|s| s.to_string()),
            message: item.get("message").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        };
        violations.push(entry);
    }
    Ok(violations)
}
