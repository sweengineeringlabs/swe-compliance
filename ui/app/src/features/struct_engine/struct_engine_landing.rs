use rsc_compat::prelude::*;
use crate::features::struct_engine::store::StructEngineStore;
use crate::features::struct_engine::struct_result::struct_results;
use crate::features::struct_engine::crate_layout::crate_layout;
use crate::features::struct_engine::project_kind::project_kind_badge;

/// Struct engine landing page (FR-1100..1102).
/// Combines the check results table, crate layout tree, and project kind badge.
#[component]
pub fn struct_engine_landing() -> View {
    let s = use_context::<StructEngineStore>();

    let kind_view = project_kind_badge(s.project_kind.get().clone());
    let results_view = struct_results(s.filtered_checks(), s.loading.get());
    let tree_view = if let Some(ref tree) = s.crate_tree.get() {
        crate_layout(tree.clone())
    } else {
        view! {}
    };
    let error_view = {
        let err_opt = s.error.get().clone();
        if let Some(ref err_msg) = err_opt {
            let s = s.clone();
            view! {
                <div class="struct-engine__error" data-testid="struct-engine-error">
                    {err_msg.as_str()}
                    <button
                        class="btn btn--ghost btn--sm"
                        on:click={move || s.clear_error()}
                        data-testid="struct-dismiss-error"
                    >
                        "Dismiss"
                    </button>
                </div>
            }
        } else {
            view! {}
        }
    };

    view! {
        style {
            .struct-engine { display: flex; flex-direction: column; gap: var(--space-4); }
            .struct-engine__header { display: flex; align-items: center; gap: var(--space-3); }
            .struct-engine__content { display: grid; grid-template-columns: 1fr 300px; gap: var(--space-4); }
            .struct-engine__error { color: var(--color-error); font-size: var(--font-size-sm); padding: var(--space-2); }
        }
        <div class="struct-engine" data-testid="struct-engine-landing">
            <div class="struct-engine__header">
                {kind_view}
                <span class="badge badge--info" data-testid="struct-check-count">
                    {format!("{} checks", s.checks.get().len())}
                </span>
            </div>
            {error_view}
            <div class="struct-engine__content">
                {results_view}
                {tree_view}
            </div>
        </div>
    }
}
