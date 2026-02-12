use rsc_ui::prelude::*;

/// Templates feature layout wrapper.
/// Provides the page heading and consistent spacing for all template routes.
component TemplatesLayout(children: Children) {
    style {
        .templates-layout {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
            width: 100%;
            max-width: 1400px;
            margin: 0 auto;
        }

        .templates-layout__header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-4);
        }

        .templates-layout__heading {
            font-size: var(--font-size-2xl);
            font-weight: 700;
            color: var(--color-text);
            margin: 0;
        }

        .templates-layout__description {
            font-size: var(--font-size-sm);
            color: var(--color-text-muted);
        }

        .templates-layout__body {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
        }
    }

    render {
        <section class="templates-layout" data-testid="templates-layout" aria-label="Template Library">
            <header class="templates-layout__header">
                <div>
                    <h1 class="templates-layout__heading" data-testid="templates-heading">
                        "Template Library"
                    </h1>
                    <p class="templates-layout__description" data-testid="templates-description">
                        "Browse and apply compliance document templates."
                    </p>
                </div>
            </header>
            <div class="templates-layout__body" data-testid="templates-body">
                <slot />
            </div>
        </section>
    }
}
