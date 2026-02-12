use crate::features::editor::editor_type::ValidationResult;
use crate::features::editor::editor_service;

/// Central reactive store for the editor feature.
/// All editor components read from these signals to ensure consistency.
pub struct EditorStore {
    /// Raw SRS markdown content being edited by the user.
    pub content: Signal<String>,

    /// Project ID currently associated with the editor, if any.
    pub project_id: Signal<Option<String>>,

    /// Latest validation result from the backend, if available.
    pub validation: Signal<Option<ValidationResult>>,

    /// Whether the current content has been saved successfully.
    pub saved: Signal<bool>,

    /// Whether the content has unsaved modifications.
    pub dirty: Signal<bool>,

    /// Whether an async operation (validate, load, save) is in progress.
    pub loading: Signal<bool>,

    /// Last error message from a failed operation, if any.
    pub error: Signal<Option<String>>,
}

impl EditorStore {
    /// Creates a new store with default empty/idle state.
    pub fn new() -> Self {
        Self {
            content: signal(String::new()),
            project_id: signal(None),
            validation: signal(None),
            saved: signal(false),
            dirty: signal(false),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived signal: whether the latest validation passed (FR-901).
pub fn is_valid(store: &EditorStore) -> Signal<bool> {
    let validation = store.validation.clone();
    derived(move || {
        validation.get().as_ref().map(|v| v.valid).unwrap_or(false)
    })
}

/// Derived signal: number of validation errors.
pub fn error_count(store: &EditorStore) -> Signal<usize> {
    let validation = store.validation.clone();
    derived(move || {
        validation.get().as_ref().map(|v| v.errors.len()).unwrap_or(0)
    })
}

/// Derived signal: whether the latest validation produced warnings.
pub fn has_warnings(store: &EditorStore) -> Signal<bool> {
    let validation = store.validation.clone();
    derived(move || {
        validation.get().as_ref().map(|v| !v.warnings.is_empty()).unwrap_or(false)
    })
}

/// Derived signal: whether the document can be saved (FR-903).
/// Requires a project ID, dirty content, and no in-flight operation.
pub fn can_save(store: &EditorStore) -> Signal<bool> {
    let project_id = store.project_id.clone();
    let dirty = store.dirty.clone();
    let loading = store.loading.clone();
    derived(move || {
        project_id.get().is_some() && dirty.get() && !loading.get()
    })
}

/// Validate the current editor content against the backend (FR-901).
pub async fn validate(store: &EditorStore) {
    store.loading.set(true);
    store.error.set(None);

    let content = store.content.get().clone();
    match editor_service::validate_srs(&content).await {
        Ok(result) => {
            store.validation.set(Some(result));
        }
        Err(msg) => {
            store.error.set(Some(msg));
        }
    }

    store.loading.set(false);
}

/// Load an SRS document for the given project into the editor (FR-903).
pub async fn load(store: &EditorStore, project_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    match editor_service::load_srs(project_id).await {
        Ok(doc) => {
            store.project_id.set(Some(doc.project_id.clone()));
            store.content.set(doc.content);
            store.dirty.set(false);
            store.saved.set(true);
            store.validation.set(None);
        }
        Err(msg) => {
            store.error.set(Some(msg));
        }
    }

    store.loading.set(false);
}

/// Save the current editor content for the active project (FR-903).
pub async fn save(store: &EditorStore) {
    let pid = match store.project_id.get().clone() {
        Some(id) => id,
        None => {
            store.error.set(Some("no project selected".into()));
            return;
        }
    };

    store.loading.set(true);
    store.error.set(None);

    let content = store.content.get().clone();
    match editor_service::save_srs(&pid, &content).await {
        Ok(_doc) => {
            store.dirty.set(false);
            store.saved.set(true);
        }
        Err(msg) => {
            store.error.set(Some(msg));
        }
    }

    store.loading.set(false);
}

/// Reset the store to its initial state, clearing all data.
pub fn reset_store(store: &EditorStore) {
    store.content.set(String::new());
    store.project_id.set(None);
    store.validation.set(None);
    store.saved.set(false);
    store.dirty.set(false);
    store.loading.set(false);
    store.error.set(None);
}
