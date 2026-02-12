use std::path::PathBuf;

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path as AxumPath, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::routes::AppState;
use crate::ws::{handle_scan_progress_ws, ProgressMessage};

/// Create scan request (FR-300, FR-301).
#[derive(Debug, Deserialize)]
pub struct CreateScanRequest {
    pub project_id: String,
    pub engine: String,
    pub checks: Option<String>,
    pub phase: Option<String>,
    pub module: Option<String>,
}

/// Scan response.
#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub id: String,
    pub project_id: String,
    pub engine: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub report: Option<serde_json::Value>,
}

/// Trend query parameters.
#[derive(Debug, Deserialize)]
pub struct TrendQuery {
    pub since: Option<String>,
}

/// POST /api/v1/scans — queue and execute a scan (FR-300, FR-301).
pub async fn create_scan(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateScanRequest>,
) -> Result<(axum::http::StatusCode, Json<ScanResponse>), AppError> {
    // Validate engine type
    if body.engine != "doc-engine" && body.engine != "struct-engine" {
        return Err(AppError::BadRequest(
            "engine must be 'doc-engine' or 'struct-engine'".into(),
        ));
    }

    // Validate project exists
    let project = state.db.get_project(&body.project_id)?;

    let config_json = serde_json::to_string(&serde_json::json!({
        "checks": body.checks,
        "phase": body.phase,
        "module": body.module,
    }))
    .unwrap();

    // Acquire scan semaphore permit
    let _permit = state.scan_semaphore.acquire().await?;

    let scan = state
        .db
        .create_scan(&body.project_id, &body.engine, Some(&config_json))?;

    let scan_id = scan.id.clone();
    let response = ScanResponse {
        id: scan.id.clone(),
        project_id: scan.project_id.clone(),
        engine: scan.engine.clone(),
        status: scan.status.clone(),
        started_at: scan.started_at.clone(),
        finished_at: None,
        report: None,
    };

    // Create WebSocket broadcast channel for this scan
    let tx = state.ws_broadcaster.create_channel(&scan_id).await;

    // Spawn the scan in a background task
    let db = state.db.clone();
    let broadcaster = state.ws_broadcaster.clone();
    let engine = body.engine.clone();
    let root_path = PathBuf::from(&project.root_path);
    let scope_str = project.scope.clone();
    let ptype_str = project.project_type.clone();
    let checks_filter = body.checks.clone();
    let phase_filter = body.phase.clone();
    let module_filter = body.module.clone();

    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || {
            if engine == "doc-engine" {
                run_doc_scan(
                    &root_path,
                    &scope_str,
                    &ptype_str,
                    checks_filter.as_deref(),
                    phase_filter.as_deref(),
                    module_filter.as_deref(),
                )
            } else {
                run_struct_scan(&root_path, checks_filter.as_deref())
            }
        })
        .await;

        match result {
            Ok(Ok(report_json)) => {
                let _ = db.finish_scan(&scan_id, "completed", Some(&report_json));
            }
            Ok(Err(err)) => {
                let error_json = serde_json::json!({"error": err}).to_string();
                let _ = db.finish_scan(&scan_id, "failed", Some(&error_json));
            }
            Err(e) => {
                let error_json =
                    serde_json::json!({"error": format!("task panicked: {e}")}).to_string();
                let _ = db.finish_scan(&scan_id, "failed", Some(&error_json));
            }
        }

        // Signal completion on the WebSocket channel
        let _ = tx.send("__DONE__".into());
        broadcaster.remove_channel(&scan_id).await;
    });

    Ok((axum::http::StatusCode::ACCEPTED, Json(response)))
}

/// GET /api/v1/scans/{id} — return scan result or in_progress status (FR-303, FR-1100).
pub async fn get_scan(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<ScanResponse>, AppError> {
    let scan = state.db.get_scan(&id)?;

    let report = scan
        .report_json
        .as_ref()
        .and_then(|j| serde_json::from_str(j).ok());

    Ok(Json(ScanResponse {
        id: scan.id,
        project_id: scan.project_id,
        engine: scan.engine,
        status: scan.status,
        started_at: scan.started_at,
        finished_at: scan.finished_at,
        report,
    }))
}

/// WS /api/v1/scans/{id}/progress — stream per-check progress (FR-302).
pub async fn scan_progress_ws(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let broadcaster = state.ws_broadcaster.clone();
    ws.on_upgrade(move |socket| handle_scan_progress_ws(socket, broadcaster, id))
}

/// GET /api/v1/projects/{id}/scans — scan history (FR-305).
pub async fn list_project_scans(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Vec<ScanResponse>>, AppError> {
    // Validate project exists
    let _ = state.db.get_project(&id)?;

    let scans = state.db.list_scans_for_project(&id)?;
    let responses: Vec<ScanResponse> = scans
        .into_iter()
        .map(|s| {
            let report = s
                .report_json
                .as_ref()
                .and_then(|j| serde_json::from_str(j).ok());

            ScanResponse {
                id: s.id,
                project_id: s.project_id,
                engine: s.engine,
                status: s.status,
                started_at: s.started_at,
                finished_at: s.finished_at,
                report,
            }
        })
        .collect();

    Ok(Json(responses))
}

/// GET /api/v1/projects/{id}/trends — aggregate pass/fail/skip over time (FR-202).
pub async fn get_trends(
    _user: AuthUser,
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
    Query(query): Query<TrendQuery>,
) -> Result<Json<Vec<crate::db::TrendPoint>>, AppError> {
    let _ = state.db.get_project(&id)?;
    let trends = state.db.get_trends(&id, query.since.as_deref())?;
    Ok(Json(trends))
}

/// Run a doc-engine scan.
fn run_doc_scan(
    root: &std::path::Path,
    scope: &str,
    project_type: &str,
    checks: Option<&str>,
    phases: Option<&str>,
    module: Option<&str>,
) -> Result<String, String> {
    use doc_engine_scan::{ProjectScope, ProjectType, ScanConfig};

    let scope = match scope {
        "Medium" => ProjectScope::Medium,
        "Large" => ProjectScope::Large,
        _ => ProjectScope::Small,
    };

    let project_type = match project_type {
        "Internal" => Some(ProjectType::Internal),
        _ => Some(ProjectType::OpenSource),
    };

    let checks_vec: Option<Vec<u8>> = checks.map(|c| {
        c.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    });

    let phases_vec: Option<Vec<String>> = phases.map(|p| {
        p.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    });

    let module_vec: Option<Vec<String>> = module.map(|m| {
        m.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    });

    let config = ScanConfig {
        project_type,
        project_scope: scope,
        checks: checks_vec,
        rules_path: None,
        phases: phases_vec,
        module_filter: module_vec,
    };

    match doc_engine_scan::scan_with_config(root, &config) {
        Ok(report) => {
            let json = doc_engine_scan::format_report_json(&report);
            Ok(json)
        }
        Err(e) => Err(format!("{e}")),
    }
}

/// Run a struct-engine scan.
fn run_struct_scan(
    root: &std::path::Path,
    checks: Option<&str>,
) -> Result<String, String> {
    use struct_engine::ScanConfig;

    let checks_vec: Option<Vec<u8>> = checks.map(|c| {
        c.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    });

    let config = ScanConfig {
        project_kind: None,
        checks: checks_vec,
        rules_path: None,
    };

    match struct_engine::scan_with_config(root, &config) {
        Ok(report) => {
            let json = struct_engine::format_report_json(&report);
            Ok(json)
        }
        Err(e) => Err(format!("{e}")),
    }
}
