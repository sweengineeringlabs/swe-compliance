use rsc_compat::prelude::*;

/// Read-only markdown preview of SRS content (FR-900).
/// Renders the current editor content as a formatted code block,
/// displayed side-by-side with the MarkdownEditor.
#[component]
pub fn markdown_preview(content: Signal<String>) -> View {
    view! {
        style {
            .markdown-preview {
                display: flex;
                flex-direction: column;
                gap: var(--space-2);
                height: 100%;
            }

            .markdown-preview__label {
                font-size: var(--font-size-sm);
                font-weight: 600;
                color: var(--color-text);
            }

            .markdown-preview__content {
                flex: 1;
                min-height: 400px;
                overflow-y: auto;
                border: 1px solid var(--color-border);
                border-radius: var(--radius-md);
                padding: var(--space-4);
                background: var(--color-surface);
            }

            .markdown-preview__empty {
                display: flex;
                align-items: center;
                justify-content: center;
                height: 100%;
                color: var(--color-text-muted);
                font-size: var(--font-size-sm);
            }

            .markdown-preview__codeblock {
                white-space: pre-wrap;
                font-family: var(--font-family-mono);
                font-size: var(--font-size-sm);
            }
        }
        <div class="markdown-preview" data-testid="markdown-preview">
            <span class="markdown-preview__label" data-testid="markdown-preview-label">
                "Preview"
            </span>
            <div class="markdown-preview__content" data-testid="markdown-preview-content">
                {if content.get().is_empty() {
                    view! {
                        <div class="markdown-preview__empty" data-testid="markdown-preview-empty">
                            "No content to preview."
                        </div>
                    }
                } else {
                    view! {
                        <div class="markdown-preview__codeblock" data-testid="markdown-preview-codeblock">
                            {content.get()}
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
