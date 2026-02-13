use std::rc::Rc;
use rsc_compat::prelude::*;
use super::types::AuditResult;

/// Audit results view showing summary, scan details, and recommendations
/// (FR-802, FR-803).
///
/// Structure:
///   - Summary card at the top with the high-level finding
///   - Expandable section for raw scan results (rendered as JSON in a code div)
///   - Numbered list of actionable recommendations with badge indices
#[component]
pub fn audit_view(
    result: Signal<Option<AuditResult>>,
    loading: bool,
    on_run: Option<Box<dyn Fn(String, String)>>,
) -> View {
    let on_run: Option<Rc<dyn Fn(String, String)>> = on_run.map(|f| Rc::from(f));
    let audit_path = signal(String::new());
    let audit_scope = signal("full".to_string());

    view! {
        style {
            .audit-view {
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            }

            .audit-view__trigger {
                display: flex;
                gap: var(--space-3);
                align-items: flex-end;
            }

            .audit-view__summary {
                padding: var(--space-3);
                font-size: var(--font-size-md);
                color: var(--color-text);
                line-height: 1.6;
            }

            .audit-view__recommendations {
                display: flex;
                flex-direction: column;
                gap: var(--space-2);
            }

            .audit-view__rec-item {
                display: flex;
                align-items: baseline;
                gap: var(--space-2);
                padding: var(--space-2) 0;
            }

            .audit-view__empty {
                color: var(--color-text-secondary);
                font-style: italic;
                padding: var(--space-4);
                text-align: center;
            }

            .audit-view__details-toggle {
                cursor: pointer;
                font-weight: 600;
                padding: var(--space-2);
                background: var(--color-surface-raised);
                border: 1px solid var(--color-border);
                border-radius: var(--radius-md);
            }

            .audit-view__details-content {
                padding: var(--space-3);
                border: 1px solid var(--color-border);
                border-top: none;
                border-radius: 0 0 var(--radius-md) var(--radius-md);
            }
        }
        div(class="audit-view", data-testid="audit-view") {
            div(class="audit-view__trigger") {
                div(class="form-field") {
                    label { "Path" }
                    input(
                        type="text",
                        class="input",
                        value=audit_path.clone(),
                        on:input={let ap = audit_path.clone(); move |v: String| ap.set(v)},
                        placeholder="e.g. /src or .",
                        data-testid="audit-path-input",
                    )
                }
                div(class="form-field") {
                    label { "Scope" }
                    select(
                        value=audit_scope.clone(),
                        on:change={let asc = audit_scope.clone(); move |v: String| asc.set(v)},
                        data-testid="audit-scope-select",
                    ) {
                        option(value="full") { "Full" }
                        option(value="incremental") { "Incremental" }
                    }
                }
                button(
                    class="btn btn--primary",
                    disabled=loading || audit_path.get().is_empty(),
                    on:click={
                        let audit_path = audit_path.clone();
                        let audit_scope = audit_scope.clone();
                        move || { if let Some(ref cb) = on_run { cb(audit_path.get().clone(), audit_scope.get().clone()) } }
                    },
                    data-testid="audit-run-btn",
                ) {
                    "Run Audit"
                }
            }

            (if loading {
                view! {
                    div(class="audit-view__empty", data-testid="audit-loading") {
                        "Running audit..."
                    }
                }
            } else {
                view! {}
            })

            (match result.get() {
                Some(audit) => {
                    let summary_text = audit.summary.clone();
                    let scan_json = serde_json::to_string_pretty(&audit.scan_results)
                        .unwrap_or_default();
                    let recs = audit.recommendations.clone();
                    let recs_empty = recs.is_empty();
                    view! {
                        div(class="card", data-testid="audit-summary") {
                            div(class="audit-view__summary") {
                                strong { "Summary: " }
                                (summary_text)
                            }
                        }

                        div(data-testid="audit-scan-details") {
                            div(class="audit-view__details-toggle") {
                                "Scan Results"
                            }
                            div(class="audit-view__details-content") {
                                div(class="code-block", data-language="json", data-testid="audit-scan-json") {
                                    pre {
                                        code {
                                            (scan_json)
                                        }
                                    }
                                }
                            }
                        }

                        div(class="card", data-testid="audit-recommendations") {
                            h4 { "Recommendations" }
                            div(class="audit-view__recommendations") {
                                Indexed(
                                    each=recs.clone(),
                                    key=|rec| rec.clone(),
                                    view=move |rec| {
                                        view! {
                                            div(class="audit-view__rec-item", data-testid="audit-rec-item") {
                                                span(class="badge badge--info", data-testid="audit-rec-badge") {
                                                    "#"
                                                }
                                                (rec)
                                            }
                                        }
                                    },
                                )
                                (if recs_empty {
                                    view! {
                                        div(class="audit-view__empty", data-testid="audit-no-recs") {
                                            "No recommendations -- all checks passed."
                                        }
                                    }
                                } else {
                                    view! {}
                                })
                            }
                        }
                    }
                }
                None => {
                    if !loading {
                        view! {
                            div(class="audit-view__empty", data-testid="audit-empty") {
                                "Run an audit to see compliance results."
                            }
                        }
                    } else {
                        view! {}
                    }
                }
            })
        }
    }
}
