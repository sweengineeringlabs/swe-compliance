use rsc_ui::prelude::*;
use crate::features::ai::ai_type::AuditResult;

/// Audit results view showing summary, scan details, and recommendations
/// (FR-802, FR-803).
///
/// Structure:
///   - Summary card at the top with the high-level finding
///   - Accordion expanding raw scan results (rendered as JSON in CodeBlock)
///   - Numbered list of actionable recommendations with Badge indices
component AuditView(
    result: Signal<Option<AuditResult>>,
    loading: bool,
    on_run: Fn(String, String),
) {
    let audit_path = signal(String::new());
    let audit_scope = signal("full".to_string());

    style {
        .audit-view {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
        }

        .audit-view__trigger {
            display: flex;
            gap: var(--space-3);
            align-items: flex-end;
        }

        .audit-view__summary {
            padding: var(--space-3);
            font-size: var(--font-size-md);
            color: var(--color-text);
            line-height: 1.6;
        }

        .audit-view__recommendations {
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }

        .audit-view__rec-item {
            display: flex;
            align-items: baseline;
            gap: var(--space-2);
            padding: var(--space-2) 0;
        }

        .audit-view__empty {
            color: var(--color-text-secondary);
            font-style: italic;
            padding: var(--space-4);
            text-align: center;
        }
    }

    render {
        <div class="audit-view" data-testid="audit-view">
            <div class="audit-view__trigger">
                <FormField label="Path">
                    <Input
                        value={audit_path.clone()}
                        on:input={|v| audit_path.set(v)}
                        placeholder="e.g. /src or ."
                        data-testid="audit-path-input"
                    />
                </FormField>
                <FormField label="Scope">
                    <Select
                        value={audit_scope.clone()}
                        on:change={|v| audit_scope.set(v)}
                        data-testid="audit-scope-select"
                    >
                        <option value="full">"Full"</option>
                        <option value="incremental">"Incremental"</option>
                    </Select>
                </FormField>
                <Button
                    label="Run Audit"
                    variant="primary"
                    disabled={loading || audit_path.get().is_empty()}
                    on:click={|| on_run(audit_path.get().clone(), audit_scope.get().clone())}
                    data-testid="audit-run-btn"
                />
            </div>

            @if loading {
                <div class="audit-view__empty" data-testid="audit-loading">
                    "Running audit..."
                </div>
            }

            @match result.get() {
                Some(audit) => {
                    <Card data-testid="audit-summary">
                        <div class="audit-view__summary">
                            <strong>"Summary: "</strong>
                            {&audit.summary}
                        </div>
                    </Card>

                    <Accordion title="Scan Results" data-testid="audit-scan-details">
                        <CodeBlock language="json" data-testid="audit-scan-json">
                            {json_stringify_pretty(&audit.scan_results)}
                        </CodeBlock>
                    </Accordion>

                    <Card data-testid="audit-recommendations">
                        <h4>"Recommendations"</h4>
                        <div class="audit-view__recommendations">
                            @for (idx, rec) in audit.recommendations.iter().enumerate() {
                                <div class="audit-view__rec-item" data-testid="audit-rec-item">
                                    <Badge variant="info" data-testid="audit-rec-badge">
                                        {idx + 1}
                                    </Badge>
                                    {rec}
                                </div>
                            }
                            @if audit.recommendations.is_empty() {
                                <div class="audit-view__empty" data-testid="audit-no-recs">
                                    "No recommendations — all checks passed."
                                </div>
                            }
                        </div>
                    </Card>
                }
                None => {
                    @if !loading {
                        <div class="audit-view__empty" data-testid="audit-empty">
                            "Run an audit to see compliance results."
                        </div>
                    }
                }
            }
        </div>
    }
}
