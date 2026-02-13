use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::specs::store::{self, SpecsStore};
use crate::features::specs::spec_tree::spec_tree;
use crate::features::specs::spec_content::spec_content;
use crate::features::specs::brd_overview::brd_overview;

/// Specs landing page combining tree browser, content viewer, and BRD overview
/// (FR-1000..1003).
#[component]
pub fn specs_landing() -> View {
    let s = use_context::<SpecsStore>();

    // Load data on mount.
    // The project_id would typically come from a route param or parent context.
    // For now, we use a placeholder approach similar to the scans landing.
    {
        let s2 = s.clone();
        effect(move || {
            // Load tree, specs, and BRD entries with a default project_id.
            store::load_tree(&s2, "default");
            store::load_specs(&s2, "default");
            store::load_brd_entries(&s2, "default");
        });
    }

    let search_query = s.search_query.clone();
    let s_input = s.clone();
    let tree = s.tree.clone();
    let s_select = s.clone();
    let file_content = s.file_content.clone();
    let selected_file = s.selected_file.clone();
    let brd_entries = s.brd_entries.clone();

    let tree_view = spec_tree(
        tree,
        Some(Rc::new({
            let s2 = s_select.clone();
            move |f| store::select_file(&s2, "default", f)
        })),
    );
    let content_view = if let Some(ref content) = file_content.get().as_ref() {
        spec_content(selected_file.clone(), content.to_string())
    } else {
        view! {}
    };
    let brd_view = brd_overview(brd_entries);

    view! {
        style {
            .specs { display: flex; flex-direction: column; gap: var(--space-4); }
            .specs__browser { display: grid; grid-template-columns: 300px 1fr; gap: var(--space-4); min-height: 500px; }
            .specs__search { display: flex; align-items: center; gap: var(--space-3); }
        }
        <div class="specs" data-testid="specs-landing">
            <div class="specs__search" data-testid="specs-search">
                <input
                    type="text"
                    class="input"
                    placeholder="Search specs..."
                    value={search_query.get()}
                    on:input={
                        let s2 = s_input.clone();
                        move |v: String| store::set_search_query(&s2, v)
                    }
                    data-testid="specs-search-input"
                />
            </div>
            <div class="specs__browser" data-testid="specs-browser">
                {tree_view}
                {content_view}
            </div>
            {brd_view}
        </div>
    }
}
