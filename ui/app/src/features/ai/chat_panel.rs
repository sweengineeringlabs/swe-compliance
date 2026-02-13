use std::rc::Rc;
use rsc_compat::prelude::*;
use super::types::ChatMessage;

/// Chat panel with message history and input row (FR-800, FR-801).
/// Displays messages with role-based styling (user messages aligned right,
/// assistant messages aligned left). Input field and send button at the bottom.
#[component]
pub fn chat_panel(
    messages: Signal<Vec<ChatMessage>>,
    input: Signal<String>,
    on_send: Option<Box<dyn Fn()>>,
    loading: bool,
) -> View {
    let on_send: Option<Rc<dyn Fn()>> = on_send.map(|f| Rc::from(f));
    view! {
        style {
            .chat-panel {
                display: flex;
                flex-direction: column;
                gap: var(--space-3);
                height: 500px;
            }

            .chat-panel__messages {
                flex: 1;
                overflow-y: auto;
                display: flex;
                flex-direction: column;
                gap: var(--space-2);
                padding: var(--space-3);
            }

            .chat-panel__message {
                padding: var(--space-2) var(--space-3);
                border-radius: var(--radius-md);
                max-width: 80%;
                white-space: pre-wrap;
                word-break: break-word;
            }

            .chat-panel__message--user {
                align-self: flex-end;
                background: var(--color-primary-light);
            }

            .chat-panel__message--assistant {
                align-self: flex-start;
                background: var(--color-surface-raised);
            }

            .chat-panel__input-row {
                display: flex;
                gap: var(--space-2);
                align-items: center;
            }

            .chat-panel__loading {
                align-self: flex-start;
                padding: var(--space-2) var(--space-3);
                color: var(--color-text-secondary);
                font-style: italic;
            }
        }
        div(class="card chat-panel", data-testid="chat-panel") {
            div(class="chat-panel__messages", data-testid="chat-messages") {
                Indexed(
                    each=messages.get().clone(),
                    key=|msg| format!("{}-{}", msg.role, msg.content.len()),
                    view=|msg| {
                        let cls = format!("chat-panel__message chat-panel__message--{}", msg.role);
                        let content = msg.content.clone();
                        view! {
                            div(class=cls, data-testid="chat-message") {
                                (content)
                            }
                        }
                    },
                )
                (if loading {
                    view! {
                        div(class="chat-panel__loading", data-testid="chat-loading") {
                            "AI is thinking..."
                        }
                    }
                } else {
                    view! {}
                })
            }
            div(class="chat-panel__input-row") {
                input(
                    type="text",
                    class="input",
                    value=input.clone(),
                    on:input={let inp = input.clone(); move |v: String| inp.set(v)},
                    placeholder="Ask about compliance...",
                    data-testid="chat-input",
                )
                button(
                    class="btn btn--primary",
                    disabled=loading || input.get().is_empty(),
                    on:click=move || { if let Some(ref cb) = on_send { cb() } },
                    data-testid="chat-send-btn",
                ) {
                    "Send"
                }
            }
        }
    }
}
