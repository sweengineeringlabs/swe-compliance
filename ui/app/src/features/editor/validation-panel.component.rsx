use rsc_ui::prelude::*;
use crate::features::editor::editor_type::ValidationResult;

/// Panel displaying SRS validation results (FR-901).
/// Shows a valid/invalid badge, a list of errors with line numbers,
/// and a list of warnings.
component ValidationPanel(validation: Signal<Option<ValidationResult>>) {
    style {
        .validation-panel__summary {
            display: flex;
            align-items: center;
            gap: var(--space-3);
        }

        .validation-panel__counts {
            font-size: var(--font-size-sm);
            color: var(--color-text-muted);
        }

        .validation-panel__section {
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }

        .validation-panel__section-title {
            font-size: var(--font-size-sm);
            font-weight: 600;
            color: var(--color-text);
        }

        .validation-panel__error {
            display: flex;
            gap: var(--space-2);
            font-size: var(--font-size-sm);
            color: var(--color-danger);
        }

        .validation-panel__error-line {
            font-weight: 600;
            font-family: var(--font-family-mono);
            white-space: nowrap;
        }

        .validation-panel__warning {
            font-size: var(--font-size-sm);
            color: var(--color-warning);
        }

        .validation-panel__empty {
            font-size: var(--font-size-sm);
            color: var(--color-text-muted);
        }
    }

    render {
        <Card data-testid="validation-panel">
            @match validation.get().as_ref() {
                Some(result) => {
                    <div class="validation-panel__summary" data-testid="validation-summary">
                        @if result.valid {
                            <Badge variant="success" data-testid="validation-badge">"Valid"</Badge>
                        } @else {
                            <Badge variant="danger" data-testid="validation-badge">"Invalid"</Badge>
                        }
                        <span class="validation-panel__counts" data-testid="validation-counts">
                            {format!("{} domains, {} requirements", result.domain_count, result.requirement_count)}
                        </span>
                    </div>

                    @if !result.errors.is_empty() {
                        <div class="validation-panel__section" data-testid="validation-errors">
                            <span class="validation-panel__section-title">
                                {format!("Errors ({})", result.errors.len())}
                            </span>
                            @for error in result.errors.iter() {
                                <div class="validation-panel__error" data-testid="validation-error-item">
                                    <span class="validation-panel__error-line" data-testid="validation-error-line">
                                        {format!("L{}", error.line)}
                                    </span>
                                    <span data-testid="validation-error-message">
                                        {&error.message}
                                    </span>
                                </div>
                            }
                        </div>
                    }

                    @if !result.warnings.is_empty() {
                        <div class="validation-panel__section" data-testid="validation-warnings">
                            <span class="validation-panel__section-title">
                                {format!("Warnings ({})", result.warnings.len())}
                            </span>
                            @for warning in result.warnings.iter() {
                                <div class="validation-panel__warning" data-testid="validation-warning-item">
                                    {warning}
                                </div>
                            }
                        </div>
                    }
                }
                None => {
                    <div class="validation-panel__empty" data-testid="validation-empty">
                        "No validation results yet. Click Validate to check your SRS content."
                    </div>
                }
            }
        </Card>
    }
}
