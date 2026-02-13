use rsc_compat::prelude::*;

/// Scan trigger form with engine/scope selection (FR-300).
#[component]
pub fn scan_trigger(
    on_trigger: Option<Box<dyn Fn(String, String, Option<String>)>>,
    disabled: bool,
) -> View {
    let engine = signal("doc-engine".to_string());
    let checks = signal(String::new());

    let engine_change = engine.clone();
    let checks_input = checks.clone();
    let engine_click = engine.clone();
    let checks_click = checks.clone();

    view! {
        style {
            .scan-trigger { display: flex; gap: var(--space-3); align-items: flex-end; }
        }
        div(class="scan-trigger", data-testid="scan-trigger") {
            FormField(label="Engine") {
                Select(value=engine.clone(), on:change={let e = engine_change.clone(); move |v: String| e.set(v)}, data-testid="scan-engine-select") {
                    option(value="doc-engine") { "doc-engine" }
                    option(value="struct-engine") { "struct-engine" }
                }
            }
            FormField(label="Checks (optional)") {
                Input(
                    value=checks.clone(),
                    on:input={let c = checks_input.clone(); move |v: String| c.set(v)},
                    placeholder="e.g. 1,2,3",
                    data-testid="scan-checks-input",
                )
            }
            Button(
                label="Run Scan",
                variant="primary",
                disabled=disabled,
                on:click={
                    let checks_click = checks_click.clone();
                    let engine_click = engine_click.clone();
                    move || {
                        let c = if checks_click.get().is_empty() { None } else { Some(checks_click.get().clone()) };
                        if let Some(ref cb) = on_trigger { cb(engine_click.get().clone(), String::new(), c) }
                    }
                },
                data-testid="scan-run-btn",
            )
        }
    }
}
