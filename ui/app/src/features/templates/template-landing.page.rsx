use rsc_ui::prelude::*;
use crate::features::templates::templates_store as store;
use crate::features::templates::templates_store::TemplatesStore;
use crate::features::templates::template_list::TemplateList;
use crate::features::templates::template_preview::TemplatePreview;
use crate::features::templates::compliance_checklist::ComplianceChecklist;

/// Templates landing page (FR-600..603).
component TemplatesLanding() {
    let s = use_context::<TemplatesStore>();

    style {
        .templates { display: flex; flex-direction: column; gap: var(--space-4); }
        .templates__content { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
    }

    render {
        <div class="templates" data-testid="templates-landing">
            <TemplateList
                templates={s.templates.clone()}
                on_select={Some(Box::new({ let selected = s.selected_template.clone(); move |t| selected.set(Some(t)) }))}
                loading={s.loading.get()}
            />
            @if let Some(ref tpl) = s.selected_template.get().as_ref() {
                <div class="templates__content">
                    <TemplatePreview
                        template={tpl.clone()}
                        on_copy={Some(Box::new({ let s2 = s.clone(); move || store::copy_selected(&s2) }))}
                    />
                    <ComplianceChecklist
                        items={s.checklist_items.clone()}
                        on_toggle={Some(Box::new({ let s3 = s.clone(); move |id: String| store::toggle_checklist_item(&s3, &id) }))}
                    />
                </div>
            }
        </div>
    }
}
