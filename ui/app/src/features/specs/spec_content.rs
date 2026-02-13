use rsc_compat::prelude::*;
use crate::features::specs::types::SpecFile;

/// Card displaying the raw content of a selected spec file (FR-1002).
/// Shows the file name, kind badge, and path in the card header,
/// with the file content rendered inside a code block.
#[component]
pub fn spec_content(
    file: Signal<Option<SpecFile>>,
    content: String,
) -> View {
    view! {
        style {
            .spec-content__header {
                display: flex;
                align-items: center;
                gap: var(--space-3);
                margin-bottom: var(--space-3);
            }

            .spec-content__name {
                font-size: var(--font-size-lg);
                font-weight: 600;
                color: var(--color-text);
            }

            .spec-content__path {
                font-size: var(--font-size-xs);
                color: var(--color-text-muted);
                font-family: var(--font-mono);
                margin-bottom: var(--space-3);
            }

            .spec-content__empty {
                padding: var(--space-6);
                text-align: center;
                color: var(--color-text-muted);
            }

            .spec-content__codeblock {
                white-space: pre-wrap;
                font-family: var(--font-family-mono);
                font-size: var(--font-size-sm);
                background: var(--color-surface);
                padding: var(--space-4);
                border-radius: var(--radius-md);
                border: 1px solid var(--color-border);
                overflow-x: auto;
            }
        }
        <div class="card" data-testid="spec-content">
            {
                let file_opt = file.get().clone();
                if let Some(ref f) = file_opt {
                    let name = f.name.clone();
                    let kind_cls = format!("badge badge--{}", f.kind_variant());
                    let kind_lbl = f.kind_label().to_string();
                    let path = f.path.clone();
                    let content = content.clone();
                    view! {
                        <div>
                            <div class="spec-content__header" data-testid="spec-content-header">
                                <span class="spec-content__name" data-testid="spec-content-name">
                                    {name.as_str()}
                                </span>
                                <span class={kind_cls} data-testid="spec-content-kind">
                                    {kind_lbl.as_str()}
                                </span>
                            </div>
                            <div class="spec-content__path" data-testid="spec-content-path">
                                {path.as_str()}
                            </div>
                            <div class="spec-content__codeblock" data-testid="spec-content-code">
                                {content.as_str()}
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div class="spec-content__empty" data-testid="spec-content-empty">
                            "Select a file from the spec tree to view its content."
                        </div>
                    }
                }
            }
        </div>
    }
}
