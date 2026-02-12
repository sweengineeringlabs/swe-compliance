use rsc_ui::prelude::*;
use crate::features::reports::reports_type::ReportData;

/// Report export card with format selection and content preview (FR-700, FR-701, FR-702).
///
/// Allows the user to choose an export format (JSON, Markdown, PDF), trigger
/// the export, and view or download the generated report.
component ReportExport(
    scan_id: Option<String>,
    format: String,
    report: Option<ReportData>,
    loading: bool,
    on_format_change: Fn(String),
    on_export: Fn(),
) {
    style {
        .report-export { display: flex; flex-direction: column; gap: var(--space-4); }
        .report-export__controls { display: flex; gap: var(--space-3); align-items: flex-end; }
        .report-export__preview { margin-top: var(--space-3); }
        .report-export__download { display: inline-flex; align-items: center; gap: var(--space-2); margin-top: var(--space-3); }
        .report-export__meta { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-2); }
    }

    render {
        <Card data-testid="report-export">
            <div class="report-export">
                <h3>"Export Report"</h3>

                <div class="report-export__controls">
                    <FormField label="Format">
                        <Select
                            value={format.clone()}
                            on:change={|v| on_format_change(v)}
                            data-testid="report-format-select"
                        >
                            <option value="json">"JSON"</option>
                            <option value="markdown">"Markdown"</option>
                            <option value="pdf">"PDF"</option>
                        </Select>
                    </FormField>

                    <Button
                        label="Export Report"
                        variant="primary"
                        disabled={loading || scan_id.is_none()}
                        on:click={|| on_export()}
                        data-testid="report-export-btn"
                    />
                </div>

                @if loading {
                    <Spinner data-testid="report-export-spinner" />
                }

                @if let Some(ref data) = report {
                    <div class="report-export__preview" data-testid="report-export-preview">
                        @if data.format == "pdf" {
                            <div class="report-export__download">
                                <Button
                                    label="Download PDF"
                                    variant="secondary"
                                    on:click={|| {
                                        download_blob(&data.content, "report.pdf", "application/pdf");
                                    }}
                                    data-testid="report-download-pdf-btn"
                                />
                            </div>
                        } else {
                            <CodeBlock
                                language={if data.format == "json" { "json" } else { "markdown" }}
                                data-testid="report-content-block"
                            >
                                {&data.content}
                            </CodeBlock>
                        }

                        @if !data.generated_at.is_empty() {
                            <p class="report-export__meta" data-testid="report-generated-at">
                                {format!("Generated at: {}", data.generated_at)}
                            </p>
                        }
                    </div>
                }
            </div>
        </Card>
    }
}
