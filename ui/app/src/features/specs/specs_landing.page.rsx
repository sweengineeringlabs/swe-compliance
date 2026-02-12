use rsc_ui::prelude::*;
use crate::features::specs::specs_store as store;
use crate::features::specs::spec_tree::SpecTree;
use crate::features::specs::spec_content::SpecContent;
use crate::features::specs::brd_overview::BrdOverview;

/// Specs landing page combining tree browser, content viewer, and BRD overview
/// (FR-1000..1003).
component SpecsLanding() {
    effect(|| {
        store::load_tree();
        store::load_specs();
        store::load_brd_entries();
    });

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
                    value={store::search_query.get()}
                    on:input={|val| store::set_search_query(val)}
                    data-testid="specs-search-input"
                />
            </div>
            <div class="specs__browser" data-testid="specs-browser">
                <SpecTree
                    tree={store::tree.clone()}
                    on_select={|f| store::select_file(f)}
                />
                @if let Some(ref content) = store::file_content.get() {
                    <SpecContent
                        file={store::selected_file.clone()}
                        content={content.clone()}
                    />
                }
            </div>
            <BrdOverview entries={store::brd_entries.clone()} />
        </div>
    }
}
