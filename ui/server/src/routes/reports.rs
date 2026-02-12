use axum::extract::{Path as AxumPath, Query, State};
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;

/// Report query parameters.
#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub format: Option<String>,
}

/// GET /api/v1/scans/{id}/report — generate report in requested format (FR-700..702).
pub async fn get_report(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    Query(query): Query<ReportQuery>,
) -> Result<axum::response::Response, AppError> {
    let scan = state.db.get_scan(&id)?;

    if scan.status != "completed" {
        return Err(AppError::BadRequest(format!(
            "scan is not completed (status: {})",
            scan.status
        )));
    }

    let report_json_str = scan
        .report_json
        .ok_or_else(|| AppError::NotFound("no report data available".into()))?;

    let format = query.format.as_deref().unwrap_or("json");

    match format {
        "json" => Ok((
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            report_json_str,
        )
            .into_response()),
        "markdown" => {
            let report: serde_json::Value = serde_json::from_str(&report_json_str)
                .map_err(|e| AppError::Internal(format!("{e}")))?;
            let md = report_to_markdown(&report, &scan.engine);
            Ok((
                [(axum::http::header::CONTENT_TYPE, "text/markdown")],
                md,
            )
                .into_response())
        }
        "pdf" => {
            // PDF generation placeholder — requires printpdf or genpdf crate
            Err(AppError::BadRequest(
                "PDF export is not yet implemented — use json or markdown format".into(),
            ))
        }
        _ => Err(AppError::BadRequest(format!(
            "unsupported format '{format}' — use json, markdown, or pdf"
        ))),
    }
}

/// GET /api/v1/scans/{id}/audit-report — ISO 15289 audit status report (FR-704).
pub async fn get_audit_report(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<axum::response::Response, AppError> {
    let scan = state.db.get_scan(&id)?;

    if scan.status != "completed" {
        return Err(AppError::BadRequest(format!(
            "scan is not completed (status: {})",
            scan.status
        )));
    }

    let report_json_str = scan
        .report_json
        .ok_or_else(|| AppError::NotFound("no report data available".into()))?;

    let report: serde_json::Value = serde_json::from_str(&report_json_str)
        .map_err(|e| AppError::Internal(format!("{e}")))?;

    let audit = generate_audit_report(&report, &scan.engine, &id);

    Ok((
        [(axum::http::header::CONTENT_TYPE, "text/markdown")],
        audit,
    )
        .into_response())
}

/// Convert a scan report to markdown format.
fn report_to_markdown(report: &serde_json::Value, engine: &str) -> String {
    let mut md = String::new();

    let standard = report
        .get("standard")
        .and_then(|v| v.as_str())
        .unwrap_or("N/A");
    let timestamp = report
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("N/A");

    md.push_str(&format!("# Compliance Scan Report — {engine}\n\n"));
    md.push_str(&format!("**Standard:** {standard}\n"));
    md.push_str(&format!("**Timestamp:** {timestamp}\n\n"));

    if let Some(summary) = report.get("summary") {
        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "| Metric | Count |\n|--------|-------|\n| Passed | {} |\n| Failed | {} |\n| Skipped | {} |\n| Total | {} |\n\n",
            summary.get("passed").and_then(|v| v.as_u64()).unwrap_or(0),
            summary.get("failed").and_then(|v| v.as_u64()).unwrap_or(0),
            summary.get("skipped").and_then(|v| v.as_u64()).unwrap_or(0),
            summary.get("total").and_then(|v| v.as_u64()).unwrap_or(0),
        ));
    }

    if let Some(results) = report.get("results").and_then(|r| r.as_array()) {
        md.push_str("## Results\n\n");
        md.push_str("| ID | Category | Description | Status |\n");
        md.push_str("|----|----------|-------------|--------|\n");

        for entry in results {
            let id = entry.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
            let cat = entry
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let desc = entry
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let status = entry
                .get("result")
                .and_then(|r| r.get("status"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            md.push_str(&format!("| {id} | {cat} | {desc} | {status} |\n"));
        }
    }

    md
}

/// Generate an ISO 15289 audit status report.
fn generate_audit_report(report: &serde_json::Value, engine: &str, scan_id: &str) -> String {
    let mut md = String::new();

    md.push_str("# Audit Status Report (ISO/IEC/IEEE 15289)\n\n");
    md.push_str(&format!("**Report ID:** ASR-{}\n", &scan_id[..8]));
    md.push_str(&format!("**Engine:** {engine}\n"));
    md.push_str(&format!(
        "**Date:** {}\n\n",
        report
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A")
    ));

    md.push_str("## 1. Audit Overview\n\n");
    md.push_str("This report presents the results of an automated compliance audit ");
    md.push_str("against the project's documentation and structural requirements.\n\n");

    if let Some(summary) = report.get("summary") {
        let total = summary.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
        let passed = summary.get("passed").and_then(|v| v.as_u64()).unwrap_or(0);
        let compliance_pct = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        md.push_str("## 2. Compliance Summary\n\n");
        md.push_str(&format!("- **Compliance Rate:** {compliance_pct:.1}%\n"));
        md.push_str(&format!(
            "- **Checks Passed:** {passed}/{total}\n"
        ));
        md.push_str(&format!(
            "- **Non-conformances:** {}\n",
            summary.get("failed").and_then(|v| v.as_u64()).unwrap_or(0)
        ));
        md.push_str(&format!(
            "- **Not Evaluated:** {}\n\n",
            summary.get("skipped").and_then(|v| v.as_u64()).unwrap_or(0)
        ));
    }

    md.push_str("## 3. Non-conformance Details\n\n");

    if let Some(results) = report.get("results").and_then(|r| r.as_array()) {
        for entry in results {
            let status = entry
                .get("result")
                .and_then(|r| r.get("status"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if status == "fail" {
                let id = entry.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let desc = entry
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let cat = entry
                    .get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                md.push_str(&format!("### Check {id}: {desc}\n\n"));
                md.push_str(&format!("- **Category:** {cat}\n"));

                if let Some(violations) = entry
                    .get("result")
                    .and_then(|r| r.get("violations"))
                    .and_then(|v| v.as_array())
                {
                    for v in violations {
                        let msg = v.get("message").and_then(|m| m.as_str()).unwrap_or("");
                        let path = v.get("path").and_then(|p| p.as_str()).unwrap_or("");
                        md.push_str(&format!("- {msg}"));
                        if !path.is_empty() {
                            md.push_str(&format!(" (`{path}`)"));
                        }
                        md.push('\n');
                    }
                }
                md.push('\n');
            }
        }
    }

    md.push_str("## 4. Recommendations\n\n");
    md.push_str("Address all non-conformances listed above to achieve full compliance.\n");

    md
}
