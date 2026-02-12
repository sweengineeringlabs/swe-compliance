use rsc_ui::prelude::*;
use crate::util::api::api_ws;
use crate::features::scans::scans_type::ScanProgress;

/// WebSocket connection state for tracking lifecycle.
enum WsState {
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

    // Connect or disconnect when scan_id changes.
    effect(move || {
        // Close any existing connection first.
        if let Some(ws) = ws_handle.get() {
            ws.close();
            ws_handle.set(None);
        }

        match scan_id.get() {
            Some(id) => {
                progress.set(None);
                reconnect_attempts.set(0);
                connect_ws(
                    &id,
                    progress,
                    ws_state,
                    ws_handle,
                    reconnect_attempts,
                );
            }
            None => {
                ws_state.set(WsState::Disconnected);
                progress.set(None);
            }
        }
    });

    // Cleanup on unmount.
    on_cleanup(move || {
        if let Some(ws) = ws_handle.get() {
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

    ws.on_open(move || {
        ws_state.set(WsState::Connected);
        reconnect_attempts.set(0);
    });

    ws.on_message(move |data: String| {
        if let Some(parsed) = json_parse(&data) {
            if let Some(msg) = ScanProgress::from_json(&parsed) {
                let is_complete = msg.is_complete();
                progress.set(Some(msg));

                if is_complete {
                    ws_state.set(WsState::Completed);
                    if let Some(handle) = ws_handle.get() {
                        handle.close();
                    }
                }
            }
        }
    });

    ws.on_error(move || {
        ws_state.set(WsState::Disconnected);
    });

    ws.on_close(move || {
        // Only attempt reconnection if the scan is not complete.
        let current_state = ws_state.get();
        match current_state {
            WsState::Completed => {
                // Scan finished — no reconnection needed.
            }
            _ => {
                let attempts = reconnect_attempts.get();
                if attempts < MAX_RECONNECT_ATTEMPTS {
                    ws_state.set(WsState::Disconnected);
                    reconnect_attempts.set(attempts + 1);
                    let delay = RECONNECT_BASE_DELAY_MS * 2u32.pow(attempts);
                    let scan_id_clone = scan_id_owned.clone();
                    set_timeout(move || {
                        connect_ws(
                            &scan_id_clone,
                            progress,
                            ws_state,
                            ws_handle,
                            reconnect_attempts,
                        );
                    }, delay);
                } else {
                    ws_state.set(WsState::Disconnected);
                }
            }
        }
    });

    ws_handle.set(Some(ws));
}
