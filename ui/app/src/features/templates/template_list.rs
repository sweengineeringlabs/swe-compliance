use rsc_compat::prelude::*;
use crate::features::templates::types::TemplateEntry;

/// Table listing available templates with name, category, file count, and tags (FR-600).
#[component]
pub fn template_list(
    templates: Signal<Vec<TemplateEntry>>,
    on_select: Option<Box<dyn Fn(TemplateEntry)>>,
    loading: bool,
) -> View {
    view! {
        style {
            .template-list { display: flex; flex-direction: column; gap: var(--space-3); }
            .template-list__tags { display: flex; gap: var(--space-1); flex-wrap: wrap; }
        }
        div(class="template-list", data-testid="template-list") {
            (if loading {
                view! {
                    p(data-testid="template-list-loading") { "Loading templates..." }
                }
            } else if templates.get().is_empty() {
                view! {
                    p(data-testid="template-list-empty") { "No templates available." }
                }
            } else {
                view! {
                    Table(data-testid="template-list-table") {
                        thead {
                            tr {
                                th { "Name" }
                                th { "Category" }
                                th { "Files" }
                                th { "Tags" }
                            }
                        }
                        tbody {
                            Keyed(
                                iterable=templates.get(),
                                key=|tpl| tpl.name.clone(),
                                view={
                                    let on_select = on_select.clone();
                                    move |tpl| {
                                        let t = tpl.clone();
                                        let on_select = on_select.clone();
                                        view! {
                                            tr(
                                                on:click=move || { if let Some(ref cb) = on_select { cb(t.clone()) } },
                                                data-testid=format!("template-row-{}", tpl.name),
                                            ) {
                                                td(data-testid="template-name") { (tpl.name.clone()) }
                                                td(data-testid="template-category") { (tpl.category.clone()) }
                                                td(data-testid="template-file-count") { (tpl.file_count) }
                                                td {
                                                    div(class="template-list__tags") {
                                                        Keyed(
                                                            iterable=tpl.tags.clone(),
                                                            key=|tag| tag.clone(),
                                                            view=|tag| view! {
                                                                Badge(variant="info", data-testid=format!("template-tag-{}", tag)) {
                                                                    (tag)
                                                                }
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            )
                        }
                    }
                }
            })
        }
    }
}
