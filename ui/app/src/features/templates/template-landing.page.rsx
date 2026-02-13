use rsc_ui::prelude::*;
use crate::features::templates::templates_store as store;
use crate::features::templates::template_list::TemplateList;
use crate::features::templates::template_preview::TemplatePreview;
use crate::features::templates::compliance_checklist::ComplianceChecklist;

/// Templates landing page (FR-600..603).
component TemplatesLanding() {
    style {
        .templates { display: flex; flex-direction: column; gap: var(--space-4); }
        .templates__content { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
    }

    render {
        <div class="templates" data-testid="templates-landing">
            <TemplateList
                templates={store::templates.clone()}
                on_select={|t| store::selected_template.set(Some(t))}
                loading={store::loading.get()}
            />
            @if let Some(ref tpl) = store::selected_template.get() {
                <div class="templates__content">
                    <TemplatePreview
                        template={tpl.clone()}
                        on_copy={|| store::copy_selected()}
                    />
                    <ComplianceChecklist
                        items={store::checklist_items.clone()}
                        on_toggle={|id| store::toggle_checklist_item(&store::STORE, &id)}
                    />
                </div>
            }
        </div>
    }
}
