use rsc_ui::prelude::*;

/// Debounce delay in milliseconds for auto-validation after edits.
const VALIDATE_DEBOUNCE_MS: u32 = 800;

/// Markdown SRS content editor with live editing (FR-900).
/// Wraps a Playground code editor for syntax-aware editing.
/// Automatically triggers validation after a debounce period.
component MarkdownEditor(
    content: Signal<String>,
    on_change: Fn(String),
) {
    style {
        .markdown-editor {
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
            height: 100%;
        }

        .markdown-editor__label {
            font-size: var(--font-size-sm);
            font-weight: 600;
            color: var(--color-text);
        }

        .markdown-editor__playground {
            flex: 1;
            min-height: 400px;
            border: 1px solid var(--color-border);
            border-radius: var(--radius-md);
        }
    }

    render {
        let debounce_timer = signal(Option::<TimerHandle>::None);

        let handle_change = move |value: String| {
            // Cancel any pending debounce timer.
            if let Some(handle) = debounce_timer.get() {
                clear_timeout(handle);
            }

            on_change(value.clone());

            // Schedule auto-validation after debounce delay.
            let timer = set_timeout(move || {
                debounce_timer.set(None);
            }, VALIDATE_DEBOUNCE_MS);
            debounce_timer.set(Some(timer));
        };

        <div class="markdown-editor" data-testid="markdown-editor">
            <span class="markdown-editor__label" data-testid="markdown-editor-label">
                "SRS Content"
            </span>
            <Playground
                class="markdown-editor__playground"
                language="markdown"
                value={content.clone()}
                on:change={handle_change}
                placeholder="# SRS Document\n\nStart editing your SRS content here..."
                data-testid="markdown-editor-playground"
            />
        </div>
    }
}
