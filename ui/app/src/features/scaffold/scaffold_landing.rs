use rsc_compat::prelude::*;
use crate::features::scaffold::store::{self, ScaffoldStore};
use crate::features::scaffold::srs_upload::srs_upload;
use crate::features::scaffold::scaffold_preview::scaffold_preview;
use crate::features::scaffold::phase_filter::phase_filter;
use crate::features::scaffold::scaffold_progress::scaffold_progress;

/// Scaffolding interface page (FR-500..504).
#[component]
pub fn scaffold_landing() -> View {
    let s = use_context::<ScaffoldStore>();

    view! {
        style {
            .scaffold { display: flex; flex-direction: column; gap: var(--space-4); }
        }
        div(class="scaffold", data-testid="scaffold-landing") {
            (srs_upload(
                s.srs_content.clone(),
                Some(Box::new({
                    let s = s.clone();
                    move || store::parse(&s)
                })),
                s.loading.get(),
            ))
            (if !s.parsed_domains.get().is_empty() {
                let s2 = s.clone();
                view! {
                    (scaffold_preview(s.parsed_domains.clone()))
                    (phase_filter(s.selected_phases.clone(), s.selected_file_types.clone()))
                    Button(label="Execute Scaffold", variant="primary", on:click={
                        let s3 = s2.clone();
                        move || store::execute(&s3)
                    }, data-testid="scaffold-execute-btn")
                }
            } else {
                view! {}
            })
            (if let Some(result) = s.scaffold_result.get() {
                scaffold_progress(result)
            } else {
                view! {}
            })
        }
    }
}
