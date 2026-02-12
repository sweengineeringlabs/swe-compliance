use std::collections::HashMap;
use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;
use tokio::sync::{broadcast, RwLock};

/// Progress message sent over WebSocket.
#[derive(Debug, Clone, Serialize)]
pub struct ProgressMessage {
    pub scan_id: String,
    pub check_id: u32,
    pub check_description: String,
    pub status: String,
    pub current: u32,
    pub total: u32,
}

/// Manages WebSocket broadcast channels for scan progress.
#[derive(Debug, Clone)]
pub struct WsBroadcaster {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl WsBroadcaster {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new broadcast channel for a scan.
    pub async fn create_channel(&self, scan_id: &str) -> broadcast::Sender<String> {
        let (tx, _) = broadcast::channel(256);
        self.channels
            .write()
            .await
            .insert(scan_id.to_string(), tx.clone());
        tx
    }

    /// Subscribe to an existing scan's progress channel.
    pub async fn subscribe(&self, scan_id: &str) -> Option<broadcast::Receiver<String>> {
        self.channels
            .read()
            .await
            .get(scan_id)
            .map(|tx| tx.subscribe())
    }

    /// Remove a channel when a scan completes.
    pub async fn remove_channel(&self, scan_id: &str) {
        self.channels.write().await.remove(scan_id);
    }

    /// Send a progress message to all subscribers of a scan.
    pub async fn send_progress(&self, scan_id: &str, msg: &ProgressMessage) {
        if let Some(tx) = self.channels.read().await.get(scan_id) {
            if let Ok(json) = serde_json::to_string(msg) {
                let _ = tx.send(json);
            }
        }
    }
}

/// Handle a WebSocket connection for scan progress streaming (FR-302).
pub async fn handle_scan_progress_ws(socket: WebSocket, broadcaster: WsBroadcaster, scan_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let rx = broadcaster.subscribe(&scan_id).await;
    let Some(mut rx) = rx else {
        let _ = sender
            .send(Message::Text(
                serde_json::json!({"error": "scan not found or already completed"}).to_string().into(),
            ))
            .await;
        return;
    };

    // Forward broadcast messages to the WebSocket client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg == "__DONE__" {
                let _ = sender
                    .send(Message::Text(
                        serde_json::json!({"status": "completed"}).to_string().into(),
                    ))
                    .await;
                break;
            }
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Consume incoming messages (client might send pings)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if matches!(msg, Message::Close(_)) {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

/// Handle a WebSocket connection for AI chat streaming (FR-801).
pub async fn handle_ai_chat_ws(socket: WebSocket, _broadcaster: WsBroadcaster) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(_text) => {
                #[cfg(feature = "ai")]
                {
                    // AI streaming would be implemented here with ComplianceChat
                    let _ = sender
                        .send(Message::Text(
                            serde_json::json!({
                                "type": "response",
                                "content": "AI chat streaming requires the 'ai' feature"
                            })
                            .to_string().into(),
                        ))
                        .await;
                }

                #[cfg(not(feature = "ai"))]
                {
                    let _ = sender
                        .send(Message::Text(
                            serde_json::json!({
                                "type": "error",
                                "content": "AI features not enabled"
                            })
                            .to_string().into(),
                        ))
                        .await;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
