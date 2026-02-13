use rsc_compat::prelude::*;
use crate::features::scaffold::types::ScaffoldResult;

/// Scaffold execution results showing created/skipped files (FR-504).
/// Note: Steps component replaced with plain div elements.
#[component]
pub fn scaffold_progress(result: ScaffoldResult) -> View {
    view! {
        Card(data-testid="scaffold-progress") {
            h3 { "Scaffold Results" }
            div {
                Badge(variant="success", data-testid="scaffold-created-count") {
                    (format!("{} created", result.created.len()))
                }
                Badge(variant="secondary", data-testid="scaffold-skipped-count") {
                    (format!("{} skipped", result.skipped.len()))
                }
            }
            div(class="steps", data-testid="scaffold-steps") {
                Keyed(
                    iterable=result.created.clone(),
                    key=|file| file.clone(),
                    view=|file| view! {
                        div(data-testid="scaffold-created-file") {
                            Badge(variant="success") { "created" }
                            " "
                            (file)
                        }
                    },
                )
                Keyed(
                    iterable=result.skipped.clone(),
                    key=|file| file.clone(),
                    view=|file| view! {
                        div(data-testid="scaffold-skipped-file") {
                            Badge(variant="secondary") { "skipped" }
                            " "
                            (file)
                        }
                    },
                )
            }
        }
    }
}
