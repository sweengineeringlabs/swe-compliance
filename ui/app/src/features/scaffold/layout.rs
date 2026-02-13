use rsc_compat::prelude::*;

/// Scaffold feature layout wrapper.
/// Provides the page heading and consistent spacing for all scaffold routes.
#[component]
pub fn scaffold_layout(children: Children) -> View {
    view! {
        style {
            .scaffold-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
                max-width: 1400px;
                margin: 0 auto;
            }

            .scaffold-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: var(--space-4);
            }

            .scaffold-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }

            .scaffold-layout__description {
                font-size: var(--font-size-sm);
                color: var(--color-text-muted);
            }

            .scaffold-layout__body {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
        }
        section(class="scaffold-layout", data-testid="scaffold-layout", aria-label="Scaffolding Interface") {
            header(class="scaffold-layout__header") {
                div {
                    h1(class="scaffold-layout__heading", data-testid="scaffold-heading") {
                        "Scaffolding Interface"
                    }
                    p(class="scaffold-layout__description", data-testid="scaffold-description") {
                        "Parse SRS documents and generate SDLC compliance artifacts."
                    }
                }
            }
            div(class="scaffold-layout__body", data-testid="scaffold-body") {
                (children)
            }
        }
    }
}
