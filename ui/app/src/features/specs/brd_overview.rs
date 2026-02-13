use rsc_compat::prelude::*;
use crate::features::specs::types::BrdEntry;

/// Table showing BRD (Business Requirements Document) entries (FR-1003).
/// Displays the requirement ID, title, status badge, and count of linked spec files.
#[component]
pub fn brd_overview(entries: Signal<Vec<BrdEntry>>) -> View {
    let is_empty = entries.get().is_empty();
    let rows: Vec<_> = entries.get().iter().map(|entry| {
        let status_variant = match entry.status.as_str() {
            "approved" => "success",
            "draft" => "warning",
            "rejected" => "danger",
            _ => "secondary",
        };
        let id = entry.id.clone();
        let title = entry.title.clone();
        let status = entry.status.clone();
        let spec_count = format!("{}", entry.spec_files.len());
        let badge_cls = format!("badge badge--{}", status_variant);
        let row_testid = format!("brd-row-{}", id);
        view! {
            <tr data-testid={row_testid}>
                <td data-testid="brd-id">{id.as_str()}</td>
                <td data-testid="brd-title">{title.as_str()}</td>
                <td>
                    <span class={badge_cls} data-testid="brd-status">
                        {status.as_str()}
                    </span>
                </td>
                <td data-testid="brd-spec-count">
                    {spec_count}
                </td>
            </tr>
        }
    }).collect();

    view! {
        style {
            .brd-overview__empty {
                padding: var(--space-6);
                text-align: center;
                color: var(--color-text-muted);
                font-size: var(--font-size-sm);
            }

            .brd-overview__heading {
                font-size: var(--font-size-lg);
                font-weight: 600;
                color: var(--color-text);
                margin-bottom: var(--space-3);
            }
        }
        <div class="card" data-testid="brd-overview">
            <h3 class="brd-overview__heading" data-testid="brd-overview-heading">
                "Business Requirements"
            </h3>
            {if is_empty {
                view! {
                    <div class="brd-overview__empty" data-testid="brd-overview-empty">
                        "No BRD entries found."
                    </div>
                }
            } else {
                view! {
                    <table class="table" data-testid="brd-overview-table">
                        <thead>
                            <tr>
                                <th>"ID"</th>
                                <th>"Title"</th>
                                <th>"Status"</th>
                                <th>"Spec Files"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {rows}
                        </tbody>
                    </table>
                }
            }}
        </div>
    }
}
