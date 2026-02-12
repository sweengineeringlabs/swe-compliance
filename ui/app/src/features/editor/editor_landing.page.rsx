use rsc_ui::prelude::*;
use crate::features::editor::editor_store as store;
use crate::features::editor::markdown_editor::MarkdownEditor;
use crate::features::editor::markdown_preview::MarkdownPreview;
use crate::features::editor::validation_panel::ValidationPanel;

/// Editor landing page (FR-900..903).
/// Provides a split-pane SRS editor with live preview,
/// validation controls, and save/load functionality.
component EditorLanding() {
    style {
        .editor { display: flex; flex-direction: column; gap: var(--space-4); }
        .editor__panes { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); min-height: 500px; }
        .editor__actions { display: flex; gap: var(--space-3); align-items: center; }
        .editor__status { font-size: var(--font-size-sm); color: var(--color-text-muted); }
    }

    render {
        <div class="editor" data-testid="editor-landing">
            <div class="editor__panes" data-testid="editor-panes">
                <MarkdownEditor
                    content={store::content.clone()}
                    on_change={|v| { store::content.set(v); store::dirty.set(true); }}
                />
                <MarkdownPreview content={store::content.clone()} />
            </div>
            <ValidationPanel validation={store::validation.clone()} />
            <div class="editor__actions" data-testid="editor-actions">
                <Button
                    label="Validate"
                    variant="secondary"
                    on:click={|| store::validate()}
                    data-testid="editor-validate-btn"
                />
                <Button
                    label="Save"
                    variant="primary"
                    disabled={!store::dirty.get()}
                    on:click={|| store::save()}
                    data-testid="editor-save-btn"
                />
                @if store::saved.get() && !store::dirty.get() {
                    <span class="editor__status" data-testid="editor-saved-status">
                        "All changes saved."
                    </span>
                }
            </div>
            @if let Some(ref err) = store::error.get() {
                <Toast variant="danger" data-testid="editor-error-toast">
                    {err}
                </Toast>
            }
        </div>
    }
}
