use rsc_ui::prelude::*;
use crate::features::reports::reports_type::ReportComparison;

/// Modal for comparing two scan reports side-by-side (FR-703).
///
/// Displays added findings (green), removed findings (red), and a count
/// of unchanged findings between the two selected scans.
component ReportComparisonView(
    scan_a: Option<String>,
    scan_b: Option<String>,
    comparison: Option<ReportComparison>,
    loading: bool,
    on_scan_b_change: Fn(String),
    on_compare: Fn(),
    on_close: Fn(),
    open: bool,
) {
    style {
        .report-comparison { display: flex; flex-direction: column; gap: var(--space-4); }
        .report-comparison__controls { display: flex; gap: var(--space-3); align-items: flex-end; }
        .report-comparison__results { display: flex; flex-direction: column; gap: var(--space-3); }
        .report-comparison__section { margin-bottom: var(--space-2); }
        .report-comparison__section-title { font-weight: 600; font-size: var(--font-size-sm); color: var(--color-text-secondary); margin-bottom: var(--space-1); }
        .report-comparison__list { display: flex; flex-direction: column; gap: var(--space-1); }
        .report-comparison__unchanged { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-2); }
    }

    render {
        <Modal open={open} on:close={|| on_close()} title="Compare Reports" data-testid="report-comparison">
            <div class="report-comparison">
                <div class="report-comparison__controls">
                    <FormField label="Compare scan">
                        <Select
                            value={scan_b.clone().unwrap_or_default()}
                            on:change={|v| on_scan_b_change(v)}
                            data-testid="report-comparison-scan-b-select"
                        >
                            <option value="" disabled>"Select a scan..."</option>
                        </Select>
                    </FormField>

                    <Button
                        label="Compare"
                        variant="primary"
                        disabled={loading || scan_a.is_none() || scan_b.is_none()}
                        on:click={|| on_compare()}
                        data-testid="report-compare-btn"
                    />
                </div>

                @if loading {
                    <Spinner data-testid="report-comparison-spinner" />
                }

                @if let Some(ref result) = comparison {
                    <div class="report-comparison__results" data-testid="report-comparison-results">
                        @if !result.added.is_empty() {
                            <div class="report-comparison__section">
                                <p class="report-comparison__section-title">
                                    {format!("Added ({})", result.added.len())}
                                </p>
                                <div class="report-comparison__list" data-testid="report-comparison-added">
                                    @for item in &result.added {
                                        <Badge variant="success" data-testid="comparison-added-badge">{item}</Badge>
                                    }
                                </div>
                            </div>
                        }

                        @if !result.removed.is_empty() {
                            <div class="report-comparison__section">
                                <p class="report-comparison__section-title">
                                    {format!("Removed ({})", result.removed.len())}
                                </p>
                                <div class="report-comparison__list" data-testid="report-comparison-removed">
                                    @for item in &result.removed {
                                        <Badge variant="danger" data-testid="comparison-removed-badge">{item}</Badge>
                                    }
                                </div>
                            </div>
                        }

                        <p class="report-comparison__unchanged" data-testid="report-comparison-unchanged">
                            {format!("Unchanged: {}", result.unchanged)}
                        </p>
                    </div>
                }
            </div>
        </Modal>
    }
}
