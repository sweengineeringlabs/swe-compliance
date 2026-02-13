use rsc_compat::prelude::*;
use crate::features::struct_engine::types::StructCheck;
use crate::features::struct_engine::store::StructEngineStore;

/// Table displaying StructCheck scan results with filtering and sorting (FR-1100).
/// Columns: Check ID, Name, Category, Status (Badge), Message.
/// Supports filtering by category and status via the store.
#[component]
pub fn struct_results(
    checks: Vec<StructCheck>,
    loading: bool,
) -> View {
    let s = use_context::<StructEngineStore>();
    let sort_column = signal("check_id".to_string());
    let sort_asc = signal(true);

    let sort_column_derived = sort_column.clone();
    let sort_asc_derived = sort_asc.clone();
    let sorted_checks = derived(move || {
        let mut items = checks.clone();
        let col = sort_column_derived.get();
        let asc = sort_asc_derived.get();

        items.sort_by(|a, b| {
            let cmp = match col.as_str() {
                "name" => a.name.cmp(&b.name),
                "category" => a.category.cmp(&b.category),
                "status" => a.status.cmp(&b.status),
                "message" => a.message.cmp(&b.message),
                _ => a.check_id.cmp(&b.check_id),
            };
            if asc { cmp } else { cmp.reverse() }
        });

        items
    });

    let cat_filter_val = s.category_filter.get().unwrap_or_default();
    let status_filter_val = s.status_filter.get().unwrap_or_default();
    let category_options: Vec<_> = s.categories().iter().map(|cat| {
        let c = cat.clone();
        view! {
            <option value={c.clone()}>{c}</option>
        }
    }).collect();
    let pass_label = format!("{} pass", s.pass_count());
    let fail_label = format!("{} fail", s.fail_count());
    let skip_label = format!("{} skip", s.skip_count());

    let content_view = if loading {
        view! {
            <div class="struct-results__empty" data-testid="struct-results-loading">
                "Loading scan results..."
            </div>
        }
    } else if sorted_checks.get().is_empty() {
        view! {
            <div class="struct-results__empty" data-testid="struct-results-empty">
                "No check results match the current filters."
            </div>
        }
    } else {
        let rows: Vec<_> = sorted_checks.get().iter().map(|check| {
            let check_id = check.check_id.clone();
            let name = check.name.clone();
            let category = check.category.clone();
            let status = check.status.clone();
            let message = check.message.clone();
            let badge_cls = format!("badge badge--{}", check.status_variant());
            let row_testid = format!("struct-check-row-{}", check_id);
            view! {
                <tr data-testid={row_testid}>
                    <td data-testid="struct-check-id">{check_id.as_str()}</td>
                    <td data-testid="struct-check-name">{name.as_str()}</td>
                    <td data-testid="struct-check-category">{category.as_str()}</td>
                    <td>
                        <span
                            class={badge_cls}
                            data-testid="struct-check-status"
                        >
                            {status.as_str()}
                        </span>
                    </td>
                    <td data-testid="struct-check-message">{message.as_str()}</td>
                </tr>
            }
        }).collect();

        view! {
            <table class="table" data-testid="struct-results-table">
                <thead>
                    <tr>
                        <th class="struct-results__sortable" on:click={
                            let sc = sort_column.clone(); let sa = sort_asc.clone();
                            move || { if sc.get() == "check_id" { sa.set(!sa.get()); } else { sc.set("check_id".to_string()); sa.set(true); } }
                        } data-testid="sort-check-id">"Check ID"</th>
                        <th class="struct-results__sortable" on:click={
                            let sc = sort_column.clone(); let sa = sort_asc.clone();
                            move || { if sc.get() == "name" { sa.set(!sa.get()); } else { sc.set("name".to_string()); sa.set(true); } }
                        } data-testid="sort-name">"Name"</th>
                        <th class="struct-results__sortable" on:click={
                            let sc = sort_column.clone(); let sa = sort_asc.clone();
                            move || { if sc.get() == "category" { sa.set(!sa.get()); } else { sc.set("category".to_string()); sa.set(true); } }
                        } data-testid="sort-category">"Category"</th>
                        <th class="struct-results__sortable" on:click={
                            let sc = sort_column.clone(); let sa = sort_asc.clone();
                            move || { if sc.get() == "status" { sa.set(!sa.get()); } else { sc.set("status".to_string()); sa.set(true); } }
                        } data-testid="sort-status">"Status"</th>
                        <th class="struct-results__sortable" on:click={
                            let sc = sort_column.clone(); let sa = sort_asc.clone();
                            move || { if sc.get() == "message" { sa.set(!sa.get()); } else { sc.set("message".to_string()); sa.set(true); } }
                        } data-testid="sort-message">"Message"</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        }
    };

    view! {
        style {
            .struct-results { display: flex; flex-direction: column; gap: var(--space-3); }
            .struct-results__filters { display: flex; gap: var(--space-3); align-items: flex-end; flex-wrap: wrap; }
            .struct-results__sortable { cursor: pointer; user-select: none; }
            .struct-results__sortable:hover { color: var(--color-primary); }
            .struct-results__empty { padding: var(--space-4); text-align: center; color: var(--color-text-secondary); }
        }
        <div class="card" data-testid="struct-results">
            <div class="struct-results">
                <div class="struct-results__filters" data-testid="struct-results-filters">
                    <div class="form-field">
                        <label>"Category"</label>
                        <select
                            value={cat_filter_val}
                            on:change={
                                let s2 = s.clone();
                                move |v: String| s2.set_category_filter(if v.is_empty() { None } else { Some(v) })
                            }
                            data-testid="struct-filter-category"
                        >
                            <option value="">"All Categories"</option>
                            {category_options}
                        </select>
                    </div>
                    <div class="form-field">
                        <label>"Status"</label>
                        <select
                            value={status_filter_val}
                            on:change={
                                let s2 = s.clone();
                                move |v: String| s2.set_status_filter(if v.is_empty() { None } else { Some(v) })
                            }
                            data-testid="struct-filter-status"
                        >
                            <option value="">"All Statuses"</option>
                            <option value="pass">"Pass"</option>
                            <option value="fail">"Fail"</option>
                            <option value="skip">"Skip"</option>
                            <option value="warning">"Warning"</option>
                        </select>
                    </div>
                    <span class="badge badge--success" data-testid="struct-pass-count">{pass_label}</span>
                    <span class="badge badge--danger" data-testid="struct-fail-count">{fail_label}</span>
                    <span class="badge badge--secondary" data-testid="struct-skip-count">{skip_label}</span>
                </div>

                {content_view}
            </div>
        </div>
    }
}
