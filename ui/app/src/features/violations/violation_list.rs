use rsc_compat::prelude::*;
use crate::features::violations::types::ViolationEntry;

/// Sortable table of violations with severity badges (FR-400..401).
#[component]
pub fn violation_list(
    violations: Signal<Vec<ViolationEntry>>,
    on_select: Option<Box<dyn Fn(usize)>>,
) -> View {
    let rows = violations.get().iter().enumerate().map(|(idx, v)| {
        let check_id = format!("{}", v.check_id);
        let category = v.category.clone();
        let severity = v.severity.clone();
        let file_path = v.file_path.clone().unwrap_or_else(|| "-".into());
        let message = v.message.clone();
        let badge_variant = match v.severity.as_str() {
            "Error" => "danger",
            "Warning" => "warning",
            _ => "info",
        };
        let row_testid = format!("violation-row-{}", v.check_id);
        view! {
            tr(
                data-testid=row_testid,
            ) {
                td(data-testid="violation-check-id") { (check_id) }
                td(data-testid="violation-category") { (category) }
                td {
                    Badge(
                        variant=badge_variant,
                        data-testid="violation-severity",
                    ) {
                        (severity)
                    }
                }
                td(data-testid="violation-file") { (file_path) }
                td(data-testid="violation-message") { (message) }
            }
        }
    }).collect::<Vec<_>>();

    view! {
        Table(data-testid="violation-list") {
            thead {
                tr {
                    th { "Check ID" }
                    th { "Category" }
                    th { "Severity" }
                    th { "File" }
                    th { "Message" }
                }
            }
            tbody {
                (rows)
            }
        }
    }
}
