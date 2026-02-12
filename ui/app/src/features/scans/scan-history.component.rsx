use rsc_ui::prelude::*;
use crate::features::scans::scans_type::Scan;

/// Table of past scans ordered by timestamp (FR-305).
component ScanHistory(scans: Signal<Vec<Scan>>) {
    style {
        .scan-history__status--completed { color: var(--color-success); }
        .scan-history__status--failed { color: var(--color-error); }
        .scan-history__status--running { color: var(--color-warning); }
    }

    render {
        <Table data-testid="scan-history">
            <thead>
                <tr><th>"ID"</th><th>"Engine"</th><th>"Status"</th><th>"Started"</th><th>"Actions"</th></tr>
            </thead>
            <tbody>
                @for scan in scans.get().iter() {
                    <tr data-testid={format!("scan-row-{}", scan.id)}>
                        <td data-testid="scan-id">{&scan.id[..8]}</td>
                        <td data-testid="scan-engine">{&scan.engine}</td>
                        <td>
                            <Badge
                                variant={match scan.status.as_str() {
                                    "completed" => "success",
                                    "failed" => "danger",
                                    _ => "warning",
                                }}
                                data-testid="scan-status"
                            >
                                {&scan.status}
                            </Badge>
                        </td>
                        <td data-testid="scan-started">{&scan.started_at}</td>
                        <td>
                            @if scan.status == "completed" {
                                <a href={format!("/violations?scan={}", scan.id)} data-testid="scan-view-link">"View"</a>
                            }
                        </td>
                    </tr>
                }
            </tbody>
        </Table>
    }
}
