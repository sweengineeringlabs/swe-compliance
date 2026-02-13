use rsc_ui::prelude::*;
use crate::features::reports::store::{self, ReportsStore};
use crate::features::reports::report_export::ReportExport;
use crate::features::reports::report_comparison::ReportComparisonView;

/// Reports management page (FR-700..704).
component ReportsLanding() {
    let s = use_context::<ReportsStore>();
    let show_comparison = signal(false);

    style {
        .reports { display: flex; flex-direction: column; gap: var(--space-4); }
        .reports__actions { display: flex; gap: var(--space-3); }
    }

    render {
        <div class="reports" data-testid="reports-landing">
            <ReportExport
                scan_id={s.selected_scan_id.get()}
                format={s.selected_format.get().clone()}
                report={s.report_data.get()}
                loading={s.loading.get()}
                on_format_change={Some(Box::new({ let fmt = s.selected_format.clone(); move |v: String| fmt.set(v) }))}
                on_export={Some(Box::new({ let s2 = s.clone(); move || store::export(&s2) }))}
            />

            <div class="reports__actions">
                <Button
                    label="Compare Reports"
                    variant="secondary"
                    on:click={move || show_comparison.set(true)}
                    data-testid="reports-open-comparison-btn"
                />
                <Button
                    label="Audit Report (ISO 15289)"
                    variant="secondary"
                    disabled={s.selected_scan_id.get().is_none() || s.loading.get()}
                    on:click={{ let s2 = s.clone(); move || store::export_audit(&s2) }}
                    data-testid="reports-audit-btn"
                />
            </div>

            <ReportComparisonView
                scan_a={s.selected_scan_id.get()}
                scan_b={s.compare_scan_id.get()}
                comparison={s.comparison.get()}
                loading={s.loading.get()}
                on_scan_b_change={Some(Box::new({ let cmp = s.compare_scan_id.clone(); move |v: String| cmp.set(if v.is_empty() { None } else { Some(v) }) }))}
                on_compare={Some(Box::new({ let s2 = s.clone(); move || store::compare(&s2) }))}
                on_close={Some(Box::new(move || show_comparison.set(false)))}
                open={show_comparison.get()}
            />

            @if let Some(ref err) = s.error.get().as_ref() {
                <Toast variant="danger" on:dismiss={{ let s2 = s.clone(); move || store::clear_error(&s2) }} data-testid="reports-error-toast">
                    {err.as_str()}
                </Toast>
            }
        </div>
    }
}
