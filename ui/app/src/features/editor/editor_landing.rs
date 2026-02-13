use rsc_compat::prelude::*;
use crate::features::editor::store::{self, EditorStore};
use crate::features::editor::markdown_editor::markdown_editor;
use crate::features::editor::markdown_preview::markdown_preview;
use crate::features::editor::validation_panel::validation_panel;

/// Editor landing page (FR-900..903).
/// Provides a split-pane SRS editor with live preview,
/// validation controls, and save/load functionality.
#[component]
pub fn editor_landing() -> View {
    let s = use_context::<EditorStore>();

    view! {
        style {
            .editor { display: flex; flex-direction: column; gap: var(--space-4); }
            .editor__panes { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); min-height: 500px; }
            .editor__actions { display: flex; gap: var(--space-3); align-items: center; }
            .editor__status { font-size: var(--font-size-sm); color: var(--color-text-muted); }
        }
        <div class="editor" data-testid="editor-landing">
            <div class="editor__panes" data-testid="editor-panes">
                {markdown_editor(
                    s.content.clone(),
                    Some(Box::new({
                        let content_sig = s.content.clone();
                        let dirty_sig = s.dirty.clone();
                        move |v: String| { content_sig.set(v); dirty_sig.set(true); }
                    })),
                )}
                {markdown_preview(s.content.clone())}
            </div>
            {validation_panel(s.validation.clone())}
            <div class="editor__actions" data-testid="editor-actions">
                <Button
                    label="Validate"
                    variant="secondary"
                    on:click={
                        let s2 = s.clone();
                        move || { let s3 = s2.clone(); spawn(async move { store::validate(&s3).await; }); }
                    }
                    data-testid="editor-validate-btn"
                />
                <Button
                    label="Save"
                    variant="primary"
                    disabled={!s.dirty.get()}
                    on:click={
                        let s2 = s.clone();
                        move || { let s3 = s2.clone(); spawn(async move { store::save(&s3).await; }); }
                    }
                    data-testid="editor-save-btn"
                />
                {if s.saved.get() && !s.dirty.get() {
                    view! {
                        <span class="editor__status" data-testid="editor-saved-status">
                            "All changes saved."
                        </span>
                    }
                } else {
                    view! {}
                }}
            </div>
            {
                let err_opt = s.error.get().clone();
                if let Some(ref err_msg) = err_opt {
                    view! {
                        <div class="toast toast--danger" role="alert" data-testid="editor-error-toast">
                            {err_msg.as_str()}
                        </div>
                    }
                } else {
                    view! {}
                }
            }
        </div>
    }
}
