use rsc_ui::prelude::*;
use crate::features::scans::scans_store as store;
use crate::features::scans::scan_trigger::ScanTrigger;
use crate::features::scans::scan_progress::ScanProgressBar;
use crate::features::scans::scan_history::ScanHistory;

/// Scans management page (FR-300..305).
component ScansLanding() {
    effect(|| { store::load_history(); });
    let is_running = derived(move || store::active_scan.get().is_some());

    style {
        .scans { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="scans" data-testid="scans-landing">
            <ScanTrigger
                on_trigger={|engine, scope, checks| store::trigger_scan(engine, scope, checks)}
                disabled={is_running.get()}
            />
            @if is_running.get() {
                <ScanProgressBar progress={store::scan_progress.clone()} />
            }
            <h3>"Scan History"</h3>
            <ScanHistory scans={store::scan_history.clone()} />
        </div>
    }
}
