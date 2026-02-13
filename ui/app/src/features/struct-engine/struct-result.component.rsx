use rsc_ui::prelude::*;
use crate::features::struct_engine::types::StructCheck;
use crate::features::struct_engine::store::StructEngineStore;

/// Table displaying StructCheck scan results with filtering and sorting (FR-1100).
/// Columns: Check ID, Name, Category, Status (Badge), Message.
/// Supports filtering by category and status via the store.
component StructResults(
    checks: Vec<StructCheck>,
    loading: bool,
) {
    let s = use_context::<StructEngineStore>();
    let sort_column = signal("check_id".to_string());
    let sort_asc = signal(true);

    let sorted_checks = derived(move || {
        let mut items = checks.clone();
        let col = sort_column.get();
        let asc = sort_asc.get();

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

    let toggle_sort = move |col: &str| {
        if sort_column.get() == col {
            sort_asc.set(!sort_asc.get());
        } else {
            sort_column.set(col.to_string());
            sort_asc.set(true);
        }
    };

    style {
        .struct-results { display: flex; flex-direction: column; gap: var(--space-3); }
        .struct-results__filters { display: flex; gap: var(--space-3); align-items: flex-end; flex-wrap: wrap; }
        .struct-results__sortable { cursor: pointer; user-select: none; }
        .struct-results__sortable:hover { color: var(--color-primary); }
        .struct-results__empty { padding: var(--space-4); text-align: center; color: var(--color-text-secondary); }
    }

    render {
        <Card data-testid="struct-results">
            <div class="struct-results">
                <div class="struct-results__filters" data-testid="struct-results-filters">
                    <FormField label="Category">
                        <Select
                            value={s.category_filter.get().unwrap_or_default()}
                            on:change={{ let s2 = s.clone(); move |v: String| s2.set_category_filter(if v.is_empty() { None } else { Some(v) }) }}
                            data-testid="struct-filter-category"
                        >
                            <option value="">"All Categories"</option>
                            @for cat in s.categories().iter() {
                                <option value={cat.clone()}>{cat}</option>
                            }
                        </Select>
                    </FormField>
                    <FormField label="Status">
                        <Select
                            value={s.status_filter.get().unwrap_or_default()}
                            on:change={{ let s2 = s.clone(); move |v: String| s2.set_status_filter(if v.is_empty() { None } else { Some(v) }) }}
                            data-testid="struct-filter-status"
                        >
                            <option value="">"All Statuses"</option>
                            <option value="pass">"Pass"</option>
                            <option value="fail">"Fail"</option>
                            <option value="skip">"Skip"</option>
                            <option value="warning">"Warning"</option>
                        </Select>
                    </FormField>
                    <Badge variant="success" data-testid="struct-pass-count">{format!("{} pass", s.pass_count())}</Badge>
                    <Badge variant="danger" data-testid="struct-fail-count">{format!("{} fail", s.fail_count())}</Badge>
                    <Badge variant="secondary" data-testid="struct-skip-count">{format!("{} skip", s.skip_count())}</Badge>
                </div>

                @if loading {
                    <div class="struct-results__empty" data-testid="struct-results-loading">
                        "Loading scan results..."
                    </div>
                } else if sorted_checks().is_empty() {
                    <div class="struct-results__empty" data-testid="struct-results-empty">
                        "No check results match the current filters."
                    </div>
                } else {
                    <Table data-testid="struct-results-table">
                        <thead>
                            <tr>
                                <th class="struct-results__sortable" on:click={move || toggle_sort("check_id")} data-testid="sort-check-id">"Check ID"</th>
                                <th class="struct-results__sortable" on:click={move || toggle_sort("name")} data-testid="sort-name">"Name"</th>
                                <th class="struct-results__sortable" on:click={move || toggle_sort("category")} data-testid="sort-category">"Category"</th>
                                <th class="struct-results__sortable" on:click={move || toggle_sort("status")} data-testid="sort-status">"Status"</th>
                                <th class="struct-results__sortable" on:click={move || toggle_sort("message")} data-testid="sort-message">"Message"</th>
                            </tr>
                        </thead>
                        <tbody>
                            @for check in sorted_checks().iter() {
                                <tr data-testid={format!("struct-check-row-{}", check.check_id)}>
                                    <td data-testid="struct-check-id">{&check.check_id}</td>
                                    <td data-testid="struct-check-name">{&check.name}</td>
                                    <td data-testid="struct-check-category">{&check.category}</td>
                                    <td>
                                        <Badge
                                            variant={check.status_variant()}
                                            data-testid="struct-check-status"
                                        >
                                            {&check.status}
                                        </Badge>
                                    </td>
                                    <td data-testid="struct-check-message">{&check.message}</td>
                                </tr>
                            }
                        </tbody>
                    </Table>
                }
            </div>
        </Card>
    }
}
