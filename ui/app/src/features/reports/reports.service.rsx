use crate::util::api::{api_get, ApiError};
use crate::features::reports::reports_type::{ReportData, ReportComparison};

/// Export scan report in specified format (FR-700, FR-701).
/// Maps to: GET /api/v1/scans/{id}/report?format={format}
pub async fn export_report(scan_id: &str, format: &str) -> Result<ReportData, ApiError> {
    let path = format!("/scans/{scan_id}/report?format={format}");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse report response".into(),
    })?;
    ReportData::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "report response missing required fields".into(),
    })
}

/// Get ISO 15289 audit report (FR-704).
/// Maps to: GET /api/v1/scans/{id}/audit-report
pub async fn get_audit_report(scan_id: &str) -> Result<ReportData, ApiError> {
    let path = format!("/scans/{scan_id}/audit-report");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse audit report response".into(),
    })?;
    ReportData::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "audit report response missing required fields".into(),
    })
}

/// Compare two scan reports (FR-703).
/// Maps to: GET /api/v1/scans/{scan_a_id}/compare/{scan_b_id}
pub async fn compare_reports(scan_a_id: &str, scan_b_id: &str) -> Result<ReportComparison, ApiError> {
    let path = format!("/scans/{scan_a_id}/compare/{scan_b_id}");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse comparison response".into(),
    })?;
    ReportComparison::from_json(&parsed).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "comparison response missing required fields".into(),
    })
}
