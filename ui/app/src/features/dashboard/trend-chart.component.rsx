use rsc_ui::prelude::*;
use crate::features::dashboard::dashboard_type::TrendPoint;

/// Line chart visualization of compliance trends over time (FR-202).
component TrendChart(trend_data: Signal<Vec<TrendPoint>>) {
    let has_data = derived(move || !trend_data.get().is_empty());

    style {
        .trend-chart { width: 100%; }
        .trend-chart__title { font-size: var(--font-size-md); font-weight: 600; margin-bottom: var(--space-4); }
        .trend-chart__empty { padding: var(--space-8); text-align: center; color: var(--color-text-muted); }
        .trend-chart__row { font-variant-numeric: tabular-nums; }
    }

    render {
        <Card data-testid="trend-chart">
            <h3 class="trend-chart__title">"Compliance Trend"</h3>
            @if !has_data.get() {
                <div class="trend-chart__empty" data-testid="trend-empty">
                    "No trend data available. Run multiple scans to see trends."
                </div>
            } @else {
                <Table data-testid="trend-table">
                    <thead>
                        <tr>
                            <th>"Date"</th><th>"Passed"</th><th>"Failed"</th><th>"Skipped"</th><th>"Rate"</th>
                        </tr>
                    </thead>
                    <tbody>
                        @for point in trend_data.get().iter() {
                            <tr class="trend-chart__row" data-testid={format!("trend-row-{}", point.scan_id)}>
                                <td data-testid="trend-timestamp">{&point.timestamp}</td>
                                <td data-testid="trend-passed">{point.passed}</td>
                                <td data-testid="trend-failed">{point.failed}</td>
                                <td data-testid="trend-skipped">{point.skipped}</td>
                                <td data-testid="trend-rate">
                                    {
                                        let total = point.passed + point.failed + point.skipped;
                                        if total > 0 {
                                            format!("{:.1}%", (point.passed as f64 / total as f64) * 100.0)
                                        } else {
                                            "--".into()
                                        }
                                    }
                                </td>
                            </tr>
                        }
                    </tbody>
                </Table>
            }
        </Card>
    }
}
