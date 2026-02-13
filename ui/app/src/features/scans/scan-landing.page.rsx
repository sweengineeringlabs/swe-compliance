use rsc_ui::prelude::*;
use crate::features::scans::store::{self, ScansStore};
use crate::features::scans::types::ScanRequest;
use crate::features::scans::scan_trigger::ScanTrigger;
use crate::features::scans::scan_progress::ScanProgressBar;
use crate::features::scans::scan_history::ScanHistory;
use crate::features::scans::use_scans_ws::use_scan_progress;

/// Scans management page (FR-300..305).
component ScansLanding() {
    let s = use_context::<ScansStore>();

    // Load scan history on mount.
    { let s2 = s.clone(); effect(move || {
        if let Some(ref pid) = s2.selected_project_id.get() {
            store::load_history(&s2, pid);
        }
    }); }

    // Derive scan_id signal for the WebSocket hook.
    let scan_id_sig = { let active = s.active_scan.clone(); derived(move || active.get().map(|scan| scan.id.clone())) };
    let (scan_progress, _ws_state) = use_scan_progress(scan_id_sig);

    let is_running = { let active = s.active_scan.clone(); derived(move || active.get().is_some()) };

    style {
        .scans { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="scans" data-testid="scans-landing">
            <ScanTrigger
                on_trigger={Some(Box::new({ let s2 = s.clone(); move |engine: String, scope: String, checks: Option<String>| {
                    let req = ScanRequest {
                        project_id: s2.selected_project_id.get().unwrap_or_default(),
                        engine,
                        checks,
                        phase: Some(scope),
                        module: None,
                    };
                    store::trigger_scan(&s2, req);
                }}))}
                disabled={is_running.get()}
            />
            @if is_running.get() {
                <ScanProgressBar progress={scan_progress.clone()} />
            }
            <h3>"Scan History"</h3>
            <ScanHistory scans={s.scan_history.clone()} />
        </div>
    }
}
