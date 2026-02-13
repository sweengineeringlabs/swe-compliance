use rsc_compat::prelude::*;

/// Phase and file type filter checkboxes (FR-503).
/// Note: Checkbox component replaced with native input[type=checkbox] elements.
#[component]
pub fn phase_filter(
    phases: Signal<Vec<String>>,
    file_types: Signal<Vec<String>>,
) -> View {
    let toggle_phase = {
        let phases = phases.clone();
        move |phase: &str| {
            let mut p = phases.get().clone();
            if p.contains(&phase.to_string()) {
                p.retain(|x| x != phase);
            } else {
                p.push(phase.into());
            }
            phases.set(p);
        }
    };
    let toggle_type = {
        let file_types = file_types.clone();
        move |ft: &str| {
            let mut t = file_types.get().clone();
            if t.contains(&ft.to_string()) {
                t.retain(|x| x != ft);
            } else {
                t.push(ft.into());
            }
            file_types.set(t);
        }
    };

    view! {
        style {
            .phase-filter { display: flex; gap: var(--space-6); flex-wrap: wrap; }
            .phase-filter__group { display: flex; flex-direction: column; gap: var(--space-2); }
            .phase-filter__label { font-weight: 600; font-size: var(--font-size-sm); }
            .checkbox { display: flex; align-items: center; gap: var(--space-2); cursor: pointer; }
            .checkbox input { cursor: pointer; }
        }
        div(class="phase-filter", data-testid="phase-filter") {
            div(class="phase-filter__group") {
                span(class="phase-filter__label") { "Phases" }
                label(class="checkbox", data-testid="phase-requirements") {
                    input(
                        type="checkbox",
                        checked=phases.get().contains(&"requirements".into()),
                        on:change={
                            let toggle_phase = toggle_phase.clone();
                            move |_v: String| toggle_phase("requirements")
                        },
                    )
                    "Requirements"
                }
                label(class="checkbox", data-testid="phase-design") {
                    input(
                        type="checkbox",
                        checked=phases.get().contains(&"design".into()),
                        on:change={
                            let toggle_phase = toggle_phase.clone();
                            move |_v: String| toggle_phase("design")
                        },
                    )
                    "Design"
                }
                label(class="checkbox", data-testid="phase-testing") {
                    input(
                        type="checkbox",
                        checked=phases.get().contains(&"testing".into()),
                        on:change={
                            let toggle_phase = toggle_phase.clone();
                            move |_v: String| toggle_phase("testing")
                        },
                    )
                    "Testing"
                }
                label(class="checkbox", data-testid="phase-deployment") {
                    input(
                        type="checkbox",
                        checked=phases.get().contains(&"deployment".into()),
                        on:change={
                            let toggle_phase = toggle_phase.clone();
                            move |_v: String| toggle_phase("deployment")
                        },
                    )
                    "Deployment"
                }
            }
            div(class="phase-filter__group") {
                span(class="phase-filter__label") { "File Types" }
                label(class="checkbox", data-testid="type-yaml") {
                    input(
                        type="checkbox",
                        checked=file_types.get().contains(&"yaml".into()),
                        on:change={
                            let toggle_type = toggle_type.clone();
                            move |_v: String| toggle_type("yaml")
                        },
                    )
                    "YAML"
                }
                label(class="checkbox", data-testid="type-spec") {
                    input(
                        type="checkbox",
                        checked=file_types.get().contains(&"spec".into()),
                        on:change={
                            let toggle_type = toggle_type.clone();
                            move |_v: String| toggle_type("spec")
                        },
                    )
                    "Spec"
                }
                label(class="checkbox", data-testid="type-arch") {
                    input(
                        type="checkbox",
                        checked=file_types.get().contains(&"arch".into()),
                        on:change={
                            let toggle_type = toggle_type.clone();
                            move |_v: String| toggle_type("arch")
                        },
                    )
                    "Arch"
                }
                label(class="checkbox", data-testid="type-test") {
                    input(
                        type="checkbox",
                        checked=file_types.get().contains(&"test".into()),
                        on:change={
                            let toggle_type = toggle_type.clone();
                            move |_v: String| toggle_type("test")
                        },
                    )
                    "Test"
                }
            }
        }
    }
}
