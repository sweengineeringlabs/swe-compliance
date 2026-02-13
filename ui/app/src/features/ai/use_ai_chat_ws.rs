use rsc_compat::prelude::*;
use crate::util::api::api_ws;
use super::types::ChatMessage;

/// WebSocket connection state for the AI chat stream.
#[derive(Clone, Debug, PartialEq)]
pub enum WsState {
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

    // Clone signals so both effect() and on_cleanup() can capture them,
    // while the originals are returned at the end of the function.
    let on_message_eff = on_message.clone();
    let ws_state_eff = ws_state.clone();
    let ws_handle_eff = ws_handle.clone();
    let ws_handle_cleanup = ws_handle.clone();

    // Connect or disconnect when enabled changes.
    effect(move || {
        // Close any existing connection first.
        if let Some(ws) = ws_handle_eff.get() {
            ws.close();
            ws_handle_eff.set(None);
        }

        if enabled.get() {
            reconnect_attempts.set(0);
            connect_ws(on_message_eff.clone(), ws_state_eff.clone(), ws_handle_eff.clone(), reconnect_attempts.clone());
        } else {
            ws_state_eff.set(WsState::Disconnected);
            on_message_eff.set(None);
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

    // Clone ws_handle before on_close captures it, so we can still call ws_handle.set() below.
    let ws_handle_close = ws_handle.clone();

    // on_close: clone signals inside the closure body before passing to set_timeout,
    // because on_close takes Fn (not FnOnce) so the closure may be called multiple times.
    ws.on_close(move || {
        let attempts = reconnect_attempts.get();
        if attempts < MAX_RECONNECT_ATTEMPTS {
            ws_state.set(WsState::Disconnected);
            reconnect_attempts.set(attempts + 1);
            let delay = RECONNECT_BASE_DELAY_MS * 2u32.pow(attempts);
            // Clone signals for the set_timeout closure so we don't
            // move out of this Fn closure (which can be called again).
            let on_message = on_message.clone();
            let ws_state = ws_state.clone();
            let ws_handle = ws_handle_close.clone();
            let reconnect_attempts = reconnect_attempts.clone();
            set_timeout(move || {
                connect_ws(on_message.clone(), ws_state.clone(), ws_handle.clone(), reconnect_attempts.clone());
            }, delay);
        } else {
            ws_state.set(WsState::Disconnected);
        }
    });

    ws_handle.set(Some(ws));
}
