use rsc_compat::prelude::*;

/// Layout wrapper for the Projects feature.
/// Provides a consistent heading and container for all project pages.
/// The <slot /> element is replaced by the routed page content.
#[component]
pub fn projects_layout(children: Children) -> View {
    view! {
        style {
            .projects-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
            }

            .projects-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
            }

            .projects-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }
        }

        <div class="projects-layout" data-testid="projects-layout">
            <div class="projects-layout__header">
                <h1 class="projects-layout__heading" data-testid="projects-heading">
                    "Projects"
                </h1>
            </div>
            {children}
        </div>
    }
}
