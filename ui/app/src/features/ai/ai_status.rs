use rsc_compat::prelude::*;
use super::types::AiStatus;

/// Badge indicator showing AI subsystem availability (FR-805).
/// Displays a success badge when enabled, a warning badge when disabled.
/// Shows a toast-like warning notification when the AI service is unavailable.
#[component]
pub fn ai_status_badge(
    status: Signal<Option<AiStatus>>,
) -> View {
    let show_toast = signal(false);

    // Show a toast when status loads and AI is not enabled.
    {
        let show_toast = show_toast.clone();
        let status = status.clone();
        effect(move || {
            if let Some(ref s) = status.get() {
                if !s.enabled {
                    show_toast.set(true);
                } else {
                    show_toast.set(false);
                }
            }
        });
    }

    view! {
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
        div(class="ai-status", data-testid="ai-status") {
            ({
                let badge_view = match status.get() {
                    Some(s) => {
                        let variant = s.badge_variant().to_string();
                        let label_text = s.label();
                        let badge_class = format!("badge badge--{}", variant);
                        view! {
                            span(class=badge_class, data-testid="ai-status-badge") {
                                (label_text)
                            }
                        }
                    }
                    None => {
                        view! {
                            span(class="ai-status__label", data-testid="ai-status-loading") {
                                "Checking AI status..."
                            }
                        }
                    }
                };
                let show_toast2 = show_toast.clone();
                let toast_view = if show_toast2.get() {
                    view! {
                        div(
                            class="toast toast--warning",
                            role="alert",
                            data-testid="ai-status-toast"
                        ) {
                            span { "AI service is currently unavailable. Some features may be limited." }
                            button(
                                class="toast__dismiss",
                                on:click={
                                    let show_toast3 = show_toast.clone();
                                    move || show_toast3.set(false)
                                }
                            ) { "Dismiss" }
                        }
                    }
                } else {
                    view! {}
                };
                View::fragment(vec![badge_view, toast_view])
            })
        }
    }
}
