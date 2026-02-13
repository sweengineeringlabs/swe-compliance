use rsc_compat::prelude::*;
use crate::features::dashboard::types::CategoryBreakdown;

/// Tabs toggling between doc-engine (128 checks) and struct-engine (44 checks) views (FR-203).
#[component]
pub fn engine_summary(categories: Signal<Vec<CategoryBreakdown>>) -> View {
    let active_tab = signal("doc-engine".to_string());
    let active_tab_doc = active_tab.clone();
    let active_tab_struct = active_tab.clone();

    view! {
        style {
            .engine-summary { width: 100%; }
            .engine-summary__title { font-size: var(--font-size-md); font-weight: 600; margin-bottom: var(--space-3); }
            .engine-summary__counts { display: flex; gap: var(--space-4); margin-top: var(--space-3); }
        }

        <Card data-testid="engine-summary">
            <h3 class="engine-summary__title">"Engine Summary"</h3>
            <Tabs data-testid="engine-tabs">
                <Tab
                    label="doc-engine (128)"
                    active={active_tab.get() == "doc-engine"}
                    on:click={move || active_tab_doc.set("doc-engine".into())}
                    data-testid="tab-doc-engine"
                />
                <Tab
                    label="struct-engine (44)"
                    active={active_tab.get() == "struct-engine"}
                    on:click={move || active_tab_struct.set("struct-engine".into())}
                    data-testid="tab-struct-engine"
                />
            </Tabs>
            <div class="engine-summary__counts" data-testid="engine-summary-content">
                if active_tab.get() == "doc-engine" {
                    <Badge variant="success" data-testid="doc-total">"128 checks"</Badge>
                } else {
                    <Badge variant="success" data-testid="struct-total">"44 checks"</Badge>
                }
            </div>
        </Card>
    }
}
