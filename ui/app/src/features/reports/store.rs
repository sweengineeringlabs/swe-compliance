use rsc_compat::prelude::*;
use crate::features::reports::types::{ReportData, ReportComparison};
use crate::features::reports::service;

/// Central reactive state store for the reports feature.
///
/// Signals:
///   selected_scan_id   -- The scan whose report is being generated
///   selected_format    -- The chosen export format (default "json")
///   report_data        -- The most recently generated report (None when empty)
///   comparison         -- The most recent comparison result (None when empty)
///   compare_scan_id    -- The second scan ID for comparison
///   loading            -- Whether an async operation is in flight
///   error              -- Most recent error message (cleared on next action)
#[derive(Clone)]
pub struct ReportsStore {
    pub selected_scan_id: Signal<Option<String>>,
    pub selected_format: Signal<String>,
    pub report_data: Signal<Option<ReportData>>,
    pub comparison: Signal<Option<ReportComparison>>,
    pub compare_scan_id: Signal<Option<String>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl ReportsStore {
    /// Create a new ReportsStore with default (empty) signal values.
    pub fn new() -> Self {
        Self {
            selected_scan_id: signal(None),
            selected_format: signal("json".into()),
            report_data: signal(None),
            comparison: signal(None),
            compare_scan_id: signal(None),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived signal: true when a report has been generated.
pub fn has_report(store: &ReportsStore) -> bool {
    let report_data = store.report_data.clone();
    derived(move || report_data.get().is_some()).get()
}

/// Derived signal: true when both scan IDs are set for comparison.
pub fn can_compare(store: &ReportsStore) -> bool {
    let selected_scan_id = store.selected_scan_id.clone();
    let compare_scan_id = store.compare_scan_id.clone();
    derived(move || {
        selected_scan_id.get().is_some() && compare_scan_id.get().is_some()
    }).get()
}

/// Export a report for the selected scan in the chosen format (FR-700, FR-701, FR-702).
/// On failure, sets the store error signal with the API error message.
pub fn export(store: &ReportsStore) {
    let scan_id = match store.selected_scan_id.get() {
        Some(id) => id.clone(),
        None => {
            store.error.set(Some("No scan selected".into()));
            return;
        }
    };
    let format = store.selected_format.get().clone();

    store.loading.set(true);
    store.error.set(None);

    let report_data = store.report_data.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();

    spawn(async move {
        match service::export_report(&scan_id, &format).await {
            Ok(report) => {
                report_data.set(Some(report));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Fetch an ISO 15289 audit report for the selected scan (FR-704).
/// On failure, sets the store error signal with the API error message.
pub fn export_audit(store: &ReportsStore) {
    let scan_id = match store.selected_scan_id.get() {
        Some(id) => id.clone(),
        None => {
            store.error.set(Some("No scan selected".into()));
            return;
        }
    };

    store.loading.set(true);
    store.error.set(None);

    let report_data = store.report_data.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();

    spawn(async move {
        match service::get_audit_report(&scan_id).await {
            Ok(report) => {
                report_data.set(Some(report));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Compare two scan reports (FR-703).
/// Requires both selected_scan_id and compare_scan_id to be set.
pub fn compare(store: &ReportsStore) {
    let scan_a = match store.selected_scan_id.get() {
        Some(id) => id.clone(),
        None => {
            store.error.set(Some("No primary scan selected".into()));
            return;
        }
    };
    let scan_b = match store.compare_scan_id.get() {
        Some(id) => id.clone(),
        None => {
            store.error.set(Some("No comparison scan selected".into()));
            return;
        }
    };

    store.loading.set(true);
    store.error.set(None);

    let comparison = store.comparison.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();

    spawn(async move {
        match service::compare_reports(&scan_a, &scan_b).await {
            Ok(result) => {
                comparison.set(Some(result));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Clear the report data signal.
pub fn clear_report(store: &ReportsStore) {
    store.report_data.set(None);
}

/// Clear the comparison signal.
pub fn clear_comparison(store: &ReportsStore) {
    store.comparison.set(None);
}

/// Clear the error signal.
pub fn clear_error(store: &ReportsStore) {
    store.error.set(None);
}
