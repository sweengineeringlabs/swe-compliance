use rsc_ui::prelude::*;
use crate::features::struct_engine::struct_engine_store as store;
use crate::features::struct_engine::struct_results::StructResults;
use crate::features::struct_engine::crate_layout::CrateLayout;
use crate::features::struct_engine::project_kind::ProjectKindBadge;

/// Struct engine landing page (FR-1100..1102).
/// Combines the check results table, crate layout tree, and project kind badge.
component StructEngineLanding() {
    style {
        .struct-engine { display: flex; flex-direction: column; gap: var(--space-4); }
        .struct-engine__header { display: flex; align-items: center; gap: var(--space-3); }
        .struct-engine__content { display: grid; grid-template-columns: 1fr 300px; gap: var(--space-4); }
        .struct-engine__error { color: var(--color-error); font-size: var(--font-size-sm); padding: var(--space-2); }
    }

    render {
        <div class="struct-engine" data-testid="struct-engine-landing">
            <div class="struct-engine__header">
                <ProjectKindBadge kind={store::project_kind.clone()} />
                <Badge variant="info" data-testid="struct-check-count">
                    {format!("{} checks", store::checks.get().len())}
                </Badge>
            </div>

            @if let Some(ref err) = store::error.get() {
                <div class="struct-engine__error" data-testid="struct-engine-error">
                    {err}
                    <Button variant="ghost" size="sm" on:click={|| store::clear_error()} data-testid="struct-dismiss-error">
                        "Dismiss"
                    </Button>
                </div>
            }

            <div class="struct-engine__content">
                <StructResults
                    checks={store::filtered_checks()}
                    loading={store::loading.get()}
                />
                @if let Some(ref tree) = store::crate_tree.get() {
                    <CrateLayout tree={tree.clone()} />
                }
            </div>
        </div>
    }
}
