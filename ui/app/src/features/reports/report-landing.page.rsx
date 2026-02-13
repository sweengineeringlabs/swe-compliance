use rsc_ui::prelude::*;
use crate::features::reports::reports_store as store;
use crate::features::reports::report_export::ReportExport;
use crate::features::reports::report_comparison::ReportComparisonView;

/// Reports management page (FR-700..704).
component ReportsLanding() {
    let show_comparison = signal(false);

    style {
        .reports { display: flex; flex-direction: column; gap: var(--space-4); }
        .reports__actions { display: flex; gap: var(--space-3); }
    }

    render {
        <div class="reports" data-testid="reports-landing">
            <ReportExport
                scan_id={store::selected_scan_id.get()}
                format={store::selected_format.get().clone()}
                report={store::report_data.get()}
                loading={store::loading.get()}
                on_format_change={|v| store::selected_format.set(v)}
                on_export={|| store::export()}
            />

            <div class="reports__actions">
                <Button
                    label="Compare Reports"
                    variant="secondary"
                    on:click={|| show_comparison.set(true)}
                    data-testid="reports-open-comparison-btn"
                />
                <Button
                    label="Audit Report (ISO 15289)"
                    variant="secondary"
                    disabled={store::selected_scan_id.get().is_none() || store::loading.get()}
                    on:click={|| store::export_audit()}
                    data-testid="reports-audit-btn"
                />
            </div>

            <ReportComparisonView
                scan_a={store::selected_scan_id.get()}
                scan_b={store::compare_scan_id.get()}
                comparison={store::comparison.get()}
                loading={store::loading.get()}
                on_scan_b_change={|v| store::compare_scan_id.set(Some(v))}
                on_compare={|| store::compare()}
                on_close={|| show_comparison.set(false)}
                open={show_comparison.get()}
            />

            @if let Some(ref err) = store::error.get() {
                <Toast variant="danger" on:dismiss={|| store::clear_error()} data-testid="reports-error-toast">
                    {err}
                </Toast>
            }
        </div>
    }
}
