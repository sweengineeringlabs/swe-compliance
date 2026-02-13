use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::reports::types::ReportComparison;

/// Modal for comparing two scan reports side-by-side (FR-703).
///
/// Displays added findings (green), removed findings (red), and a count
/// of unchanged findings between the two selected scans.
#[component]
pub fn report_comparison_view(
    scan_a: Option<String>,
    scan_b: Option<String>,
    comparison: Option<ReportComparison>,
    loading: bool,
    on_scan_b_change: Option<Box<dyn Fn(String)>>,
    on_compare: Option<Box<dyn Fn()>>,
    on_close: Option<Box<dyn Fn()>>,
    open: bool,
) -> View {
    // Wrap on_close in Rc so it can be cloned and shared between multiple closures
    let on_close: Option<Rc<dyn Fn()>> = on_close.map(|f| Rc::from(f));
    view! {
        style {
            .report-comparison { display: flex; flex-direction: column; gap: var(--space-4); }
            .report-comparison__controls { display: flex; gap: var(--space-3); align-items: flex-end; }
            .report-comparison__results { display: flex; flex-direction: column; gap: var(--space-3); }
            .report-comparison__section { margin-bottom: var(--space-2); }
            .report-comparison__section-title { font-weight: 600; font-size: var(--font-size-sm); color: var(--color-text-secondary); margin-bottom: var(--space-1); }
            .report-comparison__list { display: flex; flex-direction: column; gap: var(--space-1); }
            .report-comparison__unchanged { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-2); }
        }
        (if open {
            view! {
                div(class="modal", data-testid="report-comparison") {
                    div(class="modal__overlay", on:click={
                        let on_close = on_close.clone();
                        move || { if let Some(ref cb) = on_close { cb() } }
                    })
                    div(class="modal__content") {
                        div(class="modal__header") {
                            h3 { "Compare Reports" }
                            button(class="modal__close", on:click=move || { if let Some(ref cb) = on_close { cb() } }) {
                                "Close"
                            }
                        }
                        div(class="report-comparison") {
                            div(class="report-comparison__controls") {
                                div(class="form-field") {
                                    label { "Compare scan" }
                                    select(
                                        value=scan_b.clone().unwrap_or_default(),
                                        on:change=move |v: String| { if let Some(ref cb) = on_scan_b_change { cb(v) } },
                                        data-testid="report-comparison-scan-b-select"
                                    ) {
                                        option(value="", disabled=true) { "Select a scan..." }
                                    }
                                }

                                button(
                                    class="btn btn--primary",
                                    disabled=loading || scan_a.is_none() || scan_b.is_none(),
                                    on:click=move || { if let Some(ref cb) = on_compare { cb() } },
                                    data-testid="report-compare-btn"
                                ) {
                                    "Compare"
                                }
                            }

                            (if loading {
                                view! {
                                    div(class="spinner", role="status", data-testid="report-comparison-spinner") {
                                        "Loading..."
                                    }
                                }
                            } else {
                                view! {}
                            })

                            (if let Some(ref result) = comparison {
                                let added_label = format!("Added ({})", result.added.len());
                                let removed_label = format!("Removed ({})", result.removed.len());
                                let unchanged_label = format!("Unchanged: {}", result.unchanged);
                                view! {
                                    div(class="report-comparison__results", data-testid="report-comparison-results") {
                                        (if !result.added.is_empty() {
                                            view! {
                                                div(class="report-comparison__section") {
                                                    p(class="report-comparison__section-title") {
                                                        (added_label)
                                                    }
                                                    div(class="report-comparison__list", data-testid="report-comparison-added") {
                                                        Indexed(
                                                            each=result.added.clone(),
                                                            key=|item| item.clone(),
                                                            view=|item| view! {
                                                                span(class="badge badge--success", data-testid="comparison-added-badge") {
                                                                    (item)
                                                                }
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            view! {}
                                        })

                                        (if !result.removed.is_empty() {
                                            view! {
                                                div(class="report-comparison__section") {
                                                    p(class="report-comparison__section-title") {
                                                        (removed_label)
                                                    }
                                                    div(class="report-comparison__list", data-testid="report-comparison-removed") {
                                                        Indexed(
                                                            each=result.removed.clone(),
                                                            key=|item| item.clone(),
                                                            view=|item| view! {
                                                                span(class="badge badge--danger", data-testid="comparison-removed-badge") {
                                                                    (item)
                                                                }
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            view! {}
                                        })

                                        p(class="report-comparison__unchanged", data-testid="report-comparison-unchanged") {
                                            (unchanged_label)
                                        }
                                    }
                                }
                            } else {
                                view! {}
                            })
                        }
                    }
                }
            }
        } else {
            view! {}
        })
    }
}
