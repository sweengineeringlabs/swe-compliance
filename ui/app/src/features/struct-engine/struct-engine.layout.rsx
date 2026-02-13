use rsc_ui::prelude::*;

/// Layout wrapper for all struct engine pages.
/// Provides a consistent heading and content slot for child routes.
component StructEngineLayout(children: Children) {
    style {
        .struct-layout {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
            width: 100%;
            max-width: 1400px;
            margin: 0 auto;
        }

        .struct-layout__header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-4);
        }

        .struct-layout__heading {
            font-size: var(--font-size-2xl);
            font-weight: 700;
            color: var(--color-text);
            margin: 0;
        }

        .struct-layout__description {
            font-size: var(--font-size-sm);
            color: var(--color-text-secondary);
            margin: 0;
        }

        .struct-layout__body {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
        }
    }

    render {
        <section
            class="struct-layout"
            data-testid="struct-layout"
            aria-label="Struct Engine"
        >
            <header class="struct-layout__header">
                <div>
                    <h1
                        class="struct-layout__heading"
                        data-testid="struct-heading"
                    >
                        "Struct Engine"
                    </h1>
                    <p class="struct-layout__description" data-testid="struct-description">
                        "View structural compliance scan results and crate layout."
                    </p>
                </div>
            </header>
            <div class="struct-layout__body" data-testid="struct-body">
                <slot />
            </div>
        </section>
    }
}
