use rsc_ui::prelude::*;
use crate::features::struct_engine::store::StructEngineStore;
use crate::features::struct_engine::struct_result::StructResults;
use crate::features::struct_engine::crate_layout::CrateLayout;
use crate::features::struct_engine::project_kind::ProjectKindBadge;

/// Struct engine landing page (FR-1100..1102).
/// Combines the check results table, crate layout tree, and project kind badge.
component StructEngineLanding() {
    let s = use_context::<StructEngineStore>();

    style {
        .struct-engine { display: flex; flex-direction: column; gap: var(--space-4); }
        .struct-engine__header { display: flex; align-items: center; gap: var(--space-3); }
        .struct-engine__content { display: grid; grid-template-columns: 1fr 300px; gap: var(--space-4); }
        .struct-engine__error { color: var(--color-error); font-size: var(--font-size-sm); padding: var(--space-2); }
    }

    render {
        <div class="struct-engine" data-testid="struct-engine-landing">
            <div class="struct-engine__header">
                <ProjectKindBadge kind={s.project_kind.clone()} />
                <Badge variant="info" data-testid="struct-check-count">
                    {format!("{} checks", s.checks.get().len())}
                </Badge>
            </div>

            @if let Some(ref err) = s.error.get().as_ref() {
                <div class="struct-engine__error" data-testid="struct-engine-error">
                    {err.as_str()}
                    <Button variant="ghost" size="sm" on:click={{ let s2 = s.clone(); move || s2.clear_error() }} data-testid="struct-dismiss-error">
                        "Dismiss"
                    </Button>
                </div>
            }

            <div class="struct-engine__content">
                <StructResults
                    checks={s.filtered_checks()}
                    loading={s.loading.get()}
                />
                @if let Some(ref tree) = s.crate_tree.get().as_ref() {
                    <CrateLayout tree={tree.clone()} />
                }
            </div>
        </div>
    }
}
