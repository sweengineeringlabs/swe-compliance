use rsc_ui::prelude::*;
use crate::features::violations::store::ViolationsStore;

/// Filter bar with category, severity, and search (FR-402..403).
component ViolationFilterBar() {
    let s = use_context::<ViolationsStore>();

    let filter_category = { let f = s.filter.clone(); derived(move || f.get().category.clone().unwrap_or_default()) };
    let filter_error = { let f = s.filter.clone(); derived(move || f.get().severities.contains(&"Error".to_string())) };
    let filter_warning = { let f = s.filter.clone(); derived(move || f.get().severities.contains(&"Warning".to_string())) };
    let filter_info = { let f = s.filter.clone(); derived(move || f.get().severities.contains(&"Info".to_string())) };
    let filter_search = { let f = s.filter.clone(); derived(move || f.get().search.clone()) };

    style {
        .violation-filter { display: flex; gap: var(--space-3); align-items: flex-end; flex-wrap: wrap; }
        .violation-filter__severity { display: flex; gap: var(--space-2); align-items: center; }
    }

    render {
        <div class="violation-filter" data-testid="violation-filter">
            <FormField label="Category">
                <Select value={filter_category.get()} on:change={{ let s2 = s.clone(); move |v: String| s2.set_category(if v.is_empty() { None } else { Some(v) }) }} data-testid="filter-category">
                    <option value="">"All Categories"</option>
                    <option value="project_documentation">"Project Documentation"</option>
                    <option value="requirements_traceability">"Requirements Traceability"</option>
                    <option value="design_documentation">"Design Documentation"</option>
                    <option value="test_documentation">"Test Documentation"</option>
                </Select>
            </FormField>
            <div class="violation-filter__severity">
                <Checkbox label="Error" checked={filter_error.get()} on:change={{ let s2 = s.clone(); move |_v: String| s2.toggle_severity("Error") }} data-testid="filter-error" />
                <Checkbox label="Warning" checked={filter_warning.get()} on:change={{ let s2 = s.clone(); move |_v: String| s2.toggle_severity("Warning") }} data-testid="filter-warning" />
                <Checkbox label="Info" checked={filter_info.get()} on:change={{ let s2 = s.clone(); move |_v: String| s2.toggle_severity("Info") }} data-testid="filter-info" />
            </div>
            <FormField label="Search">
                <Input value={filter_search.get()} on:input={{ let s2 = s.clone(); move |v: String| s2.set_search(v) }} placeholder="Search violations..." data-testid="filter-search" />
            </FormField>
        </div>
    }
}
