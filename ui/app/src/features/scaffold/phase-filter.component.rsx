use rsc_ui::prelude::*;

/// Phase and file type filter checkboxes (FR-503).
component PhaseFilter(
    phases: Signal<Vec<String>>,
    file_types: Signal<Vec<String>>,
) {
    let toggle_phase = |phase: &str| {
        let mut p = phases.get().clone();
        if p.contains(&phase.to_string()) { p.retain(|x| x != phase); } else { p.push(phase.into()); }
        phases.set(p);
    };
    let toggle_type = |ft: &str| {
        let mut t = file_types.get().clone();
        if t.contains(&ft.to_string()) { t.retain(|x| x != ft); } else { t.push(ft.into()); }
        file_types.set(t);
    };

    style {
        .phase-filter { display: flex; gap: var(--space-6); flex-wrap: wrap; }
        .phase-filter__group { display: flex; flex-direction: column; gap: var(--space-2); }
        .phase-filter__label { font-weight: 600; font-size: var(--font-size-sm); }
    }

    render {
        <div class="phase-filter" data-testid="phase-filter">
            <div class="phase-filter__group">
                <span class="phase-filter__label">"Phases"</span>
                <Checkbox label="Requirements" checked={phases.get().contains(&"requirements".into())} on:change={|_| toggle_phase("requirements")} data-testid="phase-requirements" />
                <Checkbox label="Design" checked={phases.get().contains(&"design".into())} on:change={|_| toggle_phase("design")} data-testid="phase-design" />
                <Checkbox label="Testing" checked={phases.get().contains(&"testing".into())} on:change={|_| toggle_phase("testing")} data-testid="phase-testing" />
                <Checkbox label="Deployment" checked={phases.get().contains(&"deployment".into())} on:change={|_| toggle_phase("deployment")} data-testid="phase-deployment" />
            </div>
            <div class="phase-filter__group">
                <span class="phase-filter__label">"File Types"</span>
                <Checkbox label="YAML" checked={file_types.get().contains(&"yaml".into())} on:change={|_| toggle_type("yaml")} data-testid="type-yaml" />
                <Checkbox label="Spec" checked={file_types.get().contains(&"spec".into())} on:change={|_| toggle_type("spec")} data-testid="type-spec" />
                <Checkbox label="Arch" checked={file_types.get().contains(&"arch".into())} on:change={|_| toggle_type("arch")} data-testid="type-arch" />
                <Checkbox label="Test" checked={file_types.get().contains(&"test".into())} on:change={|_| toggle_type("test")} data-testid="type-test" />
            </div>
        </div>
    }
}
