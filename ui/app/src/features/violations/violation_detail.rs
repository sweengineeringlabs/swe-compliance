use rsc_compat::prelude::*;
use crate::features::violations::types::ViolationEntry;

/// Expandable detail panel showing fix guidance (FR-404).
/// Note: Accordion component rendered as a div with class "accordion"
/// since Accordion is not available in rsc_compat.
#[component]
pub fn violation_detail(violation: ViolationEntry) -> View {
    view! {
        style {
            .violation-detail { padding: var(--space-4); }
            .violation-detail__field { margin-bottom: var(--space-2); }
            .violation-detail__label { font-weight: 600; font-size: var(--font-size-sm); color: var(--color-text-secondary); }
        }
        div(class="accordion", data-testid="violation-detail") {
            div(class="violation-detail") {
                div(class="violation-detail__field") {
                    span(class="violation-detail__label") { "Check: " }
                    (format!("#{} \u{2014} {}", violation.check_id, violation.description))
                }
                div(class="violation-detail__field") {
                    span(class="violation-detail__label") { "Category: " }
                    (&violation.category)
                }
                div(class="violation-detail__field") {
                    span(class="violation-detail__label") { "Severity: " }
                    Badge(
                        variant=match violation.severity.as_str() {
                            "Error" => "danger",
                            "Warning" => "warning",
                            _ => "info",
                        },
                        data-testid="detail-severity",
                    ) {
                        (&violation.severity)
                    }
                }
                (if let Some(ref path) = violation.file_path {
                    view! {
                        div(class="violation-detail__field") {
                            span(class="violation-detail__label") { "File: " }
                            (path)
                        }
                    }
                } else {
                    view! {}
                })
                div(class="violation-detail__field") {
                    span(class="violation-detail__label") { "Message: " }
                    (&violation.message)
                }
            }
        }
    }
}
