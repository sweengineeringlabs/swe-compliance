use rsc_compat::prelude::*;
use crate::features::dashboard::types::TrendPoint;

/// Compute compliance rate string for a trend point.
fn trend_rate_str(passed: u32, failed: u32, skipped: u32) -> String {
    let total = passed + failed + skipped;
    if total > 0 {
        let pct = (passed as f64 / total as f64) * 100.0;
        format!("{:.1}%", pct)
    } else {
        "--".into()
    }
}

/// Line chart visualization of compliance trends over time (FR-202).
#[component]
pub fn trend_chart(trend_data: Signal<Vec<TrendPoint>>) -> View {
    let trend_data_check = trend_data.clone();
    let has_data = derived(move || !trend_data_check.get().is_empty());

    view! {
        style {
            .trend-chart { width: 100%; }
            .trend-chart__title { font-size: var(--font-size-md); font-weight: 600; margin-bottom: var(--space-4); }
            .trend-chart__empty { padding: var(--space-8); text-align: center; color: var(--color-text-muted); }
            .trend-chart__row { font-variant-numeric: tabular-nums; }
        }

        <Card data-testid="trend-chart">
            <h3 class="trend-chart__title">"Compliance Trend"</h3>
            if !has_data.get() {
                <div class="trend-chart__empty" data-testid="trend-empty">
                    "No trend data available. Run multiple scans to see trends."
                </div>
            } else {
                <Table data-testid="trend-table">
                    <thead>
                        <tr>
                            <th>"Date"</th><th>"Passed"</th><th>"Failed"</th><th>"Skipped"</th><th>"Rate"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            let rows: Vec<_> = trend_data.get().iter().map(|point| {
                                let rate_str = trend_rate_str(point.passed, point.failed, point.skipped);
                                let passed_str = format!("{}", point.passed);
                                let failed_str = format!("{}", point.failed);
                                let skipped_str = format!("{}", point.skipped);
                                let row_testid = format!("trend-row-{}", point.scan_id);
                                view! {
                                    <tr class="trend-chart__row" data-testid={row_testid}>
                                        <td data-testid="trend-timestamp">{&point.timestamp}</td>
                                        <td data-testid="trend-passed">{passed_str}</td>
                                        <td data-testid="trend-failed">{failed_str}</td>
                                        <td data-testid="trend-skipped">{skipped_str}</td>
                                        <td data-testid="trend-rate">
                                            {rate_str}
                                        </td>
                                    </tr>
                                }
                            }).collect();
                            rows
                        }
                    </tbody>
                </Table>
            }
        </Card>
    }
}
