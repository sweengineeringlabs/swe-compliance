use rsc_compat::prelude::*;

/// Specs feature layout wrapper.
/// Provides the page heading and consistent spacing for all spec browser routes.
#[component]
pub fn specs_layout(children: Children) -> View {
    view! {
        style {
            .specs-layout {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
                width: 100%;
                max-width: 1400px;
                margin: 0 auto;
            }

            .specs-layout__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: var(--space-4);
            }

            .specs-layout__heading {
                font-size: var(--font-size-2xl);
                font-weight: 700;
                color: var(--color-text);
                margin: 0;
            }

            .specs-layout__description {
                font-size: var(--font-size-sm);
                color: var(--color-text-muted);
            }

            .specs-layout__body {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
        }
        <section class="specs-layout" data-testid="specs-layout" aria-label="Spec Browser">
            <header class="specs-layout__header">
                <div>
                    <h1 class="specs-layout__heading" data-testid="specs-heading">
                        "Spec Browser"
                    </h1>
                    <p class="specs-layout__description" data-testid="specs-description">
                        "Browse and inspect compliance specification files."
                    </p>
                </div>
            </header>
            <div class="specs-layout__body" data-testid="specs-body">
                {children}
            </div>
        </section>
    }
}
