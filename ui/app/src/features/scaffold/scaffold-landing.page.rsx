use rsc_ui::prelude::*;
use crate::features::scaffold::scaffold_store as store;
use crate::features::scaffold::scaffold_store::ScaffoldStore;
use crate::features::scaffold::srs_upload::SrsUpload;
use crate::features::scaffold::scaffold_preview::ScaffoldPreview;
use crate::features::scaffold::phase_filter::PhaseFilter;
use crate::features::scaffold::scaffold_progress::ScaffoldProgress;

/// Scaffolding interface page (FR-500..504).
component ScaffoldLanding() {
    let s = use_context::<ScaffoldStore>();

    style {
        .scaffold { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="scaffold" data-testid="scaffold-landing">
            <SrsUpload
                content={s.srs_content.clone()}
                on_parse={Some(Box::new({ let s = s.clone(); move || store::parse(&s) }))}
                loading={s.loading.get()}
            />
            @if !s.parsed_domains.get().is_empty() {
                <ScaffoldPreview domains={s.parsed_domains.clone()} />
                <PhaseFilter phases={s.selected_phases.clone()} file_types={s.selected_file_types.clone()} />
                <Button label="Execute Scaffold" variant="primary" on:click={{ let s2 = s.clone(); move || store::execute(&s2) }} data-testid="scaffold-execute-btn" />
            }
            @if let Some(ref result) = s.scaffold_result.get().as_ref() {
                <ScaffoldProgress result={result.clone()} />
            }
        </div>
    }
}
