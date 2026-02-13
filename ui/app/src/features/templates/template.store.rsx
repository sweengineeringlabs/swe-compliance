use crate::features::templates::templates_type::{
    TemplateEntry, ChecklistItem, TemplateCopyResult,
};

/// Central reactive store for the templates feature.
/// All template components read from these signals to ensure consistency.
pub struct TemplatesStore {
    /// Full list of available templates fetched from the API.
    pub templates: Signal<Vec<TemplateEntry>>,

    /// Currently selected template for preview and copy operations.
    pub selected_template: Signal<Option<TemplateEntry>>,

    /// Compliance checklist items for the selected template.
    pub checklist_items: Signal<Vec<ChecklistItem>>,

    /// Result of the most recent template copy operation, if any.
    pub copy_result: Signal<Option<TemplateCopyResult>>,

    /// Whether an async operation (list, fetch, copy) is in progress.
    pub loading: Signal<bool>,

    /// Last error message from a failed operation, if any.
    pub error: Signal<Option<String>>,
}

impl TemplatesStore {
    /// Creates a new store with default empty/idle state.
    pub fn new() -> Self {
        Self {
            templates: signal(Vec::new()),
            selected_template: signal(None),
            checklist_items: signal(Vec::new()),
            copy_result: signal(None),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived signal: total number of available templates.
pub fn template_count(store: &TemplatesStore) -> Signal<usize> {
    let templates = store.templates.clone();
    derived(move || templates.get().len())
}

/// Derived signal: templates filtered by a given category.
pub fn filtered_templates(store: &TemplatesStore, category: &str) -> Signal<Vec<TemplateEntry>> {
    let templates = store.templates.clone();
    let cat = category.to_string();
    derived(move || {
        if cat.is_empty() {
            templates.get().clone()
        } else {
            templates.get().iter().filter(|t| t.category == cat).cloned().collect()
        }
    })
}

/// Derived signal: number of checked items in the compliance checklist.
pub fn checked_count(store: &TemplatesStore) -> Signal<usize> {
    let items = store.checklist_items.clone();
    derived(move || items.get().iter().filter(|i| i.checked).count())
}

/// Derived signal: total number of checklist items.
pub fn checklist_total(store: &TemplatesStore) -> Signal<usize> {
    let items = store.checklist_items.clone();
    derived(move || items.get().len())
}

/// Derived signal: whether a template is currently selected.
pub fn has_selection(store: &TemplatesStore) -> Signal<bool> {
    let selected = store.selected_template.clone();
    derived(move || selected.get().is_some())
}

/// Select a template and clear any previous copy result.
pub fn select_template(store: &TemplatesStore, template: TemplateEntry) {
    store.selected_template.set(Some(template));
    store.copy_result.set(None);
}

/// Toggle the checked state of a checklist item by its id.
pub fn toggle_checklist_item(store: &TemplatesStore, item_id: &str) {
    let mut items = store.checklist_items.get().clone();
    if let Some(item) = items.iter_mut().find(|i| i.id == item_id) {
        item.checked = !item.checked;
    }
    store.checklist_items.set(items);
}

/// Reset the store to its initial state, clearing all data and selections.
pub fn reset_store(store: &TemplatesStore) {
    store.templates.set(Vec::new());
    store.selected_template.set(None);
    store.checklist_items.set(Vec::new());
    store.copy_result.set(None);
    store.loading.set(false);
    store.error.set(None);
}
