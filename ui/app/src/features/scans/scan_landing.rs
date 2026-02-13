use rsc_compat::prelude::*;
use super::store::{self, ScansStore};
use super::types::ScanRequest;
use super::scan_trigger::scan_trigger;
use super::scan_progress::scan_progress_bar;
use super::scan_history::scan_history;
use super::use_scans_ws::use_scan_progress;

/// Scans management page (FR-300..305).
#[component]
pub fn scans_landing() -> View {
    let s = use_context::<ScansStore>();

    // Load scan history on mount (using empty project_id as placeholder).
    {
        let s2 = s.clone();
        effect(move || {
            if let Some(ref pid) = s2.selected_project_id.get() {
                store::load_history(&s2, pid);
            }
        });
    }

    // Derive scan_id signal for the WebSocket hook.
    let scan_id_sig = {
        let active = s.active_scan.clone();
        derived(move || active.get().map(|scan| scan.id.clone()))
    };
    let (scan_progress, _) = use_scan_progress(scan_id_sig);

    let is_running = {
        let active = s.active_scan.clone();
        derived(move || active.get().is_some())
    };

    view! {
        style {
            .scans { display: flex; flex-direction: column; gap: var(--space-4); }
        }
        div(class="scans", data-testid="scans-landing") {
            (scan_trigger(
                Some(Box::new({
                    let s2 = s.clone();
                    move |engine: String, scope: String, checks: Option<String>| {
                        let req = ScanRequest {
                            project_id: s2.selected_project_id.get().unwrap_or_default(),
                            engine,
                            checks,
                            phase: Some(scope),
                            module: None,
                        };
                        store::trigger_scan(&s2, req);
                    }
                })),
                is_running.get(),
            ))
            (if is_running.get() {
                scan_progress_bar(scan_progress.clone())
            } else {
                view! {}
            })
            h3 { "Scan History" }
            (scan_history(s.scan_history.clone()))
        }
    }
}
