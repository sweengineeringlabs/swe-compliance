use rsc_compat::prelude::*;
use crate::util::api::{api_get, ApiError};
use crate::features::struct_engine::types::{StructCheck, CrateNode};

/// Get struct engine scan results (FR-1100).
/// Maps to: GET /api/v1/scans/{scan_id}
pub async fn get_struct_results(scan_id: &str) -> Result<Vec<StructCheck>, ApiError> {
    let path = format!("/scans/{scan_id}");
    let response = api_get(&path).await?;
    let parsed: JsonValue = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse struct scan response".into(),
    })?;
    let empty_vec = vec![];
    let checks = parsed
        .get("checks")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_vec);
    Ok(checks.iter().filter_map(|v| StructCheck::from_json(v)).collect())
}

/// Get crate layout tree (FR-1101).
/// Maps to: GET /api/v1/projects/{project_id}/crate-layout
pub async fn get_crate_layout(project_id: &str) -> Result<CrateNode, ApiError> {
    let path = format!("/projects/{project_id}/crate-layout");
    let response = api_get(&path).await?;
    let parsed: JsonValue = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse crate layout response".into(),
    })?;
    CrateNode::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "crate layout response missing required fields".into(),
    })
}

/// Get project kind classification (FR-1102).
/// Maps to: GET /api/v1/projects/{project_id}/kind
pub async fn get_project_kind(project_id: &str) -> Result<String, ApiError> {
    let path = format!("/projects/{project_id}/kind");
    let response = api_get(&path).await?;
    let parsed: JsonValue = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse project kind response".into(),
    })?;
    parsed.get("kind")
        .and_then(|v| v.as_str())
        .map(|s| s.into())
        .ok_or_else(|| ApiError {
            code: "PARSE_ERROR".into(),
            message: "project kind response missing 'kind' field".into(),
        })
}
