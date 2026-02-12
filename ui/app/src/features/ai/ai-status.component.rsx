use rsc_ui::prelude::*;
use crate::features::ai::ai_type::AiStatus;

/// Badge indicator showing AI subsystem availability (FR-805).
/// Displays a success badge when enabled, a warning badge when disabled.
/// Shows a Toast error notification when the AI service is unavailable.
component AiStatusBadge(
    status: Signal<Option<AiStatus>>,
) {
    let show_toast = signal(false);

    // Show a toast when status loads and AI is not enabled.
    effect(move || {
        if let Some(ref s) = status.get() {
            if !s.enabled {
                show_toast.set(true);
            } else {
                show_toast.set(false);
            }
        }
    });

    style {
        .ai-status {
            display: inline-flex;
            align-items: center;
            gap: var(--space-2);
        }

        .ai-status__label {
            font-size: var(--font-size-sm);
            color: var(--color-text-secondary);
        }
    }

    render {
        <div class="ai-status" data-testid="ai-status">
            @match status.get() {
                Some(s) => {
                    <Badge variant={s.badge_variant()} data-testid="ai-status-badge">
                        {s.label()}
                    </Badge>
                }
                None => {
                    <span class="ai-status__label" data-testid="ai-status-loading">
                        "Checking AI status..."
                    </span>
                }
            }

            @if show_toast.get() {
                <Toast
                    variant="warning"
                    message="AI service is currently unavailable. Some features may be limited."
                    on:dismiss={|| show_toast.set(false)}
                    data-testid="ai-status-toast"
                />
            }
        </div>
    }
}
