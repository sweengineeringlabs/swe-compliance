use rsc_ui::prelude::*;
use crate::features::templates::templates_type::ChecklistItem;

/// Compliance checklist with toggleable items and completion summary (FR-603).
component ComplianceChecklist(
    items: Signal<Vec<ChecklistItem>>,
    on_toggle: Fn(String),
) {
    let checked_count = derived({
        let items = items.clone();
        move || items.get().iter().filter(|i| i.checked).count()
    });

    let total_count = derived({
        let items = items.clone();
        move || items.get().len()
    });

    style {
        .compliance-checklist { display: flex; flex-direction: column; gap: var(--space-3); }
        .compliance-checklist__summary {
            font-size: var(--font-size-sm);
            color: var(--color-text-muted);
            font-weight: 600;
        }
        .compliance-checklist__items { display: flex; flex-direction: column; gap: var(--space-2); }
    }

    render {
        <Card data-testid="compliance-checklist">
            <div class="compliance-checklist">
                <h3 data-testid="compliance-checklist-heading">"Compliance Checklist"</h3>
                <span class="compliance-checklist__summary" data-testid="compliance-checklist-summary">
                    {format!("{} / {} completed", checked_count.get(), total_count.get())}
                </span>
                @if items.get().is_empty() {
                    <p data-testid="compliance-checklist-empty">"No checklist items for this template."</p>
                } @else {
                    <div class="compliance-checklist__items">
                        @for item in items.get().iter() {
                            <Checkbox
                                label={&item.label}
                                checked={item.checked}
                                on:change={let id = item.id.clone(); move |_v: String| on_toggle(id.clone())}
                                data-testid={format!("checklist-item-{}", item.id)}
                            />
                        }
                    </div>
                }
            </div>
        </Card>
    }
}
