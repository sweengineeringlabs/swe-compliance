use rsc_ui::prelude::*;
use crate::features::scans::scans_type::{Scan, ScanRequest};
use crate::features::scans::scans_service;

/// Central reactive state store for the scans feature.
///
/// Signals:
///   active_scan        — The currently running scan (None when idle)
///   scan_history       — List of past scan records for the selected project
///   selected_project_id — The project whose scans are displayed
///   loading            — Whether an async operation is in flight
///   error              — Most recent error message (cleared on next action)
pub struct ScansStore {
    pub active_scan: Signal<Option<Scan>>,
    pub scan_history: Signal<Vec<Scan>>,
    pub selected_project_id: Signal<Option<String>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl ScansStore {
    /// Create a new ScansStore with default (empty) signal values.
    pub fn new() -> Self {
        Self {
            active_scan: signal(None),
            scan_history: signal(Vec::new()),
            selected_project_id: signal(None),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Trigger a new scan and update the store's active_scan on success.
/// On failure, sets the store error signal with the API error message.
pub fn trigger_scan(store: &ScansStore, request: ScanRequest) {
    store.loading.set(true);
    store.error.set(None);

    let active_scan = store.active_scan;
    let loading = store.loading;
    let error = store.error;
    let scan_history = store.scan_history;

    spawn(async move {
        match scans_service::create_scan(&request).await {
            Ok(scan) => {
                active_scan.set(Some(scan));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Load scan history for the given project and populate scan_history signal.
/// Clears any prior error on invocation; sets error on failure.
pub fn load_history(store: &ScansStore, project_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    let scan_history = store.scan_history;
    let loading = store.loading;
    let error = store.error;
    let project_id_owned = project_id.to_string();

    spawn(async move {
        match scans_service::list_project_scans(&project_id_owned).await {
            Ok(scans) => {
                scan_history.set(scans);
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Refresh the active scan by re-fetching its data from the API.
/// Used after WebSocket reports completion to get the full scan record with report.
pub fn refresh_active_scan(store: &ScansStore) {
    let active = store.active_scan.get();
    if let Some(scan) = active {
        let active_scan = store.active_scan;
        let error = store.error;
        let scan_id = scan.id.clone();

        spawn(async move {
            match scans_service::get_scan(&scan_id).await {
                Ok(refreshed) => {
                    active_scan.set(Some(refreshed));
                }
                Err(api_error) => {
                    error.set(Some(api_error.message));
                }
            }
        });
    }
}

/// Clear the active scan signal (e.g., when a scan completes and user dismisses it).
pub fn clear_active_scan(store: &ScansStore) {
    store.active_scan.set(None);
}

/// Clear the error signal.
pub fn clear_error(store: &ScansStore) {
    store.error.set(None);
}
