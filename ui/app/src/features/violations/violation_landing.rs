use rsc_compat::prelude::*;
use super::store::ViolationsStore;
use super::violation_list::violation_list;
use super::violation_filter::violation_filter_bar;
use super::violation_detail::violation_detail;

/// Violations browser page (FR-400..404).
#[component]
pub fn violations_landing() -> View {
    let s = use_context::<ViolationsStore>();
    let selected_idx = signal(Option::<usize>::None);
    let selected_idx_cb = selected_idx.clone();

    view! {
        style {
            .violations { display: flex; flex-direction: column; gap: var(--space-4); }
        }
        div(class="violations", data-testid="violations-landing") {
            (violation_filter_bar())
            (violation_list(
                s.filtered_violations.clone(),
                Some(Box::new(move |idx: usize| selected_idx_cb.set(Some(idx)))),
            ))
            (if let Some(idx) = selected_idx.get() {
                if let Some(v) = s.filtered_violations.get().get(idx) {
                    violation_detail(v.clone())
                } else {
                    view! {}
                }
            } else {
                view! {}
            })
        }
    }
}
