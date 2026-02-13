use rsc_ui::prelude::*;
use crate::util::api::api_ws;
use crate::features::ai::ai_type::ChatMessage;

/// WebSocket connection state for the AI chat stream.
enum WsState {
    Connecting,
    Connected,
    Disconnected,
}

/// Maximum number of reconnection attempts before giving up.
const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Base delay in milliseconds between reconnection attempts (doubles each retry).
const RECONNECT_BASE_DELAY_MS: u32 = 1000;

/// Hook that connects to the AI chat streaming WebSocket endpoint and
/// delivers incremental ChatMessage tokens as a reactive signal.
///
/// Automatically handles:
///   - Connection lifecycle (connect, reconnect, close)
///   - Exponential backoff on disconnection (up to 5 retries)
///   - Cleanup on component unmount
///
/// Maps to: WS /api/v1/ai/chat/stream (FR-801)
pub fn use_ai_chat_ws(
    enabled: Signal<bool>,
) -> (Signal<Option<ChatMessage>>, Signal<WsState>) {
    let on_message = signal(Option::<ChatMessage>::None);
    let ws_state = signal(WsState::Disconnected);
    let ws_handle = signal(Option::<WebSocket>::None);
    let reconnect_attempts = signal(0u32);

    // Clone ws_handle so both effect() and on_cleanup() can capture it.
    let ws_handle_cleanup = ws_handle.clone();

    // Connect or disconnect when enabled changes.
    effect(move || {
        // Close any existing connection first.
        if let Some(ws) = ws_handle.get() {
            ws.close();
            ws_handle.set(None);
        }

        if enabled.get() {
            reconnect_attempts.set(0);
            connect_ws(on_message, ws_state, ws_handle, reconnect_attempts);
        } else {
            ws_state.set(WsState::Disconnected);
            on_message.set(None);
        }
    });

    // Cleanup on unmount.
    on_cleanup(move || {
        if let Some(ws) = ws_handle_cleanup.get() {
            ws.close();
        }
    });

    (on_message, ws_state)
}

/// Establish a WebSocket connection and wire message/error/close handlers.
fn connect_ws(
    on_message: Signal<Option<ChatMessage>>,
    ws_state: Signal<WsState>,
    ws_handle: Signal<Option<WebSocket>>,
    reconnect_attempts: Signal<u32>,
) {
    ws_state.set(WsState::Connecting);

    let ws = api_ws("/ai/chat/stream");

    // Clone signals for on_open closure.
    let ws_state_open = ws_state.clone();
    let reconnect_open = reconnect_attempts.clone();
    ws.on_open(move || {
        ws_state_open.set(WsState::Connected);
        reconnect_open.set(0);
    });

    // Clone on_message signal for on_message closure.
    let on_message_handler = on_message.clone();
    ws.on_message(move |data: String| {
        if let Some(parsed) = json_parse(&data) {
            if let Some(msg) = ChatMessage::from_json(&parsed) {
                on_message_handler.set(Some(msg));
            }
        }
    });

    // Clone ws_state for on_error closure.
    let ws_state_err = ws_state.clone();
    ws.on_error(move || {
        ws_state_err.set(WsState::Disconnected);
    });

    // on_close is the last closure — it can consume the original signals.
    ws.on_close(move || {
        let attempts = reconnect_attempts.get();
        if attempts < MAX_RECONNECT_ATTEMPTS {
            ws_state.set(WsState::Disconnected);
            reconnect_attempts.set(attempts + 1);
            let delay = RECONNECT_BASE_DELAY_MS * 2u32.pow(attempts);
            set_timeout(move || {
                connect_ws(on_message, ws_state, ws_handle, reconnect_attempts);
            }, delay);
        } else {
            ws_state.set(WsState::Disconnected);
        }
    });

    ws_handle.set(Some(ws));
}
