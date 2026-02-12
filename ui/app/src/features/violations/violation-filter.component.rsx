use rsc_ui::prelude::*;
use crate::features::violations::violations_store as store;

/// Filter bar with category, severity, and search (FR-402..403).
component ViolationFilter() {
    style {
        .violation-filter { display: flex; gap: var(--space-3); align-items: flex-end; flex-wrap: wrap; }
        .violation-filter__severity { display: flex; gap: var(--space-2); align-items: center; }
    }

    render {
        <div class="violation-filter" data-testid="violation-filter">
            <FormField label="Category">
                <Select value={store::filter_category.clone()} on:change={|v| store::filter_category.set(v)} data-testid="filter-category">
                    <option value="">"All Categories"</option>
                    <option value="project_documentation">"Project Documentation"</option>
                    <option value="requirements_traceability">"Requirements Traceability"</option>
                    <option value="design_documentation">"Design Documentation"</option>
                    <option value="test_documentation">"Test Documentation"</option>
                </Select>
            </FormField>
            <div class="violation-filter__severity">
                <Checkbox label="Error" checked={store::filter_error.clone()} on:change={|v| store::filter_error.set(v)} data-testid="filter-error" />
                <Checkbox label="Warning" checked={store::filter_warning.clone()} on:change={|v| store::filter_warning.set(v)} data-testid="filter-warning" />
                <Checkbox label="Info" checked={store::filter_info.clone()} on:change={|v| store::filter_info.set(v)} data-testid="filter-info" />
            </div>
            <FormField label="Search">
                <Input value={store::filter_search.clone()} on:input={|v| store::filter_search.set(v)} placeholder="Search violations..." data-testid="filter-search" />
            </FormField>
        </div>
    }
}
