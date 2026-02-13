use rsc_compat::prelude::*;
use super::types::Scan;

/// Table of past scans ordered by timestamp (FR-305).
#[component]
pub fn scan_history(scans: Signal<Vec<Scan>>) -> View {
    let rows = scans.get().iter().map(|scan| {
        let scan_id = scan.id.clone();
        let scan_id_short = if scan.id.len() >= 8 { scan.id[..8].to_string() } else { scan.id.clone() };
        let engine = scan.engine.clone();
        let status = scan.status.clone();
        let started_at = scan.started_at.clone();
        let badge_variant = match scan.status.as_str() {
            "completed" => "success",
            "failed" => "danger",
            _ => "warning",
        };
        let is_completed = scan.status == "completed";
        view! {
            tr(data-testid=format!("scan-row-{}", scan_id)) {
                td(data-testid="scan-id") { (scan_id_short) }
                td(data-testid="scan-engine") { (engine) }
                td {
                    Badge(
                        variant=badge_variant,
                        data-testid="scan-status",
                    ) {
                        (status)
                    }
                }
                td(data-testid="scan-started") { (started_at) }
                td {
                    (if is_completed {
                        view! {
                            a(href=format!("/violations?scan={}", scan_id), data-testid="scan-view-link") {
                                "View"
                            }
                        }
                    } else {
                        view! {}
                    })
                }
            }
        }
    }).collect::<Vec<_>>();

    view! {
        style {
            .scan-history__status--completed { color: var(--color-success); }
            .scan-history__status--failed { color: var(--color-error); }
            .scan-history__status--running { color: var(--color-warning); }
        }
        Table(data-testid="scan-history") {
            thead {
                tr {
                    th { "ID" }
                    th { "Engine" }
                    th { "Status" }
                    th { "Started" }
                    th { "Actions" }
                }
            }
            tbody {
                (rows)
            }
        }
    }
}
