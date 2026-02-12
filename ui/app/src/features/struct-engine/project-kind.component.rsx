use rsc_ui::prelude::*;
use crate::features::struct_engine::struct_engine_type::ProjectKind;

/// Badge showing the project kind classification with color coding (FR-1102).
/// Displays the ProjectKind label with an appropriate badge variant.
component ProjectKindBadge(kind: Option<String>) {
    let parsed_kind = derived(move || {
        match kind.as_deref() {
            Some(k) => ProjectKind::from_str(k),
            None => ProjectKind::Unknown,
        }
    });

    style {
        .project-kind { display: inline-flex; align-items: center; gap: var(--space-2); }
        .project-kind__label { font-size: var(--font-size-sm); font-weight: 600; color: var(--color-text-secondary); }
    }

    render {
        <div class="project-kind" data-testid="project-kind">
            <span class="project-kind__label" data-testid="project-kind-label">"Project Type:"</span>
            <Badge
                variant={parsed_kind().badge_variant()}
                data-testid="project-kind-badge"
            >
                {parsed_kind().label()}
            </Badge>
        </div>
    }
}
