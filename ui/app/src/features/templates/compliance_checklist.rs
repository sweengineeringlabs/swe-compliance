use rsc_compat::prelude::*;
use crate::features::templates::types::ChecklistItem;

/// Compliance checklist with toggleable items and completion summary (FR-603).
/// Note: Checkbox component replaced with native input[type=checkbox] elements.
#[component]
pub fn compliance_checklist(
    items: Signal<Vec<ChecklistItem>>,
    on_toggle: Option<Box<dyn Fn(String)>>,
) -> View {
    let checked_count = derived({
        let items = items.clone();
        move || items.get().iter().filter(|i| i.checked).count()
    });

    let total_count = derived({
        let items = items.clone();
        move || items.get().len()
    });

    view! {
        style {
            .compliance-checklist { display: flex; flex-direction: column; gap: var(--space-3); }
            .compliance-checklist__summary {
                font-size: var(--font-size-sm);
                color: var(--color-text-muted);
                font-weight: 600;
            }
            .compliance-checklist__items { display: flex; flex-direction: column; gap: var(--space-2); }
            .checkbox { display: flex; align-items: center; gap: var(--space-2); cursor: pointer; }
            .checkbox input { cursor: pointer; }
        }
        Card(data-testid="compliance-checklist") {
            div(class="compliance-checklist") {
                h3(data-testid="compliance-checklist-heading") { "Compliance Checklist" }
                span(class="compliance-checklist__summary", data-testid="compliance-checklist-summary") {
                    (format!("{} / {} completed", checked_count.get(), total_count.get()))
                }
                (if items.get().is_empty() {
                    view! {
                        p(data-testid="compliance-checklist-empty") { "No checklist items for this template." }
                    }
                } else {
                    view! {
                        div(class="compliance-checklist__items") {
                            Keyed(
                                iterable=items.get(),
                                key=|item| item.id.clone(),
                                view={
                                    let on_toggle = on_toggle.clone();
                                    move |item| {
                                        let id = item.id.clone();
                                        let on_toggle = on_toggle.clone();
                                        view! {
                                            label(
                                                class="checkbox",
                                                data-testid=format!("checklist-item-{}", item.id),
                                            ) {
                                                input(
                                                    type="checkbox",
                                                    checked=item.checked,
                                                    on:change=move |_v: String| { if let Some(ref cb) = on_toggle { cb(id.clone()) } },
                                                )
                                                (item.label.clone())
                                            }
                                        }
                                    }
                                },
                            )
                        }
                    }
                })
            }
        }
    }
}
