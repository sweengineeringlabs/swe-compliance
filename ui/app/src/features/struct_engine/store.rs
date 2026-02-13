use rsc_compat::prelude::*;
use crate::features::struct_engine::types::{StructCheck, CrateNode};
use crate::features::struct_engine::service;

/// Central reactive state store for the struct engine feature.
///
/// Signals:
///   checks              -- All struct engine check results from the latest scan
///   crate_tree          -- Parsed crate layout tree (None until loaded)
///   project_kind        -- Project kind classification string (None until loaded)
///   category_filter     -- Active category filter (None = show all)
///   status_filter       -- Active status filter (None = show all)
///   loading             -- Whether an async operation is in flight
///   error               -- Most recent error message (cleared on next action)
#[derive(Clone)]
pub struct StructEngineStore {
    pub checks: Signal<Vec<StructCheck>>,
    pub crate_tree: Signal<Option<CrateNode>>,
    pub project_kind: Signal<Option<String>>,
    pub category_filter: Signal<Option<String>>,
    pub status_filter: Signal<Option<String>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl StructEngineStore {
    /// Create a new StructEngineStore with default (empty) signal values.
    pub fn new() -> Self {
        Self {
            checks: signal(Vec::new()),
            crate_tree: signal(None),
            project_kind: signal(None),
            category_filter: signal(None),
            status_filter: signal(None),
            loading: signal(false),
            error: signal(None),
        }
    }

    /// Derived: checks filtered by category and status.
    pub fn filtered_checks(&self) -> Vec<StructCheck> {
        let all = self.checks.get();
        let cat = self.category_filter.get();
        let stat = self.status_filter.get();

        all.iter()
            .filter(|c| {
                if let Some(ref cat_val) = cat {
                    if !cat_val.is_empty() && c.category != *cat_val {
                        return false;
                    }
                }
                if let Some(ref stat_val) = stat {
                    if !stat_val.is_empty() && c.status != *stat_val {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    /// Derived: count of checks with status "pass".
    pub fn pass_count(&self) -> usize {
        self.checks.get().iter().filter(|c| c.status == "pass").count()
    }

    /// Derived: count of checks with status "fail".
    pub fn fail_count(&self) -> usize {
        self.checks.get().iter().filter(|c| c.status == "fail").count()
    }

    /// Derived: count of checks with status "skip".
    pub fn skip_count(&self) -> usize {
        self.checks.get().iter().filter(|c| c.status == "skip").count()
    }

    /// Derived: unique category names from current checks.
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.checks
            .get()
            .iter()
            .map(|c| c.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Load struct engine scan results for the given scan ID (FR-1100).
    /// Clears any prior error on invocation; sets error on failure.
    pub fn load_results(&self, scan_id: &str) {
        self.loading.set(true);
        self.error.set(None);

        let checks = self.checks.clone();
        let loading = self.loading.clone();
        let error = self.error.clone();
        let scan_id_owned = scan_id.to_string();

        spawn(async move {
            match service::get_struct_results(&scan_id_owned).await {
                Ok(results) => {
                    checks.set(results);
                    loading.set(false);
                }
                Err(api_error) => {
                    error.set(Some(api_error.message));
                    loading.set(false);
                }
            }
        });
    }

    /// Load crate layout tree for the given project (FR-1101).
    pub fn load_crate_layout(&self, project_id: &str) {
        self.loading.set(true);
        self.error.set(None);

        let crate_tree = self.crate_tree.clone();
        let loading = self.loading.clone();
        let error = self.error.clone();
        let project_id_owned = project_id.to_string();

        spawn(async move {
            match service::get_crate_layout(&project_id_owned).await {
                Ok(tree) => {
                    crate_tree.set(Some(tree));
                    loading.set(false);
                }
                Err(api_error) => {
                    error.set(Some(api_error.message));
                    loading.set(false);
                }
            }
        });
    }

    /// Load project kind classification (FR-1102).
    pub fn load_project_kind(&self, project_id: &str) {
        let project_kind = self.project_kind.clone();
        let error = self.error.clone();
        let project_id_owned = project_id.to_string();

        spawn(async move {
            match service::get_project_kind(&project_id_owned).await {
                Ok(kind) => {
                    project_kind.set(Some(kind));
                }
                Err(api_error) => {
                    error.set(Some(api_error.message));
                }
            }
        });
    }

    /// Set category filter.
    pub fn set_category_filter(&self, value: Option<String>) {
        self.category_filter.set(value);
    }

    /// Set status filter.
    pub fn set_status_filter(&self, value: Option<String>) {
        self.status_filter.set(value);
    }

    /// Clear the error signal.
    pub fn clear_error(&self) {
        self.error.set(None);
    }
}
