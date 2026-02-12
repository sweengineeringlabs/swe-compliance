use rsc_ui::prelude::*;
use crate::features::scans::scans_type::ScanProgress;

/// Real-time scan progress bar updated via WebSocket (FR-302).
component ScanProgressBar(progress: Signal<Option<ScanProgress>>) {
    let pct = derived(move || {
        progress.get().map(|p| {
            if p.total > 0 { (p.current as f64 / p.total as f64) * 100.0 } else { 0.0 }
        }).unwrap_or(0.0)
    });

    style {
        .scan-progress { padding: var(--space-4); }
        .scan-progress__label { font-size: var(--font-size-sm); color: var(--color-text-secondary); margin-bottom: var(--space-2); }
    }

    render {
        <Card class="scan-progress" data-testid="scan-progress">
            @if let Some(p) = progress.get() {
                <div class="scan-progress__label" data-testid="scan-progress-label">
                    {format!("Check {}/{}: {}", p.current, p.total, p.check_description)}
                </div>
                <Progress value={pct.get()} data-testid="scan-progress-bar" />
            } @else {
                <div class="scan-progress__label">"Waiting for scan progress..."</div>
                <Progress indeterminate={true} data-testid="scan-progress-indeterminate" />
            }
        </Card>
    }
}
