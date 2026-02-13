use rsc_ui::prelude::*;
use crate::features::violations::violations_store as store;
use crate::features::violations::violation_list::ViolationList;
use crate::features::violations::violation_filter::ViolationFilter;
use crate::features::violations::violation_detail::ViolationDetail;

/// Violations browser page (FR-400..404).
component ViolationsLanding() {
    let selected_idx = signal(Option::<usize>::None);

    style {
        .violations { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="violations" data-testid="violations-landing">
            <ViolationFilter />
            <ViolationList
                violations={store::filtered_violations.clone()}
                on_select={|idx| selected_idx.set(Some(idx))}
            />
            @if let Some(idx) = selected_idx.get() {
                @if let Some(v) = store::filtered_violations.get().get(idx) {
                    <ViolationDetail violation={v.clone()} />
                }
            }
        </div>
    }
}
