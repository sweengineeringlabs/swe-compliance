use rsc_compat::prelude::*;

/// Layout wrapper for all AI compliance feature routes.
/// Provides the page heading and consistent spacing for child views.
#[component]
pub fn ai_layout(children: Children) -> View {
    view! {
        style {
            .ai-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
                max-width: 1400px;
                margin: 0 auto;
            }

            .ai-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: var(--space-4);
            }

            .ai-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }

            .ai-layout__body {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
        }
        section(class="ai-layout", data-testid="ai-layout", aria-label="AI Compliance Features") {
            header(class="ai-layout__header") {
                h1(class="ai-layout__heading", data-testid="ai-heading") {
                    "AI Compliance Features"
                }
            }
            div(class="ai-layout__body", data-testid="ai-body") {
                (children)
            }
        }
    }
}
