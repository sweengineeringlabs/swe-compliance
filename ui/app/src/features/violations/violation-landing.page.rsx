use rsc_ui::prelude::*;
use crate::features::violations::store::ViolationsStore;
use crate::features::violations::violation_list::ViolationList;
use crate::features::violations::violation_filter::ViolationFilterBar;
use crate::features::violations::violation_detail::ViolationDetail;

/// Violations browser page (FR-400..404).
component ViolationsLanding() {
    let s = use_context::<ViolationsStore>();
    let selected_idx = signal(Option::<usize>::None);

    style {
        .violations { display: flex; flex-direction: column; gap: var(--space-4); }
    }

    render {
        <div class="violations" data-testid="violations-landing">
            <ViolationFilterBar />
            <ViolationList
                violations={s.filtered_violations.clone()}
                on_select={Some(Box::new(move |idx: usize| selected_idx.set(Some(idx))))}
            />
            @if let Some(idx) = selected_idx.get() {
                @if let Some(v) = s.filtered_violations.get().get(idx) {
                    <ViolationDetail violation={v.clone()} />
                }
            }
        </div>
    }
}
