use rsc_ui::prelude::*;

/// Read-only markdown preview of SRS content (FR-900).
/// Renders the current editor content as a formatted CodeBlock,
/// displayed side-by-side with the MarkdownEditor.
component MarkdownPreview(content: Signal<String>) {
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
    }

    render {
        <div class="markdown-preview" data-testid="markdown-preview">
            <span class="markdown-preview__label" data-testid="markdown-preview-label">
                "Preview"
            </span>
            <div class="markdown-preview__content" data-testid="markdown-preview-content">
                @if content.get().is_empty() {
                    <div class="markdown-preview__empty" data-testid="markdown-preview-empty">
                        "No content to preview."
                    </div>
                } @else {
                    <CodeBlock
                        language="markdown"
                        code={content.clone()}
                        data-testid="markdown-preview-codeblock"
                    />
                }
            </div>
        </div>
    }
}
