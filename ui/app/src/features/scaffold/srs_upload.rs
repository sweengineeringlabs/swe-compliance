use rsc_compat::prelude::*;

/// SRS content upload/paste component (FR-500).
#[component]
pub fn srs_upload(
    content: Signal<String>,
    on_parse: Option<Box<dyn Fn()>>,
    loading: bool,
) -> View {
    let content_input = content.clone();
    let content_check = content.clone();

    view! {
        style {
            .srs-upload { display: flex; flex-direction: column; gap: var(--space-3); }
            .srs-upload__textarea { min-height: 200px; font-family: var(--font-family-mono); font-size: var(--font-size-sm); }
        }
        div(class="srs-upload", data-testid="srs-upload") {
            FormField(label="SRS Markdown Content") {
                Input(
                    value=content.clone(),
                    on:input={let c = content_input.clone(); move |v: String| c.set(v)},
                    multiline=true,
                    class="srs-upload__textarea",
                    placeholder="Paste your SRS markdown content here...",
                    data-testid="srs-content-input",
                )
            }
            Button(
                label="Parse SRS",
                variant="primary",
                disabled=loading || content_check.get().is_empty(),
                on:click=move || { if let Some(ref cb) = on_parse { cb() } },
                data-testid="srs-parse-btn",
            )
        }
    }
}
