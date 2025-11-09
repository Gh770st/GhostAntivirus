//! WebSocket Broadcasting Module
//!
//! Provides real-time communication for broadcasting events
//! to connected WebSocket clients.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, RwLock};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::{info, warn, debug};

use crate::config::Config;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WSMessage {
    /// Scan progress update
    ScanProgress {
        scan_id: String,
        files_scanned: u64,
        total_files: u64,
        threats_found: u64,
        progress_percent: f32,
    },
    /// Threat detected
    ThreatDetected {
        threat_id: String,
        file_path: String,
        threat_name: String,
        threat_type: String,
        severity: String,
        timestamp: u64,
    },
    /// System status update
    SystemStatus {
        cpu_usage: f32,
        memory_usage: f32,
        disk_usage: f32,
        active_processes: u32,
        network_connections: u32,
    },
    /// Update available
    UpdateAvailable {
        version: String,
        description: String,
        is_critical: bool,
        requires_restart: bool,
    },
    /// Scan completed
    ScanCompleted {
        scan_id: String,
        total_files: u64,
        threats_found: u64,
        duration_seconds: u64,
    },
    /// Alert message
    Alert {
        level: String,
        title: String,
        message: String,
        timestamp: u64,
    },
}

/// WebSocket broadcaster for real-time updates
#[derive(Debug)]
pub struct WSBroadcaster {
    /// Broadcast channel for messages
    tx: broadcast::Sender<WSMessage>,
    /// Connected clients
    clients: Arc<RwLock<HashMap<String, ClientInfo>>>,
    /// Configuration
    config: Config,
}

/// Client information
#[derive(Debug, Clone)]
struct ClientInfo {
    /// Client ID
    id: String,
    /// Connected timestamp
    connected_at: std::time::SystemTime,
    /// Client type (web, mobile, etc.)
    client_type: String,
}

impl WSBroadcaster {
    /// Create new WebSocket broadcaster
    pub fn new(config: Config) -> Result<Self> {
        let (tx, _rx) = broadcast::channel(1000);
        
        Ok(Self {
            tx,
            clients: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }
    
    /// Get a new receiver for the broadcast channel
    pub fn subscribe(&self) -> broadcast::Receiver<WSMessage> {
        self.tx.subscribe()
    }
    
    /// Broadcast message to all connected clients
    pub async fn broadcast(&self, message: WSMessage) -> Result<()> {
        debug!("Broadcasting WebSocket message: {:?}", message);
        
        match self.tx.send(message) {
            Ok(receiver_count) => {
                debug!("Message sent to {} receivers", receiver_count);
            }
            Err(_) => {
                warn!("No active receivers for WebSocket broadcast");
            }
        }
        
        Ok(())
    }
    
    /// Add client to connected list
    pub async fn add_client(&self, client_id: String, client_type: String) {
        let mut clients = self.clients.write().await;
        let client_id_clone = client_id.clone();
        clients.insert(client_id, ClientInfo {
            id: client_id_clone.clone(),
            connected_at: std::time::SystemTime::now(),
            client_type,
        });
        
        info!("WebSocket client connected: {}", client_id_clone);
        debug!("Total connected clients: {}", clients.len());
    }
    
    /// Remove client from connected list
    pub async fn remove_client(&self, client_id: &str) {
        let mut clients = self.clients.write().await;
        clients.remove(client_id);
        
        info!("WebSocket client disconnected: {}", client_id);
        debug!("Total connected clients: {}", clients.len());
    }
    
    /// Get connected clients count
    pub async fn get_client_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.len()
    }
    
    /// Get list of connected clients
    pub async fn get_clients(&self) -> Vec<ClientInfo> {
        let clients = self.clients.read().await;
        clients.values().cloned().collect()
    }
    
    /// Send scan progress update
    pub async fn send_scan_progress(
        &self,
        scan_id: String,
        files_scanned: u64,
        total_files: u64,
        threats_found: u64,
    ) -> Result<()> {
        let progress_percent = if total_files > 0 {
            (files_scanned as f32 / total_files as f32) * 100.0
        } else {
            0.0
        };
        
        self.broadcast(WSMessage::ScanProgress {
            scan_id,
            files_scanned,
            total_files,
            threats_found,
            progress_percent,
        }).await
    }
    
    /// Send threat detected notification
    pub async fn send_threat_detected(
        &self,
        threat_id: String,
        file_path: String,
        threat_name: String,
        threat_type: String,
        severity: String,
    ) -> Result<()> {
        self.broadcast(WSMessage::ThreatDetected {
            threat_id,
            file_path,
            threat_name,
            threat_type,
            severity,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }).await
    }
    
    /// Send system status update
    pub async fn send_system_status(
        &self,
        cpu_usage: f32,
        memory_usage: f32,
        disk_usage: f32,
        active_processes: u32,
        network_connections: u32,
    ) -> Result<()> {
        self.broadcast(WSMessage::SystemStatus {
            cpu_usage,
            memory_usage,
            disk_usage,
            active_processes,
            network_connections,
        }).await
    }
    
    /// Send scan completed notification
    pub async fn send_scan_completed(
        &self,
        scan_id: String,
        total_files: u64,
        threats_found: u64,
        duration_seconds: u64,
    ) -> Result<()> {
        self.broadcast(WSMessage::ScanCompleted {
            scan_id,
            total_files,
            threats_found,
            duration_seconds,
        }).await
    }
    
    /// Send alert message
    pub async fn send_alert(
        &self,
        level: String,
        title: String,
        message: String,
    ) -> Result<()> {
        self.broadcast(WSMessage::Alert {
            level,
            title,
            message,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }).await
    }
    
    /// Send update available notification
    pub async fn send_update_available(
        &self,
        version: String,
        description: String,
        is_critical: bool,
        requires_restart: bool,
    ) -> Result<()> {
        self.broadcast(WSMessage::UpdateAvailable {
            version,
            description,
            is_critical,
            requires_restart,
        }).await
    }
}

/// Global WebSocket broadcaster instance
lazy_static::lazy_static! {
    static ref GLOBAL_BROADCASTER: Arc<Mutex<Option<WSBroadcaster>>> = Arc::new(Mutex::new(None));
}

/// Initialize global WebSocket broadcaster
pub fn initialize_broadcaster(config: Config) -> Result<()> {
    let broadcaster = WSBroadcaster::new(config)?;
    let mut global = GLOBAL_BROADCASTER.lock().unwrap();
    *global = Some(broadcaster);
    info!("Global WebSocket broadcaster initialized");
    Ok(())
}

/// Get global WebSocket broadcaster
pub fn get_broadcaster() -> Option<WSBroadcaster> {
    let global = GLOBAL_BROADCASTER.lock().unwrap();
    global.as_ref().map(|b| {
        WSBroadcaster {
            tx: b.tx.clone(),
            clients: Arc::clone(&b.clients),
            config: b.config.clone(),
        }
    })
}

/// Broadcast message using global broadcaster
pub async fn broadcast_global(message: WSMessage) -> Result<()> {
    if let Some(broadcaster) = get_broadcaster() {
        broadcaster.broadcast(message).await
    } else {
        warn!("Global WebSocket broadcaster not initialized");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_serialization() {
        let message = WSMessage::ScanProgress {
            scan_id: "test".to_string(),
            files_scanned: 10,
            total_files: 100,
            threats_found: 1,
            progress_percent: 10.0,
        };
        
        let json = serde_json::to_string(&message);
        assert!(json.is_ok());
    }
    
    #[tokio::test]
    async fn test_broadcaster_creation() {
        let config = Config::default();
        let broadcaster = WSBroadcaster::new(config);
        assert!(broadcaster.is_ok());
    }
}