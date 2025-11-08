// WebSocket handler
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::{
    AppState,
    models::{WebSocketMessage, ScanProgressUpdate, ThreatAlert, ThreatSeverity},
};

/// Global broadcast channel for WebSocket messages
lazy_static::lazy_static! {
    static ref WS_BROADCAST: Arc<RwLock<Option<broadcast::Sender<String>>>> = Arc::new(RwLock::new(None));
}

/// Initialize WebSocket broadcast channel
pub async fn init_broadcast_channel() {
    let (tx, _) = broadcast::channel::<String>(100);
    let mut broadcast = WS_BROADCAST.write().await;
    *broadcast = Some(tx);
}

/// WebSocket handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Subscribe to broadcast channel
    let broadcast_lock = WS_BROADCAST.read().await;
    let mut rx = if let Some(tx) = broadcast_lock.as_ref() {
        tx.subscribe()
    } else {
        // If no broadcast channel, create a dummy one
        let (tx, rx) = broadcast::channel::<String>(1);
        drop(tx);
        rx
    };
    drop(broadcast_lock);
    
    // Spawn a task to send messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // Spawn a task to receive messages from the client
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::info!("Received WebSocket message: {}", text);
                    // Handle incoming messages - could be commands or requests
                    if let Err(e) = handle_client_message(&text, &state_clone).await {
                        tracing::error!("Error handling client message: {}", e);
                    }
                }
                Message::Close(_) => {
                    tracing::info!("WebSocket client disconnected");
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Spawn a task to send periodic updates with real data
    let state_clone = state.clone();
    let update_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        loop {
            interval.tick().await;
            
            // Get real scan progress from engine
            let engine = state_clone.engine.read().await;
            let scanner_stats = engine.scanner.get_statistics();
            
            if scanner_stats.is_scanning {
                let progress = ScanProgressUpdate {
                    progress: scanner_stats.progress_percentage as f64,
                    current_file: "Scanning...".to_string(), // Could be enhanced to show actual file
                    files_scanned: scanner_stats.total_files_scanned,
                    threats_found: scanner_stats.threats_detected,
                };
                
                let ws_msg = WebSocketMessage {
                    event: "scan_progress".to_string(),
                    data: serde_json::to_value(&progress).unwrap(),
                    timestamp: Utc::now(),
                };
                
                if let Ok(json) = serde_json::to_string(&ws_msg) {
                    let _ = broadcast_message(json).await;
                }
            }
        }
    });
    
    // Wait for any task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
            update_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
            update_task.abort();
        },
    }
}

/// Handle incoming client messages
async fn handle_client_message(text: &str, state: &AppState) -> anyhow::Result<()> {
    // Parse message and handle commands
    // For example: {"command": "get_status"}
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(command) = value.get("command").and_then(|v| v.as_str()) {
            match command {
                "get_status" => {
                    let engine = state.engine.read().await;
                    let status = engine.get_status();
                    let response = WebSocketMessage {
                        event: "status".to_string(),
                        data: serde_json::to_value(&status)?,
                        timestamp: Utc::now(),
                    };
                    broadcast_message(serde_json::to_string(&response)?).await?;
                }
                _ => {
                    tracing::warn!("Unknown command: {}", command);
                }
            }
        }
    }
    Ok(())
}

/// Broadcast a message to all connected WebSocket clients
async fn broadcast_message(message: String) -> anyhow::Result<()> {
    let broadcast_lock = WS_BROADCAST.read().await;
    if let Some(tx) = broadcast_lock.as_ref() {
        let _ = tx.send(message);
    }
    Ok(())
}

/// Broadcast a threat alert to all connected clients
pub async fn broadcast_threat_alert(alert: ThreatAlert) {
    let ws_msg = WebSocketMessage {
        event: "threat_alert".to_string(),
        data: serde_json::to_value(&alert).unwrap_or_default(),
        timestamp: Utc::now(),
    };
    
    if let Ok(json) = serde_json::to_string(&ws_msg) {
        let _ = broadcast_message(json).await;
    }
    
    tracing::info!("Broadcasted threat alert: {:?}", alert);
}

/// Broadcast scan progress to all connected clients
pub async fn broadcast_scan_progress(progress: ScanProgressUpdate) {
    let ws_msg = WebSocketMessage {
        event: "scan_progress".to_string(),
        data: serde_json::to_value(&progress).unwrap_or_default(),
        timestamp: Utc::now(),
    };
    
    if let Ok(json) = serde_json::to_string(&ws_msg) {
        let _ = broadcast_message(json).await;
    }
    
    tracing::debug!("Broadcasted scan progress: {:?}", progress);
}