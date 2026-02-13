use rsc_ui::prelude::*;
use crate::features::editor::store::{self, EditorStore};
use crate::features::editor::markdown_editor::MarkdownEditor;
use crate::features::editor::markdown_preview::MarkdownPreview;
use crate::features::editor::validation_panel::ValidationPanel;

/// Editor landing page (FR-900..903).
/// Provides a split-pane SRS editor with live preview,
/// validation controls, and save/load functionality.
component EditorLanding() {
    let s = use_context::<EditorStore>();

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
                    content={s.content.clone()}
                    on_change={Some(Box::new({
                        let content_sig = s.content.clone();
                        let dirty_sig = s.dirty.clone();
                        move |v: String| { content_sig.set(v); dirty_sig.set(true); }
                    }))}
                />
                <MarkdownPreview content={s.content.clone()} />
            </div>
            <ValidationPanel validation={s.validation.clone()} />
            <div class="editor__actions" data-testid="editor-actions">
                <Button
                    label="Validate"
                    variant="secondary"
                    on:click={{ let s2 = s.clone(); move || { let s3 = s2.clone(); spawn(async move { store::validate(&s3).await; }); } }}
                    data-testid="editor-validate-btn"
                />
                <Button
                    label="Save"
                    variant="primary"
                    disabled={!s.dirty.get()}
                    on:click={{ let s2 = s.clone(); move || { let s3 = s2.clone(); spawn(async move { store::save(&s3).await; }); } }}
                    data-testid="editor-save-btn"
                />
                @if s.saved.get() && !s.dirty.get() {
                    <span class="editor__status" data-testid="editor-saved-status">
                        "All changes saved."
                    </span>
                }
            </div>
            @if let Some(ref err) = s.error.get().as_ref() {
                <Toast variant="danger" data-testid="editor-error-toast">
                    {err.as_str()}
                </Toast>
            }
        </div>
    }
}
