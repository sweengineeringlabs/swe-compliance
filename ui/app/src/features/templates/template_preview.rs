use rsc_compat::prelude::*;
use crate::features::templates::types::TemplateEntry;

/// Card showing selected template details with a copy action (FR-601, FR-602).
#[component]
pub fn template_preview(
    template: TemplateEntry,
    on_copy: Option<Box<dyn Fn()>>,
) -> View {
    view! {
        style {
            .template-preview { display: flex; flex-direction: column; gap: var(--space-3); }
            .template-preview__meta { display: flex; gap: var(--space-4); align-items: center; flex-wrap: wrap; }
            .template-preview__tags { display: flex; gap: var(--space-1); flex-wrap: wrap; }
            .template-preview__actions { display: flex; justify-content: flex-end; padding-top: var(--space-2); }
        }
        Card(data-testid="template-preview") {
            div(class="template-preview") {
                h2(data-testid="template-preview-name") { (template.name.clone()) }
                p(data-testid="template-preview-description") { (template.description.clone()) }
                div(class="template-preview__meta") {
                    Badge(variant="default", data-testid="template-preview-category") {
                        (template.category.clone())
                    }
                    span(data-testid="template-preview-file-count") {
                        (format!("{} files", template.file_count))
                    }
                }
                div(class="template-preview__tags") {
                    Keyed(
                        iterable=template.tags.clone(),
                        key=|tag| tag.clone(),
                        view=|tag| view! {
                            Badge(variant="info", data-testid=format!("template-preview-tag-{}", tag)) {
                                (tag)
                            }
                        },
                    )
                }
                div(class="template-preview__actions") {
                    Button(
                        label="Copy to Project",
                        variant="primary",
                        on:click=move || { if let Some(ref cb) = on_copy { cb() } },
                        data-testid="template-copy-btn",
                    )
                }
            }
        }
    }
}
