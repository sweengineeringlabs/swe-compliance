use rsc_ui::prelude::*;
use crate::features::specs::specs_type::SpecFile;

/// Card displaying the raw content of a selected spec file (FR-1002).
/// Shows the file name, kind badge, and path in the card header,
/// with the file content rendered inside a CodeBlock.
component SpecContent(
    file: Signal<Option<SpecFile>>,
    content: String,
) {
    style {
        .spec-content__header {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            margin-bottom: var(--space-3);
        }

        .spec-content__name {
            font-size: var(--font-size-lg);
            font-weight: 600;
            color: var(--color-text);
        }

        .spec-content__path {
            font-size: var(--font-size-xs);
            color: var(--color-text-muted);
            font-family: var(--font-mono);
            margin-bottom: var(--space-3);
        }

        .spec-content__empty {
            padding: var(--space-6);
            text-align: center;
            color: var(--color-text-muted);
        }
    }

    render {
        <Card data-testid="spec-content">
            @if let Some(ref f) = file.get() {
                <div class="spec-content__header" data-testid="spec-content-header">
                    <span class="spec-content__name" data-testid="spec-content-name">
                        {&f.name}
                    </span>
                    <Badge
                        variant={f.kind_variant()}
                        data-testid="spec-content-kind"
                    >
                        {f.kind_label()}
                    </Badge>
                </div>
                <div class="spec-content__path" data-testid="spec-content-path">
                    {&f.path}
                </div>
                <CodeBlock data-testid="spec-content-code">
                    {&content}
                </CodeBlock>
            } @else {
                <div class="spec-content__empty" data-testid="spec-content-empty">
                    "Select a file from the spec tree to view its content."
                </div>
            }
        </Card>
    }
}
