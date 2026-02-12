use rsc_ui::prelude::*;
use crate::features::templates::templates_type::TemplateEntry;

/// Card showing selected template details with a copy action (FR-601, FR-602).
component TemplatePreview(
    template: TemplateEntry,
    on_copy: Fn(),
) {
    style {
        .template-preview { display: flex; flex-direction: column; gap: var(--space-3); }
        .template-preview__meta { display: flex; gap: var(--space-4); align-items: center; flex-wrap: wrap; }
        .template-preview__tags { display: flex; gap: var(--space-1); flex-wrap: wrap; }
        .template-preview__actions { display: flex; justify-content: flex-end; padding-top: var(--space-2); }
    }

    render {
        <Card data-testid="template-preview">
            <div class="template-preview">
                <h2 data-testid="template-preview-name">{&template.name}</h2>
                <p data-testid="template-preview-description">{&template.description}</p>
                <div class="template-preview__meta">
                    <Badge variant="default" data-testid="template-preview-category">{&template.category}</Badge>
                    <span data-testid="template-preview-file-count">
                        {format!("{} files", template.file_count)}
                    </span>
                </div>
                <div class="template-preview__tags">
                    @for tag in template.tags.iter() {
                        <Badge variant="info" data-testid={format!("template-preview-tag-{}", tag)}>
                            {tag}
                        </Badge>
                    }
                </div>
                <div class="template-preview__actions">
                    <Button
                        label="Copy to Project"
                        variant="primary"
                        on:click={on_copy}
                        data-testid="template-copy-btn"
                    />
                </div>
            </div>
        </Card>
    }
}
