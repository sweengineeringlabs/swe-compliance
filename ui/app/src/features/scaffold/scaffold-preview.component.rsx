use rsc_ui::prelude::*;
use crate::features::scaffold::scaffold_type::ParsedDomain;

/// Preview of parsed SRS structure (FR-501).
component ScaffoldPreview(domains: Signal<Vec<ParsedDomain>>) {
    render {
        <Table data-testid="scaffold-preview">
            <thead><tr><th>"Section"</th><th>"Title"</th><th>"Requirements"</th></tr></thead>
            <tbody>
                @for domain in domains.get().iter() {
                    <tr data-testid={format!("domain-row-{}", domain.slug)}>
                        <td data-testid="domain-section">{&domain.section}</td>
                        <td data-testid="domain-title">{&domain.title}</td>
                        <td><Badge data-testid="domain-req-count">{domain.requirements.len()}</Badge></td>
                    </tr>
                }
            </tbody>
        </Table>
    }
}
