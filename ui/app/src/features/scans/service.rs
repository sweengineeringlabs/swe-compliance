use rsc_compat::prelude::*;
use crate::util::api::{api_get, api_post, ApiError};
use crate::features::scans::types::{Scan, ScanRequest};

/// Create a new scan by posting a ScanRequest to the API.
/// Returns the newly created Scan (with status "queued") on success.
/// Maps to: POST /api/v1/scans (FR-300, FR-301, FR-304)
pub async fn create_scan(request: &ScanRequest) -> Result<Scan, ApiError> {
    let body = request.to_json();
    let response = api_post("/scans", &body).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse scan creation response".into(),
    })?;
    Scan::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "scan response missing required fields".into(),
    })
}

/// Retrieve a single scan by its ID.
/// Returns the full Scan record including report data when complete.
/// Maps to: GET /api/v1/scans/{id} (FR-303)
pub async fn get_scan(scan_id: &str) -> Result<Scan, ApiError> {
    let path = format!("/scans/{scan_id}");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse scan response".into(),
    })?;
    Scan::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "scan response missing required fields".into(),
    })
}

/// List all scans for a given project, ordered by timestamp descending.
/// Maps to: GET /api/v1/projects/{id}/scans (FR-305)
pub async fn list_project_scans(project_id: &str) -> Result<Vec<Scan>, ApiError> {
    let path = format!("/projects/{project_id}/scans");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse scan list response".into(),
    })?;
    let array = parsed.as_array().ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "expected array of scans".into(),
    })?;

    let mut scans = Vec::new();
    for item in array {
        if let Some(scan) = Scan::from_json(item) {
            scans.push(scan);
        }
    }
    Ok(scans)
}
