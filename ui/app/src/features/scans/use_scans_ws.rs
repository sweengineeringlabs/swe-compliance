use rsc_compat::prelude::*;
use crate::util::api::api_ws;
use super::types::ScanProgress;

/// WebSocket connection state for tracking lifecycle.
#[derive(Clone, Debug, PartialEq)]
pub enum WsState {
    Connecting,
    Connected,
    Disconnected,
    Completed,
}

/// Maximum number of reconnection attempts before giving up.
const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Base delay in milliseconds between reconnection attempts (doubles each retry).
const RECONNECT_BASE_DELAY_MS: u32 = 1000;

/// Hook that connects to the scan progress WebSocket endpoint and returns
/// a reactive signal of ScanProgress updates.
///
/// Automatically handles:
///   - Connection lifecycle (connect, reconnect, close)
///   - Exponential backoff on disconnection (up to 5 retries)
///   - Transition to Completed state when current == total
///   - Cleanup on component unmount
///
/// Maps to: WS /api/v1/scans/{id}/progress (FR-302)
pub fn use_scan_progress(scan_id: Signal<Option<String>>) -> (Signal<Option<ScanProgress>>, Signal<WsState>) {
    let progress = signal(Option::<ScanProgress>::None);
    let ws_state = signal(WsState::Disconnected);
    let ws_handle = signal(Option::<WebSocket>::None);
    let reconnect_attempts = signal(0u32);

    // Clone signals so both effect() and on_cleanup() can capture them,
    // while the originals are returned at the end of the function.
    let progress_eff = progress.clone();
    let ws_state_eff = ws_state.clone();
    let ws_handle_eff = ws_handle.clone();
    let reconnect_eff = reconnect_attempts.clone();
    let ws_handle_cleanup = ws_handle.clone();

    // Connect or disconnect when scan_id changes.
    effect(move || {
        // Close any existing connection first.
        if let Some(ws) = ws_handle_eff.get() {
            ws.close();
            ws_handle_eff.set(None);
        }

        match scan_id.get() {
            Some(id) => {
                progress_eff.set(None);
                reconnect_eff.set(0);
                connect_ws(
                    &id,
                    progress_eff.clone(),
                    ws_state_eff.clone(),
                    ws_handle_eff.clone(),
                    reconnect_eff.clone(),
                );
            }
            None => {
                ws_state_eff.set(WsState::Disconnected);
                progress_eff.set(None);
            }
        }
    });

    // Cleanup on unmount.
    on_cleanup(move || {
        if let Some(ws) = ws_handle_cleanup.get() {
            ws.close();
        }
    });

    (progress, ws_state)
}

/// Establish a WebSocket connection and wire message/error/close handlers.
fn connect_ws(
    scan_id: &str,
    progress: Signal<Option<ScanProgress>>,
    ws_state: Signal<WsState>,
    ws_handle: Signal<Option<WebSocket>>,
    reconnect_attempts: Signal<u32>,
) {
    let path = format!("/scans/{scan_id}/progress");
    ws_state.set(WsState::Connecting);

    let ws = api_ws(&path);
    let scan_id_owned = scan_id.to_string();

    // Clone signals for on_open closure.
    let ws_state_open = ws_state.clone();
    let reconnect_open = reconnect_attempts.clone();
    ws.on_open(move || {
        ws_state_open.set(WsState::Connected);
        reconnect_open.set(0);
    });

    // Clone signals for on_message closure.
    let progress_msg = progress.clone();
    let ws_state_msg = ws_state.clone();
    let ws_handle_msg = ws_handle.clone();
    ws.on_message(move |data: String| {
        if let Some(parsed) = json_parse(&data) {
            if let Some(msg) = ScanProgress::from_json(&parsed) {
                let is_complete = msg.is_complete();
                progress_msg.set(Some(msg));

                if is_complete {
                    ws_state_msg.set(WsState::Completed);
                    if let Some(handle) = ws_handle_msg.get() {
                        handle.close();
                    }
                }
            }
        }
    });

    // Clone ws_state for on_error closure.
    let ws_state_err = ws_state.clone();
    ws.on_error(move || {
        ws_state_err.set(WsState::Disconnected);
    });

    // Clone signals before on_close so the Fn closure can be called multiple times.
    let progress_close = progress.clone();
    let ws_state_close = ws_state.clone();
    let ws_handle_close = ws_handle.clone();
    let reconnect_close = reconnect_attempts.clone();
    ws.on_close(move || {
        // Only attempt reconnection if the scan is not complete.
        let current_state = ws_state_close.get();
        match current_state {
            WsState::Completed => {
                // Scan finished — no reconnection needed.
            }
            _ => {
                let attempts = reconnect_close.get();
                if attempts < MAX_RECONNECT_ATTEMPTS {
                    ws_state_close.set(WsState::Disconnected);
                    reconnect_close.set(attempts + 1);
                    let delay = RECONNECT_BASE_DELAY_MS * 2u32.pow(attempts);
                    let scan_id_clone = scan_id_owned.clone();
                    // Clone signals for the set_timeout Fn closure.
                    let progress_timeout = progress_close.clone();
                    let ws_state_timeout = ws_state_close.clone();
                    let ws_handle_timeout = ws_handle_close.clone();
                    let reconnect_timeout = reconnect_close.clone();
                    set_timeout(move || {
                        connect_ws(
                            &scan_id_clone,
                            progress_timeout.clone(),
                            ws_state_timeout.clone(),
                            ws_handle_timeout.clone(),
                            reconnect_timeout.clone(),
                        );
                    }, delay);
                } else {
                    ws_state_close.set(WsState::Disconnected);
                }
            }
        }
    });

    ws_handle.set(Some(ws));
}
