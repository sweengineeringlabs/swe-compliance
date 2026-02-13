use crate::util::api::api_get;
use crate::features::violations::violations_type::ViolationEntry;

/// Fetch violations for a scan (FR-404).
pub async fn fetch_violations(scan_id: &str) -> Result<Vec<ViolationEntry>, String> {
    let response = api_get(&format!("/scans/{scan_id}/violations")).await
        .map_err(|e| e.message)?;
    let violations: Vec<ViolationEntry> = json_parse_vec(&response)
        .map_err(|e| format!("parse error: {e}"))?;
    Ok(violations)
}
