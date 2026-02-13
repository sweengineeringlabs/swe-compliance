use rsc_compat::prelude::*;

/// Violations feature layout wrapper.
/// Provides the page heading and consistent spacing for all violation routes.
#[component]
pub fn violations_layout(children: Children) -> View {
    view! {
        style {
            .violations-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
                max-width: 1400px;
                margin: 0 auto;
            }

            .violations-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: var(--space-4);
            }

            .violations-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }

            .violations-layout__body {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
        }
        section(
            class="violations-layout",
            data-testid="violations-layout",
            aria-label="Violation Browser",
        ) {
            header(class="violations-layout__header") {
                h1(
                    class="violations-layout__heading",
                    data-testid="violations-heading",
                ) {
                    "Violation Browser"
                }
            }
            div(class="violations-layout__body", data-testid="violations-body") {
                (children)
            }
        }
    }
}
