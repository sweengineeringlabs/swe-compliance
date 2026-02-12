use rsc_ui::prelude::*;
use crate::features::violations::violations_type::ViolationEntry;

/// Expandable detail panel showing fix guidance (FR-404).
component ViolationDetail(violation: ViolationEntry) {
    style {
        .violation-detail { padding: var(--space-4); }
        .violation-detail__field { margin-bottom: var(--space-2); }
        .violation-detail__label { font-weight: 600; font-size: var(--font-size-sm); color: var(--color-text-secondary); }
    }

    render {
        <Accordion data-testid="violation-detail">
            <div class="violation-detail">
                <div class="violation-detail__field">
                    <span class="violation-detail__label">"Check: "</span>
                    {format!("#{} — {}", violation.check_id, violation.description)}
                </div>
                <div class="violation-detail__field">
                    <span class="violation-detail__label">"Category: "</span>{&violation.category}
                </div>
                <div class="violation-detail__field">
                    <span class="violation-detail__label">"Severity: "</span>
                    <Badge variant={match violation.severity.as_str() { "Error" => "danger", "Warning" => "warning", _ => "info" }}
                           data-testid="detail-severity">{&violation.severity}</Badge>
                </div>
                @if let Some(ref path) = violation.file_path {
                    <div class="violation-detail__field">
                        <span class="violation-detail__label">"File: "</span>{path}
                    </div>
                }
                <div class="violation-detail__field">
                    <span class="violation-detail__label">"Message: "</span>{&violation.message}
                </div>
            </div>
        </Accordion>
    }
}
