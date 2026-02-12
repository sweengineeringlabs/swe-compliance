use axum::extract::{Path as AxumPath, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// Violation query parameters.
#[derive(Debug, Deserialize)]
pub struct ViolationQuery {
    pub format: Option<String>,
}

/// Extracted violation record.
#[derive(Debug, Serialize)]
pub struct ViolationEntry {
    pub check_id: u32,
    pub category: String,
    pub description: String,
    pub severity: String,
    pub file_path: Option<String>,
    pub message: String,
}

/// GET /api/v1/scans/{id}/violations — extract failed checks (FR-404).
pub async fn get_violations(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    Query(query): Query<ViolationQuery>,
) -> Result<axum::response::Response, AppError> {
    let scan = state.db.get_scan(&id)?;

    if scan.status != "completed" {
        return Err(AppError::BadRequest(format!(
            "scan is not completed (status: {})",
            scan.status
        )));
    }

    let report_json = scan
        .report_json
        .ok_or_else(|| AppError::NotFound("no report data available".into()))?;

    let report: serde_json::Value =
        serde_json::from_str(&report_json).map_err(|e| AppError::Internal(format!("{e}")))?;

    let violations = extract_violations(&report);

    let format = query.format.as_deref().unwrap_or("json");
    match format {
        "csv" => {
            let csv = violations_to_csv(&violations);
            Ok((
                [(axum::http::header::CONTENT_TYPE, "text/csv")],
                csv,
            )
                .into_response())
        }
        _ => Ok(axum::Json(violations).into_response()),
    }
}

/// Extract violations from a scan report JSON.
fn extract_violations(report: &serde_json::Value) -> Vec<ViolationEntry> {
    let mut violations = Vec::new();

    if let Some(results) = report.get("results").and_then(|r| r.as_array()) {
        for entry in results {
            let check_id = entry
                .get("id")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            let category = entry
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let description = entry
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let result = entry.get("result");
            let status = result
                .and_then(|r| r.get("status"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if status == "fail" {
                if let Some(vs) = result
                    .and_then(|r| r.get("violations"))
                    .and_then(|v| v.as_array())
                {
                    for v in vs {
                        violations.push(ViolationEntry {
                            check_id,
                            category: category.clone(),
                            description: description.clone(),
                            severity: v
                                .get("severity")
                                .and_then(|s| s.as_str())
                                .unwrap_or("Error")
                                .to_string(),
                            file_path: v
                                .get("path")
                                .and_then(|p| p.as_str())
                                .map(String::from),
                            message: v
                                .get("message")
                                .and_then(|m| m.as_str())
                                .unwrap_or("")
                                .to_string(),
                        });
                    }
                }
            }
        }
    }

    violations
}

/// Convert violations to CSV format.
fn violations_to_csv(violations: &[ViolationEntry]) -> String {
    let mut csv = String::from("check_id,category,description,severity,file_path,message\n");
    for v in violations {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            v.check_id,
            csv_escape(&v.category),
            csv_escape(&v.description),
            v.severity,
            v.file_path.as_deref().unwrap_or(""),
            csv_escape(&v.message),
        ));
    }
    csv
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

use axum::response::IntoResponse;
