use rsc_compat::prelude::*;
use crate::features::reports::store::{self, ReportsStore};
use super::report_export::report_export;
use super::report_comparison::report_comparison_view;

/// Reports management page (FR-700..704).
#[component]
pub fn reports_landing() -> View {
    let s = use_context::<ReportsStore>();
    let show_comparison = signal(false);

    let format_change_cb: Option<Box<dyn Fn(String)>> = Some(Box::new({
        let s = s.clone();
        move |_v: String| s.selected_format.set(_v)
    }));
    let export_cb: Option<Box<dyn Fn()>> = Some(Box::new({
        let s = s.clone();
        move || store::export(&s)
    }));
    let scan_b_cb: Option<Box<dyn Fn(String)>> = Some(Box::new({
        let s = s.clone();
        move |_v: String| s.compare_scan_id.set(Some(_v))
    }));
    let compare_cb: Option<Box<dyn Fn()>> = Some(Box::new({
        let s = s.clone();
        move || store::compare(&s)
    }));
    let close_cb: Option<Box<dyn Fn()>> = Some(Box::new({
        let sc = show_comparison.clone();
        move || sc.set(false)
    }));

    let err_opt = s.error.get().clone();
    let error_view = if let Some(ref err_msg) = err_opt {
        let s3 = s.clone();
        let msg = err_msg.clone();
        view! {
            div(class="toast toast--danger", role="alert", data-testid="reports-error-toast") {
                span { (msg) }
                button(
                    class="toast__dismiss",
                    on:click={
                        let s4 = s3.clone();
                        move || store::clear_error(&s4)
                    }
                ) { "Dismiss" }
            }
        }
    } else {
        view! {}
    };

    view! {
        style {
            .reports { display: flex; flex-direction: column; gap: var(--space-4); }
            .reports__actions { display: flex; gap: var(--space-3); }
        }
        div(class="reports", data-testid="reports-landing") {
            (report_export(
                s.selected_scan_id.get(),
                s.selected_format.get().clone(),
                s.report_data.get(),
                s.loading.get(),
                format_change_cb,
                export_cb,
            ))

            div(class="reports__actions") {
                button(
                    class="btn btn--secondary",
                    on:click={
                        let sc = show_comparison.clone();
                        move || sc.set(true)
                    },
                    data-testid="reports-open-comparison-btn"
                ) {
                    "Compare Reports"
                }
                button(
                    class="btn btn--secondary",
                    disabled=s.selected_scan_id.get().is_none() || s.loading.get(),
                    on:click={
                        let s = s.clone();
                        move || store::export_audit(&s)
                    },
                    data-testid="reports-audit-btn"
                ) {
                    "Audit Report (ISO 15289)"
                }
            }

            (report_comparison_view(
                s.selected_scan_id.get(),
                s.compare_scan_id.get(),
                s.comparison.get(),
                s.loading.get(),
                scan_b_cb,
                compare_cb,
                close_cb,
                show_comparison.get(),
            ))

            (error_view)
        }
    }
}
