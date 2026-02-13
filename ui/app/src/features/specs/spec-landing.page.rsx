use rsc_ui::prelude::*;
use crate::features::specs::specs_store as store;
use crate::features::specs::specs_store::SpecsStore;
use crate::features::specs::spec_tree::SpecTree;
use crate::features::specs::spec_content::SpecContent;
use crate::features::specs::brd_overview::BrdOverview;

/// Specs landing page combining tree browser, content viewer, and BRD overview
/// (FR-1000..1003).
component SpecsLanding() {
    let s = use_context::<SpecsStore>();

    // Load data on mount.
    { let s2 = s.clone(); effect(move || {
        store::load_tree(&s2, "default");
        store::load_specs(&s2, "default");
        store::load_brd_entries(&s2, "default");
    }); }

    style {
        .specs { display: flex; flex-direction: column; gap: var(--space-4); }
        .specs__browser { display: grid; grid-template-columns: 300px 1fr; gap: var(--space-4); min-height: 500px; }
        .specs__search { display: flex; align-items: center; gap: var(--space-3); }
    }

    render {
        <div class="specs" data-testid="specs-landing">
            <div class="specs__search" data-testid="specs-search">
                <Input
                    placeholder="Search specs..."
                    value={s.search_query.get()}
                    on:input={{ let s2 = s.clone(); move |v: String| store::set_search_query(&s2, v) }}
                    data-testid="specs-search-input"
                />
            </div>
            <div class="specs__browser" data-testid="specs-browser">
                <SpecTree
                    tree={s.tree.clone()}
                    on_select={Some(Box::new({ let s2 = s.clone(); move |f| store::select_file(&s2, "default", f) }))}
                />
                @if let Some(ref content) = s.file_content.get().as_ref() {
                    <SpecContent
                        file={s.selected_file.clone()}
                        content={content.clone()}
                    />
                }
            </div>
            <BrdOverview entries={s.brd_entries.clone()} />
        </div>
    }
}
