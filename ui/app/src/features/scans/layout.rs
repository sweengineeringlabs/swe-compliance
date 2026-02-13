use rsc_compat::prelude::*;

/// Layout wrapper for all scan execution pages.
/// Provides a consistent heading and content slot for child routes.
#[component]
pub fn scans_layout(children: Children) -> View {
    view! {
        style { r#"
            .scans-layout { display: flex; flex-direction: column; gap: var(--space-6); width: 100%; }
            .scans-layout__header { display: flex; align-items: center; justify-content: space-between; }
            .scans-layout__heading { font-size: var(--font-size-2xl); font-weight: 700; color: var(--color-text); margin: 0; }
            .scans-layout__content { display: flex; flex-direction: column; gap: var(--space-6); }
        "# }
        section(class="scans-layout", data-testid="scans-layout", aria-label="Scan Execution") {
            div(class="scans-layout__header") {
                h1(class="scans-layout__heading", data-testid="scans-heading") {
                    "Scan Execution"
                }
            }
            div(class="scans-layout__content", data-testid="scans-content") {
                (children)
            }
        }
    }
}
