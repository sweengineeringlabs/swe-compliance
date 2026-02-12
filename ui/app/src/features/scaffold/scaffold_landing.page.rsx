use rsc_ui::prelude::*;
use crate::features::scaffold::scaffold_store as store;
use crate::features::scaffold::srs_upload::SrsUpload;
use crate::features::scaffold::scaffold_preview::ScaffoldPreview;
use crate::features::scaffold::phase_filter::PhaseFilter;
use crate::features::scaffold::scaffold_progress::ScaffoldProgress;

/// Scaffolding interface page (FR-500..504).
component ScaffoldLanding() {
    style {
        .scaffold { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="scaffold" data-testid="scaffold-landing">
            <SrsUpload
                content={store::srs_content.clone()}
                on_parse={|| store::parse()}
                loading={store::loading.get()}
            />
            @if !store::parsed_domains.get().is_empty() {
                <ScaffoldPreview domains={store::parsed_domains.clone()} />
                <PhaseFilter phases={store::selected_phases.clone()} file_types={store::selected_file_types.clone()} />
                <Button label="Execute Scaffold" variant="primary" on:click={|| store::execute()} data-testid="scaffold-execute-btn" />
            }
            @if let Some(ref result) = store::scaffold_result.get() {
                <ScaffoldProgress result={result.clone()} />
            }
        </div>
    }
}
