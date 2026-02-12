use rsc_ui::prelude::*;

/// Scan trigger form with engine/scope selection (FR-300).
component ScanTrigger(
    on_trigger: Fn(String, String, Option<String>),
    disabled: bool,
) {
    let engine = signal("doc-engine".to_string());
    let checks = signal(String::new());

    style {
        .scan-trigger { display: flex; gap: var(--space-3); align-items: flex-end; }
    }

    render {
        <div class="scan-trigger" data-testid="scan-trigger">
            <FormField label="Engine">
                <Select value={engine.clone()} on:change={|v| engine.set(v)} data-testid="scan-engine-select">
                    <option value="doc-engine">"doc-engine"</option>
                    <option value="struct-engine">"struct-engine"</option>
                </Select>
            </FormField>
            <FormField label="Checks (optional)">
                <Input value={checks.clone()} on:input={|v| checks.set(v)} placeholder="e.g. 1,2,3" data-testid="scan-checks-input" />
            </FormField>
            <Button
                label="Run Scan"
                variant="primary"
                disabled={disabled}
                on:click={|| {
                    let c = if checks.get().is_empty() { None } else { Some(checks.get().clone()) };
                    on_trigger(engine.get().clone(), String::new(), c);
                }}
                data-testid="scan-run-btn"
            />
        </div>
    }
}
