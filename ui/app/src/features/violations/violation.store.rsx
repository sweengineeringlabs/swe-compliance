use rsc_ui::prelude::*;
use crate::features::violations::violations_type::{
    ViolationEntry, ViolationFilter, SortField, SortDirection,
    severity_weight,
};

/// Reactive store for the violations feature.
/// All state is held in signals; the filtered/sorted view is a derived computation.
pub struct ViolationsStore {
    /// Raw violation entries fetched from the API.
    pub all_violations: Signal<Vec<ViolationEntry>>,
    /// Current filter state (category, severities, search text).
    pub filter: Signal<ViolationFilter>,
    /// Active sort column.
    pub sort_field: Signal<SortField>,
    /// Active sort direction.
    pub sort_direction: Signal<SortDirection>,
    /// Scan ID currently loaded into the browser (None = no scan selected).
    pub selected_scan_id: Signal<Option<String>>,
    /// True while an API request is in flight.
    pub loading: Signal<bool>,
    /// Derived: filtered + sorted violations ready for display.
    pub filtered_violations: Derived<Vec<ViolationEntry>>,
    /// Current page index (0-based) for pagination.
    pub current_page: Signal<u32>,
    /// Number of rows per page.
    pub page_size: Signal<u32>,
}

/// Default page size for the violation table.
const DEFAULT_PAGE_SIZE: u32 = 25;

impl ViolationsStore {
    /// Create a new store with default initial state.
    pub fn new() -> Self {
        let all_violations = signal(Vec::<ViolationEntry>::new());
        let filter = signal(ViolationFilter::default());
        let sort_field = signal(SortField::default());
        let sort_direction = signal(SortDirection::default());
        let selected_scan_id = signal(Option::<String>::None);
        let loading = signal(false);
        let current_page = signal(0_u32);
        let page_size = signal(DEFAULT_PAGE_SIZE);

        let filtered_violations = derived(|| {
            let all = all_violations.get();
            let f = filter.get();
            let sf = sort_field.get();
            let sd = sort_direction.get();

            // --- Filter ---
            let search_lower = f.search.to_lowercase();

            let mut result: Vec<ViolationEntry> = all
                .iter()
                .filter(|v| {
                    // Category filter
                    if let Some(ref cat) = f.category {
                        if &v.category != cat {
                            return false;
                        }
                    }

                    // Severity filter
                    if !f.severities.contains(&v.severity) {
                        return false;
                    }

                    // Free-text search on description and message
                    if !search_lower.is_empty() {
                        let in_desc = v.description.to_lowercase().contains(&search_lower);
                        let in_msg = v.message.to_lowercase().contains(&search_lower);
                        if !in_desc && !in_msg {
                            return false;
                        }
                    }

                    true
                })
                .cloned()
                .collect();

            // --- Sort ---
            result.sort_by(|a, b| {
                let ordering = match sf {
                    SortField::CheckId => a.check_id.cmp(&b.check_id),
                    SortField::Severity => {
                        let cmp = severity_weight(&a.severity)
                            .cmp(&severity_weight(&b.severity));
                        // Secondary sort by check_id for stable ordering
                        if cmp == std::cmp::Ordering::Equal {
                            a.check_id.cmp(&b.check_id)
                        } else {
                            cmp
                        }
                    }
                    SortField::Category => {
                        let cmp = a.category.cmp(&b.category);
                        if cmp == std::cmp::Ordering::Equal {
                            a.check_id.cmp(&b.check_id)
                        } else {
                            cmp
                        }
                    }
                    SortField::FilePath => {
                        let a_path = a.file_path.as_deref().unwrap_or("");
                        let b_path = b.file_path.as_deref().unwrap_or("");
                        let cmp = a_path.cmp(b_path);
                        if cmp == std::cmp::Ordering::Equal {
                            a.check_id.cmp(&b.check_id)
                        } else {
                            cmp
                        }
                    }
                };

                match sd {
                    SortDirection::Asc => ordering,
                    SortDirection::Desc => ordering.reverse(),
                }
            });

            result
        });

        Self {
            all_violations,
            filter,
            sort_field,
            sort_direction,
            selected_scan_id,
            loading,
            filtered_violations,
            current_page,
            page_size,
        }
    }

    /// Toggle sort: if clicking the same column, flip direction; otherwise set new column Desc.
    pub fn toggle_sort(&self, field: SortField) {
        if self.sort_field.get() == field {
            let new_dir = match self.sort_direction.get() {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            };
            self.sort_direction.set(new_dir);
        } else {
            self.sort_field.set(field);
            self.sort_direction.set(SortDirection::Desc);
        }
        // Reset to first page when sort changes
        self.current_page.set(0);
    }

    /// Update the category filter. Pass None to clear.
    pub fn set_category(&self, category: Option<String>) {
        let mut f = self.filter.get().clone();
        f.category = category;
        self.filter.set(f);
        self.current_page.set(0);
    }

    /// Toggle a severity level in the filter.
    pub fn toggle_severity(&self, severity: &str) {
        let mut f = self.filter.get().clone();
        if let Some(pos) = f.severities.iter().position(|s| s == severity) {
            f.severities.remove(pos);
        } else {
            f.severities.push(severity.into());
        }
        self.filter.set(f);
        self.current_page.set(0);
    }

    /// Update the free-text search term.
    pub fn set_search(&self, text: String) {
        let mut f = self.filter.get().clone();
        f.search = text;
        self.filter.set(f);
        self.current_page.set(0);
    }

    /// Total number of pages based on filtered results and page size.
    pub fn total_pages(&self) -> u32 {
        let count = self.filtered_violations.get().len() as u32;
        let size = self.page_size.get();
        if size == 0 {
            1
        } else {
            (count + size - 1) / size
        }
    }

    /// Get the current page slice of filtered violations.
    pub fn paged_violations(&self) -> Vec<ViolationEntry> {
        let all = self.filtered_violations.get();
        let size = self.page_size.get() as usize;
        let start = (self.current_page.get() as usize) * size;
        all.into_iter().skip(start).take(size).collect()
    }

    /// Navigate to a specific page (clamped to valid range).
    pub fn go_to_page(&self, page: u32) {
        let max = self.total_pages().saturating_sub(1);
        self.current_page.set(page.min(max));
    }

    /// Clear all filters and reset to defaults.
    pub fn reset_filters(&self) {
        self.filter.set(ViolationFilter::default());
        self.sort_field.set(SortField::default());
        self.sort_direction.set(SortDirection::default());
        self.current_page.set(0);
    }
}

/// Provide the violations store via context for child components.
pub fn provide_violations_store() -> ViolationsStore {
    let store = ViolationsStore::new();
    provide_context(store.clone());
    store
}

/// Consume the violations store from context.
pub fn use_violations_store() -> ViolationsStore {
    use_context::<ViolationsStore>()
}
