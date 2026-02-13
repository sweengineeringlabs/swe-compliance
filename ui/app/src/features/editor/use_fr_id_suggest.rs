use rsc_compat::prelude::*;
use crate::features::editor::types::FrIdSuggestion;

/// Hook providing FR ID auto-completion suggestions based on cursor input (FR-902).
///
/// Monitors typed text for "FR-" prefixes and presents matching FR ID suggestions.
/// Components call `update_query` on each keystroke and render the `suggestions`
/// list when `visible` is true. Accepting a suggestion returns the full FR ID string.
pub struct FrIdSuggestHook {
    /// Filtered list of FR ID suggestions matching the current query.
    pub suggestions: Signal<Vec<FrIdSuggestion>>,

    /// Whether the suggestion dropdown should be visible.
    pub visible: Signal<bool>,

    /// Current partial FR-ID query extracted from editor text.
    pub query: Signal<String>,
}

impl FrIdSuggestHook {
    /// Creates a new hook instance with empty/hidden state.
    pub fn new() -> Self {
        Self {
            suggestions: signal(Vec::new()),
            visible: signal(false),
            query: signal(String::new()),
        }
    }

    /// Update the query based on the current text near the cursor.
    /// If the text contains an "FR-" prefix, the suggestion list is filtered
    /// and made visible. Otherwise the list is hidden.
    pub fn update_query(&self, text: &str) {
        if text.contains("FR-") {
            let prefix = text.rsplit("FR-").next().unwrap_or("");
            self.query.set(format!("FR-{}", prefix));
            self.visible.set(true);
            self.filter_suggestions();
        } else {
            self.visible.set(false);
        }
    }

    /// Filter the known FR IDs against the current query prefix.
    fn filter_suggestions(&self) {
        let q = self.query.get().to_lowercase();
        let all = get_known_fr_ids();
        let filtered: Vec<FrIdSuggestion> = all
            .into_iter()
            .filter(|s| s.id.to_lowercase().starts_with(&q))
            .collect();
        self.suggestions.set(filtered);
    }

    /// Accept a suggestion, hide the dropdown, and return the full FR ID.
    pub fn accept(&self, suggestion: &FrIdSuggestion) -> String {
        self.visible.set(false);
        suggestion.id.clone()
    }
}

/// Returns the full catalogue of known FR IDs for auto-completion.
fn get_known_fr_ids() -> Vec<FrIdSuggestion> {
    vec![
        FrIdSuggestion { id: "FR-100".into(), title: "Create Project".into(), category: "Projects".into() },
        FrIdSuggestion { id: "FR-200".into(), title: "Dashboard View".into(), category: "Dashboard".into() },
        FrIdSuggestion { id: "FR-300".into(), title: "Run Scan".into(), category: "Scans".into() },
        FrIdSuggestion { id: "FR-400".into(), title: "View Violations".into(), category: "Violations".into() },
        FrIdSuggestion { id: "FR-500".into(), title: "Parse SRS".into(), category: "Scaffold".into() },
        FrIdSuggestion { id: "FR-600".into(), title: "List Templates".into(), category: "Templates".into() },
        FrIdSuggestion { id: "FR-700".into(), title: "Export Report".into(), category: "Reports".into() },
        FrIdSuggestion { id: "FR-800".into(), title: "AI Chat".into(), category: "AI".into() },
        FrIdSuggestion { id: "FR-900".into(), title: "SRS Editor".into(), category: "Editor".into() },
        FrIdSuggestion { id: "FR-1000".into(), title: "Spec Browser".into(), category: "Specs".into() },
        FrIdSuggestion { id: "FR-1100".into(), title: "Struct Engine".into(), category: "Struct".into() },
    ]
}
