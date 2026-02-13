use rsc_compat::prelude::*;
use crate::features::scaffold::types::ParsedDomain;

/// Preview of parsed SRS structure (FR-501).
#[component]
pub fn scaffold_preview(domains: Signal<Vec<ParsedDomain>>) -> View {
    view! {
        Table(data-testid="scaffold-preview") {
            thead {
                tr {
                    th { "Section" }
                    th { "Title" }
                    th { "Requirements" }
                }
            }
            tbody {
                Keyed(
                    iterable=domains.get(),
                    key=|domain| domain.slug.clone(),
                    view=|domain| view! {
                        tr(data-testid=format!("domain-row-{}", domain.slug)) {
                            td(data-testid="domain-section") { (domain.section.clone()) }
                            td(data-testid="domain-title") { (domain.title.clone()) }
                            td {
                                Badge(data-testid="domain-req-count") {
                                    (domain.requirements.len())
                                }
                            }
                        }
                    },
                )
            }
        }
    }
}
