use rsc_compat::prelude::*;
use crate::features::specs::types::{SpecFile, SpecDirectory, BrdEntry};
use crate::features::specs::service;

/// Central reactive state store for the specs feature.
///
/// Signals:
///   specs              -- Flat list of spec files for the current project (FR-1000)
///   tree               -- Directory tree of specs (FR-1001)
///   selected_file      -- The file the user clicked on for content view
///   file_content       -- Raw content of the selected spec file (FR-1002)
///   brd_entries        -- BRD overview entries (FR-1003)
///   search_query       -- User-entered search/filter text
///   kind_filter        -- Filter by spec kind (empty string = all)
///   loading            -- Whether an async operation is in flight
///   error              -- Most recent error message (cleared on next action)
#[derive(Clone)]
pub struct SpecsStore {
    pub specs: Signal<Vec<SpecFile>>,
    pub tree: Signal<Option<SpecDirectory>>,
    pub selected_file: Signal<Option<SpecFile>>,
    pub file_content: Signal<Option<String>>,
    pub brd_entries: Signal<Vec<BrdEntry>>,
    pub search_query: Signal<String>,
    pub kind_filter: Signal<String>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl SpecsStore {
    /// Create a new SpecsStore with default (empty) signal values.
    pub fn new() -> Self {
        Self {
            specs: signal(Vec::new()),
            tree: signal(None),
            selected_file: signal(None),
            file_content: signal(None),
            brd_entries: signal(Vec::new()),
            search_query: signal(String::new()),
            kind_filter: signal(String::new()),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived: filtered specs based on search_query and kind_filter.
pub fn filtered_specs(store: &SpecsStore) -> Signal<Vec<SpecFile>> {
    let search_query = store.search_query.clone();
    let kind_filter = store.kind_filter.clone();
    let specs = store.specs.clone();
    derived(move || {
        let query = search_query.get().to_lowercase();
        let kind = kind_filter.get();
        specs.get().iter().filter(|f| {
            let matches_query = query.is_empty()
                || f.name.to_lowercase().contains(&query)
                || f.path.to_lowercase().contains(&query);
            let matches_kind = kind.is_empty() || f.kind == kind;
            matches_query && matches_kind
        }).cloned().collect()
    })
}

/// Derived: total file count across all specs.
pub fn file_count(store: &SpecsStore) -> Signal<usize> {
    let specs = store.specs.clone();
    derived(move || specs.get().len())
}

/// Load the flat spec list for the given project (FR-1000).
pub fn load_specs(store: &SpecsStore, project_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    let specs = store.specs.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();
    let project_id_owned = project_id.to_string();

    spawn(async move {
        match service::list_specs(&project_id_owned).await {
            Ok(result) => {
                specs.set(result);
                loading.set(false);
            }
            Err(msg) => {
                error.set(Some(msg));
                loading.set(false);
            }
        }
    });
}

/// Load the spec directory tree for the given project (FR-1001).
pub fn load_tree(store: &SpecsStore, project_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    let tree = store.tree.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();
    let project_id_owned = project_id.to_string();

    spawn(async move {
        match service::get_spec_tree(&project_id_owned).await {
            Ok(result) => {
                tree.set(Some(result));
                loading.set(false);
            }
            Err(msg) => {
                error.set(Some(msg));
                loading.set(false);
            }
        }
    });
}

/// Select a file and load its content (FR-1002).
pub fn select_file(store: &SpecsStore, project_id: &str, file: SpecFile) {
    store.selected_file.set(Some(file.clone()));
    store.file_content.set(None);
    store.error.set(None);

    let file_content = store.file_content.clone();
    let error = store.error.clone();
    let project_id_owned = project_id.to_string();
    let file_path = file.path.clone();

    spawn(async move {
        match service::get_spec_content(&project_id_owned, &file_path).await {
            Ok(content) => {
                file_content.set(Some(content));
            }
            Err(msg) => {
                error.set(Some(msg));
            }
        }
    });
}

/// Load BRD entries for the given project (FR-1003).
pub fn load_brd_entries(store: &SpecsStore, project_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    let brd_entries = store.brd_entries.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();
    let project_id_owned = project_id.to_string();

    spawn(async move {
        match service::list_brd_entries(&project_id_owned).await {
            Ok(entries) => {
                brd_entries.set(entries);
                loading.set(false);
            }
            Err(msg) => {
                error.set(Some(msg));
                loading.set(false);
            }
        }
    });
}

/// Update the search query signal.
pub fn set_search_query(store: &SpecsStore, query: String) {
    store.search_query.set(query);
}

/// Update the kind filter signal.
pub fn set_kind_filter(store: &SpecsStore, kind: String) {
    store.kind_filter.set(kind);
}

/// Clear the selected file and content.
pub fn clear_selection(store: &SpecsStore) {
    store.selected_file.set(None);
    store.file_content.set(None);
}

/// Clear the error signal.
pub fn clear_error(store: &SpecsStore) {
    store.error.set(None);
}
