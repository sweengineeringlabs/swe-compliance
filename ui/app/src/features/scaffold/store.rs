use rsc_compat::prelude::*;
use crate::features::scaffold::types::{
    ParsedDomain, ScaffoldResult,
};
use crate::features::scaffold::service;

/// Central reactive store for the scaffold feature.
/// All scaffold components read from these signals to ensure consistency.
#[derive(Clone)]
pub struct ScaffoldStore {
    /// Raw SRS markdown content pasted or uploaded by the user.
    pub srs_content: Signal<String>,

    /// Parsed domain sections extracted from the SRS content.
    pub parsed_domains: Signal<Vec<ParsedDomain>>,

    /// Result of the most recent scaffold execution, if any.
    pub scaffold_result: Signal<Option<ScaffoldResult>>,

    /// Currently selected SDLC phases for generation.
    pub selected_phases: Signal<Vec<String>>,

    /// Currently selected file types for generation.
    pub selected_file_types: Signal<Vec<String>>,

    /// Whether an async operation (parse or scaffold) is in progress.
    pub loading: Signal<bool>,

    /// Last error message from a failed operation, if any.
    pub error: Signal<Option<String>>,
}

impl ScaffoldStore {
    /// Creates a new store with default empty/idle state.
    /// All phases and file types start unselected.
    pub fn new() -> Self {
        Self {
            srs_content: signal(String::new()),
            parsed_domains: signal(Vec::new()),
            scaffold_result: signal(None),
            selected_phases: signal(Vec::new()),
            selected_file_types: signal(Vec::new()),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Derived signal: total number of parsed domains.
pub fn domain_count(store: &ScaffoldStore) -> Signal<usize> {
    let domains = store.parsed_domains.clone();
    derived(move || domains.get().len())
}

/// Derived signal: total number of requirements across all parsed domains.
pub fn total_requirement_count(store: &ScaffoldStore) -> Signal<usize> {
    let domains = store.parsed_domains.clone();
    derived(move || {
        domains.get().iter().map(|d| d.requirements.len()).sum()
    })
}

/// Derived signal: whether the SRS has been parsed and domains are available.
pub fn has_parsed_data(store: &ScaffoldStore) -> Signal<bool> {
    let domains = store.parsed_domains.clone();
    derived(move || !domains.get().is_empty())
}

/// Derived signal: whether the scaffold can be executed.
/// Requires parsed data, at least one phase, and at least one file type.
pub fn can_execute(store: &ScaffoldStore) -> Signal<bool> {
    let domains = store.parsed_domains.clone();
    let phases = store.selected_phases.clone();
    let file_types = store.selected_file_types.clone();
    let loading = store.loading.clone();
    derived(move || {
        !domains.get().is_empty()
            && !phases.get().is_empty()
            && !file_types.get().is_empty()
            && !loading.get()
    })
}

/// Derived signal: summary counts from the latest scaffold result.
/// Returns (created_count, skipped_count) or (0, 0) if no result.
pub fn result_counts(store: &ScaffoldStore) -> Signal<(usize, usize)> {
    let result = store.scaffold_result.clone();
    derived(move || {
        match result.get().as_ref() {
            Some(r) => (r.created.len(), r.skipped.len()),
            None => (0, 0),
        }
    })
}

/// Toggle a phase in the selected_phases list.
/// Adds it if missing, removes it if present.
pub fn toggle_phase(store: &ScaffoldStore, phase: &str) {
    let mut phases = store.selected_phases.get().clone();
    if let Some(pos) = phases.iter().position(|p| p == phase) {
        phases.remove(pos);
    } else {
        phases.push(phase.to_string());
    }
    store.selected_phases.set(phases);
}

/// Toggle a file type in the selected_file_types list.
/// Adds it if missing, removes it if present.
pub fn toggle_file_type(store: &ScaffoldStore, file_type: &str) {
    let mut types = store.selected_file_types.get().clone();
    if let Some(pos) = types.iter().position(|t| t == file_type) {
        types.remove(pos);
    } else {
        types.push(file_type.to_string());
    }
    store.selected_file_types.set(types);
}

/// Parse the current SRS content and populate parsed_domains (FR-500).
pub fn parse(store: &ScaffoldStore) {
    store.loading.set(true);
    store.error.set(None);

    let content = store.srs_content.get().clone();
    let parsed_domains = store.parsed_domains.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();

    spawn(async move {
        match service::parse_srs(&content).await {
            Ok(domains) => {
                parsed_domains.set(domains);
                loading.set(false);
            }
            Err(msg) => {
                error.set(Some(msg));
                loading.set(false);
            }
        }
    });
}

/// Execute scaffolding with current selections (FR-502).
pub fn execute(store: &ScaffoldStore) {
    store.loading.set(true);
    store.error.set(None);

    let phases = store.selected_phases.get().clone();
    let file_types = store.selected_file_types.get().clone();
    let scaffold_result = store.scaffold_result.clone();
    let loading = store.loading.clone();
    let error = store.error.clone();

    spawn(async move {
        match service::execute_scaffold(".", ".", &phases, &file_types, false).await {
            Ok(result) => {
                scaffold_result.set(Some(result));
                loading.set(false);
            }
            Err(msg) => {
                error.set(Some(msg));
                loading.set(false);
            }
        }
    });
}

/// Reset the store to its initial state, clearing all data and selections.
pub fn reset_store(store: &ScaffoldStore) {
    store.srs_content.set(String::new());
    store.parsed_domains.set(Vec::new());
    store.scaffold_result.set(None);
    store.selected_phases.set(Vec::new());
    store.selected_file_types.set(Vec::new());
    store.loading.set(false);
    store.error.set(None);
}
