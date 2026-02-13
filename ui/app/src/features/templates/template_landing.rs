use rsc_compat::prelude::*;
use crate::features::templates::store::{self, TemplatesStore};
use crate::features::templates::template_list::template_list;
use crate::features::templates::template_preview::template_preview;
use crate::features::templates::compliance_checklist::compliance_checklist;

/// Templates landing page (FR-600..603).
#[component]
pub fn templates_landing() -> View {
    let s = use_context::<TemplatesStore>();

    view! {
        style {
            .templates { display: flex; flex-direction: column; gap: var(--space-4); }
            .templates__content { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
        }
        div(class="templates", data-testid="templates-landing") {
            (template_list(
                s.templates.clone(),
                Some(Box::new({
                    let selected = s.selected_template.clone();
                    move |t| selected.set(Some(t))
                })),
                s.loading.get(),
            ))
            (if let Some(tpl) = s.selected_template.get() {
                let s2 = s.clone();
                let s3 = s.clone();
                view! {
                    div(class="templates__content") {
                        (template_preview(
                            tpl,
                            Some(Box::new({
                                let s4 = s2.clone();
                                move || store::copy_selected(&s4)
                            })),
                        ))
                        (compliance_checklist(
                            s3.checklist_items.clone(),
                            Some(Box::new({
                                let s5 = s3.clone();
                                move |id: String| store::toggle_checklist_item(&s5, &id)
                            })),
                        ))
                    }
                }
            } else {
                view! {}
            })
        }
    }
}
