use rsc_compat::prelude::*;

/// Layout wrapper for all report and export pages.
/// Provides a consistent heading and content slot for child routes.
#[component]
pub fn reports_layout(children: Children) -> View {
    view! {
        style {
            .reports-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
                max-width: 1400px;
                margin: 0 auto;
            }

            .reports-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: var(--space-4);
            }

            .reports-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }

            .reports-layout__description {
                font-size: var(--font-size-sm);
                color: var(--color-text-muted);
            }

            .reports-layout__body {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
        }
        section(class="reports-layout", data-testid="reports-layout", aria-label="Reports & Export") {
            header(class="reports-layout__header") {
                div {
                    h1(class="reports-layout__heading", data-testid="reports-heading") {
                        "Reports & Export"
                    }
                    p(class="reports-layout__description") {
                        "Generate and export compliance reports in multiple formats."
                    }
                }
            }
            div(class="reports-layout__body", data-testid="reports-body") {
                (children)
            }
        }
    }
}
