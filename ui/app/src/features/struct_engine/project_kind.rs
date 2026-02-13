use rsc_compat::prelude::*;
use crate::features::struct_engine::types::ProjectKind;

/// Badge showing the project kind classification with color coding (FR-1102).
/// Displays the ProjectKind label with an appropriate badge variant.
#[component]
pub fn project_kind_badge(kind: Option<String>) -> View {
    let parsed_kind = derived(move || {
        match kind.as_deref() {
            Some(k) => ProjectKind::from_str(k),
            None => ProjectKind::Unknown,
        }
    });

    view! {
        style {
            .project-kind { display: inline-flex; align-items: center; gap: var(--space-2); }
            .project-kind__label { font-size: var(--font-size-sm); font-weight: 600; color: var(--color-text-secondary); }
        }
        <div class="project-kind" data-testid="project-kind">
            <span class="project-kind__label" data-testid="project-kind-label">"Project Type:"</span>
            <span
                class={format!("badge badge--{}", parsed_kind.get().badge_variant())}
                data-testid="project-kind-badge"
            >
                {parsed_kind.get().label()}
            </span>
        </div>
    }
}
