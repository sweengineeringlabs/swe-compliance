use rsc_ui::prelude::*;

/// SRS content upload/paste component (FR-500).
component SrsUpload(
    content: Signal<String>,
    on_parse: Fn(),
    loading: bool,
) {
    style {
        .srs-upload { display: flex; flex-direction: column; gap: var(--space-3); }
        .srs-upload__textarea { min-height: 200px; font-family: var(--font-family-mono); font-size: var(--font-size-sm); }
    }

    render {
        <div class="srs-upload" data-testid="srs-upload">
            <FormField label="SRS Markdown Content">
                <Input
                    value={content.clone()}
                    on:input={|v| content.set(v)}
                    multiline={true}
                    class="srs-upload__textarea"
                    placeholder="Paste your SRS markdown content here..."
                    data-testid="srs-content-input"
                />
            </FormField>
            <Button label="Parse SRS" variant="primary" disabled={loading || content.get().is_empty()} on:click={on_parse} data-testid="srs-parse-btn" />
        </div>
    }
}
