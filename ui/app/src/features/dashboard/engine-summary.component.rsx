use rsc_ui::prelude::*;

/// Tabs toggling between doc-engine (128 checks) and struct-engine (44 checks) views (FR-203).
component EngineSummary(categories: Signal<Vec<(String, u32, u32, u32)>>) {
    let active_tab = signal("doc-engine".to_string());

    style {
        .engine-summary { width: 100%; }
        .engine-summary__title { font-size: var(--font-size-md); font-weight: 600; margin-bottom: var(--space-3); }
        .engine-summary__counts { display: flex; gap: var(--space-4); margin-top: var(--space-3); }
    }

    render {
        <Card data-testid="engine-summary">
            <h3 class="engine-summary__title">"Engine Summary"</h3>
            <Tabs data-testid="engine-tabs">
                <Tab
                    label="doc-engine (128)"
                    active={active_tab.get() == "doc-engine"}
                    on:click={|| active_tab.set("doc-engine".into())}
                    data-testid="tab-doc-engine"
                />
                <Tab
                    label="struct-engine (44)"
                    active={active_tab.get() == "struct-engine"}
                    on:click={|| active_tab.set("struct-engine".into())}
                    data-testid="tab-struct-engine"
                />
            </Tabs>
            <div class="engine-summary__counts" data-testid="engine-summary-content">
                @if active_tab.get() == "doc-engine" {
                    <Badge variant="success" data-testid="doc-total">"128 checks"</Badge>
                } @else {
                    <Badge variant="success" data-testid="struct-total">"44 checks"</Badge>
                }
            </div>
        </Card>
    }
}
