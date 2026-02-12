use rsc_ui::prelude::*;

/// Editor feature layout wrapper.
/// Provides the page heading and consistent spacing for all editor routes.
component EditorLayout(children: Children) {
    style {
        .editor-layout {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
            width: 100%;
            max-width: 1400px;
            margin: 0 auto;
        }

        .editor-layout__header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-4);
        }

        .editor-layout__heading {
            font-size: var(--font-size-2xl);
            font-weight: 700;
            color: var(--color-text);
            margin: 0;
        }

        .editor-layout__description {
            font-size: var(--font-size-sm);
            color: var(--color-text-muted);
        }

        .editor-layout__body {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
        }
    }

    render {
        <section class="editor-layout" data-testid="editor-layout" aria-label="SRS Editor">
            <header class="editor-layout__header">
                <div>
                    <h1 class="editor-layout__heading" data-testid="editor-heading">
                        "SRS Editor"
                    </h1>
                    <p class="editor-layout__description" data-testid="editor-description">
                        "Edit, validate, and manage SRS documents."
                    </p>
                </div>
            </header>
            <div class="editor-layout__body" data-testid="editor-body">
                <slot />
            </div>
        </section>
    }
}
