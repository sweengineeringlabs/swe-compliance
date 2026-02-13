use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::reports::types::ReportData;

/// Stub for download_blob -- triggers a file download in the browser.
fn download_blob(content: &str, filename: &str, mime_type: &str) {
    // Platform-specific download implementation provided by rsc_compat runtime.
    let _ = (content, filename, mime_type);
}

/// Report export card with format selection and content preview (FR-700, FR-701, FR-702).
///
/// Allows the user to choose an export format (JSON, Markdown, PDF), trigger
/// the export, and view or download the generated report.
#[component]
pub fn report_export(
    scan_id: Option<String>,
    format: String,
    report: Option<ReportData>,
    loading: bool,
    on_format_change: Option<Box<dyn Fn(String)>>,
    on_export: Option<Box<dyn Fn()>>,
) -> View {
    let on_format_change: Option<Rc<dyn Fn(String)>> = on_format_change.map(|f| Rc::from(f));
    let on_export: Option<Rc<dyn Fn()>> = on_export.map(|f| Rc::from(f));
    view! {
        style {
            .report-export { display: flex; flex-direction: column; gap: var(--space-4); }
            .report-export__controls { display: flex; gap: var(--space-3); align-items: flex-end; }
            .report-export__preview { margin-top: var(--space-3); }
            .report-export__download { display: inline-flex; align-items: center; gap: var(--space-2); margin-top: var(--space-3); }
            .report-export__meta { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-2); }
        }
        div(class="card", data-testid="report-export") {
            div(class="report-export") {
                h3 { "Export Report" }

                div(class="report-export__controls") {
                    div(class="form-field") {
                        label { "Format" }
                        select(
                            value=format.clone(),
                            on:change=move |v: String| { if let Some(ref cb) = on_format_change { cb(v) } },
                            data-testid="report-format-select"
                        ) {
                            option(value="json") { "JSON" }
                            option(value="markdown") { "Markdown" }
                            option(value="pdf") { "PDF" }
                        }
                    }

                    button(
                        class="btn btn--primary",
                        disabled=loading || scan_id.is_none(),
                        on:click=move || { if let Some(ref cb) = on_export { cb() } },
                        data-testid="report-export-btn"
                    ) {
                        "Export Report"
                    }
                }

                (if loading {
                    view! {
                        div(class="spinner", role="status", data-testid="report-export-spinner") {
                            "Loading..."
                        }
                    }
                } else {
                    view! {}
                })

                (if let Some(ref data) = report {
                    view! {
                        div(class="report-export__preview", data-testid="report-export-preview") {
                            (if data.format == "pdf" {
                                view! {
                                    div(class="report-export__download") {
                                        button(
                                            class="btn btn--secondary",
                                            on:click={
                                                let content = data.content.clone();
                                                move || {
                                                    download_blob(&content, "report.pdf", "application/pdf");
                                                }
                                            },
                                            data-testid="report-download-pdf-btn"
                                        ) {
                                            "Download PDF"
                                        }
                                    }
                                }
                            } else {
                                let lang = if data.format == "json" { "json" } else { "markdown" };
                                view! {
                                    div(class="code-block", data-language=lang, data-testid="report-content-block") {
                                        pre {
                                            code {
                                                (data.content.clone())
                                            }
                                        }
                                    }
                                }
                            })

                            (if !data.generated_at.is_empty() {
                                let ts = format!("Generated at: {}", data.generated_at);
                                view! {
                                    p(class="report-export__meta", data-testid="report-generated-at") {
                                        (ts)
                                    }
                                }
                            } else {
                                view! {}
                            })
                        }
                    }
                } else {
                    view! {}
                })
            }
        }
    }
}
