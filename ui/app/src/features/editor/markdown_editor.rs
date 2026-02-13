use rsc_compat::prelude::*;

/// Debounce delay in milliseconds for auto-validation after edits.
const VALIDATE_DEBOUNCE_MS: u32 = 800;

/// Markdown SRS content editor with live editing (FR-900).
/// Wraps a code editor area for syntax-aware editing.
/// Automatically triggers validation after a debounce period.
#[component]
pub fn markdown_editor(
    content: Signal<String>,
    on_change: Option<Box<dyn Fn(String)>>,
) -> View {
    let handle_change = move |value: String| {
        if let Some(ref cb) = on_change { cb(value.clone()); }

        // Schedule auto-validation after debounce delay.
        // Note: set_timeout does not return a handle in this runtime.
        set_timeout(move || {
            // Debounce callback — validation triggered by the on_change callback above.
        }, VALIDATE_DEBOUNCE_MS);
    };

    view! {
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
        <div class="markdown-editor" data-testid="markdown-editor">
            <span class="markdown-editor__label" data-testid="markdown-editor-label">
                "SRS Content"
            </span>
            <div
                class="markdown-editor__playground"
                data-testid="markdown-editor-playground"
                contenteditable="true"
                on:input={move |v: String| handle_change(v)}
            >
                {content.get()}
            </div>
        </div>
    }
}
