use rsc_ui::prelude::*;
use crate::features::ai::ai_type::ChatMessage;

/// Chat panel with message history and input row (FR-800, FR-801).
/// Displays messages with role-based styling (user messages aligned right,
/// assistant messages aligned left). Input field and send button at the bottom.
component ChatPanel(
    messages: Signal<Vec<ChatMessage>>,
    input: Signal<String>,
    on_send: Fn(),
    loading: bool,
) {
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

    render {
        <Card class="chat-panel" data-testid="chat-panel">
            <div class="chat-panel__messages" data-testid="chat-messages">
                @for msg in messages.get().iter() {
                    <div
                        class={format!("chat-panel__message chat-panel__message--{}", msg.role)}
                        data-testid="chat-message"
                    >
                        {&msg.content}
                    </div>
                }
                @if loading {
                    <div class="chat-panel__loading" data-testid="chat-loading">
                        "AI is thinking..."
                    </div>
                }
            </div>
            <div class="chat-panel__input-row">
                <Input
                    value={input.clone()}
                    on:input={|v| input.set(v)}
                    placeholder="Ask about compliance..."
                    data-testid="chat-input"
                />
                <Button
                    label="Send"
                    variant="primary"
                    disabled={loading || input.get().is_empty()}
                    on:click={on_send}
                    data-testid="chat-send-btn"
                />
            </div>
        </Card>
    }
}
