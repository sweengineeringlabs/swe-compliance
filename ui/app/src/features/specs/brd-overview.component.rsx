use rsc_ui::prelude::*;
use crate::features::specs::specs_type::BrdEntry;

/// Table showing BRD (Business Requirements Document) entries (FR-1003).
/// Displays the requirement ID, title, status badge, and count of linked spec files.
component BrdOverview(entries: Signal<Vec<BrdEntry>>) {
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

    render {
        <Card data-testid="brd-overview">
            <h3 class="brd-overview__heading" data-testid="brd-overview-heading">
                "Business Requirements"
            </h3>
            @if entries.get().is_empty() {
                <div class="brd-overview__empty" data-testid="brd-overview-empty">
                    "No BRD entries found."
                </div>
            } @else {
                <Table data-testid="brd-overview-table">
                    <thead>
                        <tr>
                            <th>"ID"</th>
                            <th>"Title"</th>
                            <th>"Status"</th>
                            <th>"Spec Files"</th>
                        </tr>
                    </thead>
                    <tbody>
                        @for entry in entries.get().iter() {
                            <tr data-testid={format!("brd-row-{}", entry.id)}>
                                <td data-testid="brd-id">{&entry.id}</td>
                                <td data-testid="brd-title">{&entry.title}</td>
                                <td>
                                    <Badge
                                        variant={match entry.status.as_str() {
                                            "approved" => "success",
                                            "draft" => "warning",
                                            "rejected" => "danger",
                                            _ => "secondary",
                                        }}
                                        data-testid="brd-status"
                                    >
                                        {&entry.status}
                                    </Badge>
                                </td>
                                <td data-testid="brd-spec-count">
                                    {format!("{}", entry.spec_files.len())}
                                </td>
                            </tr>
                        }
                    </tbody>
                </Table>
            }
        </Card>
    }
}
