pub mod health;
pub mod projects;
pub mod scans;
pub mod violations;
pub mod scaffold;
pub mod templates;
pub mod reports;
pub mod ai;
pub mod editor;
pub mod specs;

use axum::middleware;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;

use crate::auth::JwtSecret;
use crate::middleware::{rate_limit_middleware, RateLimiter, ScanSemaphore};
use crate::db::Db;
use crate::ws::WsBroadcaster;
use crate::config::ServerConfig;

/// Shared application state passed to all handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Db,
    pub config: ServerConfig,
    pub ws_broadcaster: WsBroadcaster,
    pub scan_semaphore: ScanSemaphore,
}

/// Build the complete router with all API routes.
pub fn build_router(state: AppState) -> Router {
    let jwt_secret = JwtSecret(state.config.jwt_secret.clone());
    let rate_limiter = RateLimiter::new(state.config.rate_limit_per_min);

    // Public routes (no auth required)
    let public = Router::new()
        .route("/health", get(health::health_check))
        .route("/api/v1/auth/login", post(health::login));

    // Protected API routes
    let api = Router::new()
        // Projects
        .route("/api/v1/projects", post(projects::create_project))
        .route("/api/v1/projects", get(projects::list_projects))
        .route("/api/v1/projects/{id}", get(projects::get_project))
        .route("/api/v1/projects/{id}", patch(projects::update_project))
        .route("/api/v1/projects/{id}", delete(projects::delete_project))
        // Scans
        .route("/api/v1/scans", post(scans::create_scan))
        .route("/api/v1/scans/{id}", get(scans::get_scan))
        .route("/api/v1/scans/{id}/progress", get(scans::scan_progress_ws))
        .route("/api/v1/projects/{id}/scans", get(scans::list_project_scans))
        .route("/api/v1/projects/{id}/trends", get(scans::get_trends))
        // Violations
        .route("/api/v1/scans/{id}/violations", get(violations::get_violations))
        // Reports
        .route("/api/v1/scans/{id}/report", get(reports::get_report))
        .route("/api/v1/scans/{id}/audit-report", get(reports::get_audit_report))
        // Scaffold
        .route("/api/v1/scaffold/parse", post(scaffold::parse_srs))
        .route("/api/v1/scaffold/execute", post(scaffold::execute_scaffold))
        // Templates
        .route("/api/v1/templates", get(templates::list_templates))
        .route("/api/v1/templates/{name}/copy", post(templates::copy_template))
        // AI
        .route("/api/v1/ai/status", get(ai::ai_status))
        .route("/api/v1/ai/chat", post(ai::ai_chat))
        .route("/api/v1/ai/chat/stream", get(ai::ai_chat_stream_ws))
        .route("/api/v1/ai/audit", post(ai::ai_audit))
        .route("/api/v1/ai/generate-commands", post(ai::ai_generate_commands))
        // Editor
        .route("/api/v1/editor/validate", post(editor::validate_srs))
        .route("/api/v1/projects/{id}/srs", get(editor::get_srs))
        .route("/api/v1/projects/{id}/srs", put(editor::save_srs))
        // Specs
        .route("/api/v1/projects/{id}/specs", get(specs::get_specs))
        .layer(middleware::from_fn(rate_limit_middleware));

    Router::new()
        .merge(public)
        .merge(api)
        .layer(axum::Extension(jwt_secret))
        .layer(axum::Extension(rate_limiter))
        .with_state(state)
}
