use rsc_compat::prelude::*;
use crate::features::dashboard::types::{
    DashboardProject, TrendPoint, CategoryBreakdown,
};

/// Central reactive store for the dashboard feature.
/// All dashboard components read from these signals to ensure consistency.
#[derive(Clone)]
pub struct DashboardStore {
    /// List of all projects with their latest compliance summaries.
    pub projects: Signal<Vec<DashboardProject>>,

    /// Currently selected project for detail views and trend data.
    pub selected_project: Signal<Option<DashboardProject>>,

    /// Trend data points for the selected project over time.
    pub trend_data: Signal<Vec<TrendPoint>>,

    /// Category-level breakdown for the selected project's latest scan.
    pub category_breakdown: Signal<Vec<CategoryBreakdown>>,

    /// Whether any async operation is in progress.
    pub loading: Signal<bool>,

    /// Last error message from a failed operation, if any.
    pub error: Signal<Option<String>>,
}

impl DashboardStore {
    /// Creates a new store with default empty/idle state.
    pub fn new() -> Self {
        Self {
            projects: signal(Vec::new()),
            selected_project: signal(None),
            trend_data: signal(Vec::new()),
            category_breakdown: signal(Vec::new()),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived signal: total compliance percentage for the currently selected project.
/// Returns None when no project is selected.
pub fn total_compliance(store: &DashboardStore) -> Signal<Option<f64>> {
    let selected = store.selected_project.clone();
    derived(move || {
        selected.get().as_ref().map(|project| {
            project.compliance_summary.percentage()
        })
    })
}

/// Derived signal: summary counts across all loaded projects.
/// Returns (total_passed, total_failed, total_skipped, total_checks).
pub fn aggregate_compliance(store: &DashboardStore) -> Signal<(u32, u32, u32, u32)> {
    let projects = store.projects.clone();
    derived(move || {
        let list = projects.get();
        let mut passed = 0u32;
        let mut failed = 0u32;
        let mut skipped = 0u32;
        let mut total = 0u32;
        for project in list.iter() {
            passed += project.compliance_summary.passed;
            failed += project.compliance_summary.failed;
            skipped += project.compliance_summary.skipped;
            total += project.compliance_summary.total;
        }
        (passed, failed, skipped, total)
    })
}

/// Derived signal: the most recent trend point's compliance percentage.
/// Returns None when trend data is empty.
pub fn latest_trend_percentage(store: &DashboardStore) -> Signal<Option<f64>> {
    let trend = store.trend_data.clone();
    derived(move || {
        trend.get().last().map(|point| point.percentage())
    })
}

/// Derived signal: number of projects currently loaded.
pub fn project_count(store: &DashboardStore) -> Signal<usize> {
    let projects = store.projects.clone();
    derived(move || projects.get().len())
}

/// Select a project by its ID, updating the store's selected_project signal.
/// Returns true if the project was found and selected.
pub fn select_project(store: &DashboardStore, project_id: &str) -> bool {
    let projects = store.projects.get();
    if let Some(project) = projects.iter().find(|p| p.id == project_id) {
        store.selected_project.set(Some(project.clone()));
        true
    } else {
        false
    }
}

/// Clear the current selection and associated detail data.
pub fn clear_selection(store: &DashboardStore) {
    store.selected_project.set(None);
    store.trend_data.set(Vec::new());
    store.category_breakdown.set(Vec::new());
}
