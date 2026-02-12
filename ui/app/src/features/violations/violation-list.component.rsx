use rsc_ui::prelude::*;
use crate::features::violations::violations_type::ViolationEntry;

/// Sortable table of violations with severity badges (FR-400..401).
component ViolationList(
    violations: Signal<Vec<ViolationEntry>>,
    on_select: Fn(usize),
) {
    render {
        <Table data-testid="violation-list">
            <thead>
                <tr>
                    <th>"Check ID"</th><th>"Category"</th><th>"Severity"</th><th>"File"</th><th>"Message"</th>
                </tr>
            </thead>
            <tbody>
                @for (idx, v) in violations.get().iter().enumerate() {
                    <tr on:click={move || on_select(idx)} data-testid={format!("violation-row-{}", v.check_id)}>
                        <td data-testid="violation-check-id">{v.check_id}</td>
                        <td data-testid="violation-category">{&v.category}</td>
                        <td>
                            <Badge variant={match v.severity.as_str() { "Error" => "danger", "Warning" => "warning", _ => "info" }}
                                   data-testid="violation-severity">{&v.severity}</Badge>
                        </td>
                        <td data-testid="violation-file">{v.file_path.as_deref().unwrap_or("-")}</td>
                        <td data-testid="violation-message">{&v.message}</td>
                    </tr>
                }
            </tbody>
        </Table>
    }
}
