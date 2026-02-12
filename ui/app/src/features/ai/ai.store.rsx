use rsc_ui::prelude::*;
use crate::features::ai::ai_type::{AiStatus, ChatMessage, AuditResult, CommandGenResult};
use crate::features::ai::ai_service;

/// Central reactive state store for the AI compliance feature.
///
/// Signals:
///   status         — Current AI subsystem status (None until first fetch)
///   messages       — Chat conversation history
///   current_input  — Current chat input field value
///   audit_result   — Most recent audit result (None until first audit)
///   command_result — Most recent command generation result
///   loading        — Whether an async operation is in flight
///   error          — Most recent error message (cleared on next action)
///   active_tab     — Currently selected tab ("chat", "audit", or "commands")
pub struct AiStore {
    pub status: Signal<Option<AiStatus>>,
    pub messages: Signal<Vec<ChatMessage>>,
    pub current_input: Signal<String>,
    pub audit_result: Signal<Option<AuditResult>>,
    pub command_result: Signal<Option<CommandGenResult>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub active_tab: Signal<String>,
}

impl AiStore {
    /// Create a new AiStore with default (empty) signal values.
    pub fn new() -> Self {
        Self {
            status: signal(None),
            messages: signal(Vec::new()),
            current_input: signal(String::new()),
            audit_result: signal(None),
            command_result: signal(None),
            loading: signal(false),
            error: signal(None),
            active_tab: signal("chat".into()),
        }
    }
}

/// Derived signal: whether the AI subsystem is enabled.
pub fn is_enabled(store: &AiStore) -> Signal<bool> {
    let status = store.status.clone();
    derived(move || status.get().map(|s| s.enabled).unwrap_or(false))
}

/// Derived signal: total number of chat messages.
pub fn message_count(store: &AiStore) -> Signal<usize> {
    let messages = store.messages.clone();
    derived(move || messages.get().len())
}

/// Append a ChatMessage to the conversation history.
pub fn add_message(store: &AiStore, msg: ChatMessage) {
    let mut msgs = store.messages.get().clone();
    msgs.push(msg);
    store.messages.set(msgs);
}

/// Clear all messages and reset the error state.
pub fn clear_messages(store: &AiStore) {
    store.messages.set(Vec::new());
    store.error.set(None);
}

/// Fetch AI subsystem status and update the store.
/// Maps to FR-805.
pub fn load_status(store: &AiStore) {
    let status = store.status;
    let error = store.error;

    spawn(async move {
        match ai_service::get_ai_status().await {
            Ok(ai_status) => {
                status.set(Some(ai_status));
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
            }
        }
    });
}

/// Send the current input as a chat message, append both user and assistant
/// messages to the conversation, and clear the input field.
/// Maps to FR-800.
pub fn send_message(store: &AiStore) {
    let text = store.current_input.get().clone();
    if text.is_empty() {
        return;
    }

    // Append the user message immediately.
    add_message(store, ChatMessage {
        role: "user".into(),
        content: text.clone(),
    });
    store.current_input.set(String::new());
    store.loading.set(true);
    store.error.set(None);

    let messages = store.messages;
    let loading = store.loading;
    let error = store.error;

    spawn(async move {
        match ai_service::send_chat(&text).await {
            Ok(reply) => {
                let mut msgs = messages.get().clone();
                msgs.push(reply);
                messages.set(msgs);
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Run an AI compliance audit with the given path and scope.
/// Maps to FR-802.
pub fn run_audit(store: &AiStore, path: &str, scope: &str) {
    store.loading.set(true);
    store.error.set(None);

    let audit_result = store.audit_result;
    let loading = store.loading;
    let error = store.error;
    let path_owned = path.to_string();
    let scope_owned = scope.to_string();

    spawn(async move {
        match ai_service::run_audit(&path_owned, &scope_owned).await {
            Ok(result) => {
                audit_result.set(Some(result));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}

/// Generate compliance commands from requirements JSON and project context.
/// Maps to FR-804.
pub fn generate_commands(store: &AiStore, requirements_json: &str, project_context: &str) {
    store.loading.set(true);
    store.error.set(None);

    let command_result = store.command_result;
    let loading = store.loading;
    let error = store.error;
    let reqs_owned = requirements_json.to_string();
    let ctx_owned = project_context.to_string();

    spawn(async move {
        match ai_service::generate_commands(&reqs_owned, &ctx_owned).await {
            Ok(result) => {
                command_result.set(Some(result));
                loading.set(false);
            }
            Err(api_error) => {
                error.set(Some(api_error.message));
                loading.set(false);
            }
        }
    });
}
