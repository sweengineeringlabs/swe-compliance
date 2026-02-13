use rsc_compat::prelude::*;
use crate::features::editor::types::ValidationResult;

/// Panel displaying SRS validation results (FR-901).
/// Shows a valid/invalid badge, a list of errors with line numbers,
/// and a list of warnings.
#[component]
pub fn validation_panel(validation: Signal<Option<ValidationResult>>) -> View {
    view! {
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
        <div class="card" data-testid="validation-panel">
            {match validation.get().as_ref() {
                Some(result) => {
                    let errors_html = result.errors.iter().map(|error| {
                        view! {
                            <div class="validation-panel__error" data-testid="validation-error-item">
                                <span class="validation-panel__error-line" data-testid="validation-error-line">
                                    {format!("L{}", error.line)}
                                </span>
                                <span data-testid="validation-error-message">
                                    {&error.message}
                                </span>
                            </div>
                        }
                    }).collect::<Vec<_>>();

                    let warnings_html = result.warnings.iter().map(|warning| {
                        view! {
                            <div class="validation-panel__warning" data-testid="validation-warning-item">
                                {warning}
                            </div>
                        }
                    }).collect::<Vec<_>>();

                    view! {
                        <div>
                            <div class="validation-panel__summary" data-testid="validation-summary">
                                {if result.valid {
                                    view! {
                                        <span class="badge badge--success" data-testid="validation-badge">"Valid"</span>
                                    }
                                } else {
                                    view! {
                                        <span class="badge badge--danger" data-testid="validation-badge">"Invalid"</span>
                                    }
                                }}
                                <span class="validation-panel__counts" data-testid="validation-counts">
                                    {format!("{} domains, {} requirements", result.domain_count, result.requirement_count)}
                                </span>
                            </div>

                            {if !result.errors.is_empty() {
                                view! {
                                    <div class="validation-panel__section" data-testid="validation-errors">
                                        <span class="validation-panel__section-title">
                                            {format!("Errors ({})", result.errors.len())}
                                        </span>
                                        {errors_html}
                                    </div>
                                }
                            } else {
                                view! {}
                            }}

                            {if !result.warnings.is_empty() {
                                view! {
                                    <div class="validation-panel__section" data-testid="validation-warnings">
                                        <span class="validation-panel__section-title">
                                            {format!("Warnings ({})", result.warnings.len())}
                                        </span>
                                        {warnings_html}
                                    </div>
                                }
                            } else {
                                view! {}
                            }}
                        </div>
                    }
                }
                None => {
                    view! {
                        <div class="validation-panel__empty" data-testid="validation-empty">
                            "No validation results yet. Click Validate to check your SRS content."
                        </div>
                    }
                }
            }}
        </div>
    }
}
