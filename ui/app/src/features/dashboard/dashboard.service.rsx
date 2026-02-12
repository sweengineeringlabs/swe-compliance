use crate::util::api::{api_get, ApiError};
use crate::features::dashboard::dashboard_type::{
    DashboardProject, ComplianceSummary, TrendPoint, CategoryBreakdown,
};
use crate::features::dashboard::dashboard_store::DashboardStore;

/// Fetch the list of all projects with compliance summaries from the API.
/// GET /api/v1/projects
pub async fn fetch_projects() -> Result<Vec<DashboardProject>, ApiError> {
    let response = api_get("/projects").await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse projects response".into(),
    })?;

    let items = parsed.get_array("projects").unwrap_or_default();
    let mut projects = Vec::new();

    for item in items.iter() {
        let summary_obj = item.get("compliance_summary");
        let summary = match summary_obj {
            Some(s) => ComplianceSummary {
                passed: s.get_u32("passed").unwrap_or(0),
                failed: s.get_u32("failed").unwrap_or(0),
                skipped: s.get_u32("skipped").unwrap_or(0),
                total: s.get_u32("total").unwrap_or(0),
            },
            None => ComplianceSummary {
                passed: 0,
                failed: 0,
                skipped: 0,
                total: 0,
            },
        };

        projects.push(DashboardProject {
            id: item.get_str("id").unwrap_or_default().into(),
            name: item.get_str("name").unwrap_or_default().into(),
            scope: item.get_str("scope").unwrap_or_default().into(),
            project_type: item.get_str("project_type").unwrap_or_default().into(),
            compliance_summary: summary,
        });
    }

    Ok(projects)
}

/// Fetch trend data for a specific project since the given ISO timestamp.
/// GET /api/v1/projects/{id}/trends?since={since}
pub async fn fetch_trends(project_id: &str, since: &str) -> Result<Vec<TrendPoint>, ApiError> {
    let path = format!("/projects/{project_id}/trends?since={since}");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse trends response".into(),
    })?;

    let items = parsed.get_array("trends").unwrap_or_default();
    let mut points = Vec::new();

    for item in items.iter() {
        points.push(TrendPoint {
            scan_id: item.get_str("scan_id").unwrap_or_default().into(),
            timestamp: item.get_str("timestamp").unwrap_or_default().into(),
            passed: item.get_u32("passed").unwrap_or(0),
            failed: item.get_u32("failed").unwrap_or(0),
            skipped: item.get_u32("skipped").unwrap_or(0),
        });
    }

    Ok(points)
}

/// Fetch category-level compliance breakdown for a specific project.
/// GET /api/v1/projects/{id}/categories
pub async fn fetch_categories(project_id: &str) -> Result<Vec<CategoryBreakdown>, ApiError> {
    let path = format!("/projects/{project_id}/categories");
    let response = api_get(&path).await?;
    let parsed = json_parse(&response).ok_or_else(|| ApiError {
        code: "PARSE_ERROR".into(),
        message: "failed to parse categories response".into(),
    })?;

    let items = parsed.get_array("categories").unwrap_or_default();
    let mut categories = Vec::new();

    for item in items.iter() {
        categories.push(CategoryBreakdown {
            category: item.get_str("category").unwrap_or_default().into(),
            passed: item.get_u32("passed").unwrap_or(0),
            failed: item.get_u32("failed").unwrap_or(0),
            skipped: item.get_u32("skipped").unwrap_or(0),
        });
    }

    Ok(categories)
}

/// Load all dashboard data: projects list, and optionally trend/category data
/// for the first project. Updates the store signals directly.
pub async fn load_dashboard(store: &DashboardStore) {
    store.loading.set(true);
    store.error.set(None);

    match fetch_projects().await {
        Ok(projects) => {
            store.projects.set(projects.clone());

            // Auto-select the first project and load its details
            if let Some(first) = projects.first() {
                store.selected_project.set(Some(first.clone()));

                // Load trend data for the past 30 days
                let since = days_ago_iso(30);
                match fetch_trends(&first.id, &since).await {
                    Ok(trends) => store.trend_data.set(trends),
                    Err(e) => {
                        log_warn(&format!("failed to load trends: {}", e.message));
                    }
                }

                // Load category breakdown
                match fetch_categories(&first.id).await {
                    Ok(cats) => store.category_breakdown.set(cats),
                    Err(e) => {
                        log_warn(&format!("failed to load categories: {}", e.message));
                    }
                }
            }

            store.loading.set(false);
        }
        Err(e) => {
            store.error.set(Some(e.message.clone()));
            store.loading.set(false);
        }
    }
}

/// Reload trend and category data when the selected project changes.
pub async fn load_project_details(store: &DashboardStore, project_id: &str) {
    store.loading.set(true);

    let since = days_ago_iso(30);

    let trend_result = fetch_trends(project_id, &since).await;
    let category_result = fetch_categories(project_id).await;

    match trend_result {
        Ok(trends) => store.trend_data.set(trends),
        Err(e) => log_warn(&format!("failed to load trends: {}", e.message)),
    }

    match category_result {
        Ok(cats) => store.category_breakdown.set(cats),
        Err(e) => log_warn(&format!("failed to load categories: {}", e.message)),
    }

    store.loading.set(false);
}

/// Helper: return an ISO date string for N days ago.
fn days_ago_iso(days: u32) -> String {
    let now = js_date_now();
    let ms_per_day = 86_400_000.0;
    let past = now - (days as f64 * ms_per_day);
    js_date_to_iso(past)
}
