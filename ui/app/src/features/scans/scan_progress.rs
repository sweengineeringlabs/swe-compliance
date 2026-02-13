use rsc_compat::prelude::*;
use super::types::ScanProgress;

/// Real-time scan progress bar updated via WebSocket (FR-302).
#[component]
pub fn scan_progress_bar(progress: Signal<Option<ScanProgress>>) -> View {
    let progress_derived = progress.clone();
    let pct = derived(move || {
        progress_derived.get().map(|p| {
            if p.total > 0 { (p.current as f64 / p.total as f64) * 100.0 } else { 0.0 }
        }).unwrap_or(0.0)
    });

    view! {
        style {
            .scan-progress { padding: var(--space-4); }
            .scan-progress__label { font-size: var(--font-size-sm); color: var(--color-text-secondary); margin-bottom: var(--space-2); }
        }
        Card(class="scan-progress", data-testid="scan-progress") {
            (if let Some(p) = progress.get() {
                view! {
                    div(class="scan-progress__label", data-testid="scan-progress-label") {
                        (format!("Check {}/{}: {}", p.current, p.total, p.check_description))
                    }
                    Progress(value=pct.get(), data-testid="scan-progress-bar")
                }
            } else {
                view! {
                    div(class="scan-progress__label") { "Waiting for scan progress..." }
                    Progress(indeterminate=true, data-testid="scan-progress-indeterminate")
                }
            })
        }
    }
}
