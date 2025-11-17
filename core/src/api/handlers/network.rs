// Network handlers
use axum::{
    extract::State,
    response::Response,
};
use chrono::{Utc, DateTime};

use crate::api::{
    AppState,
    models::{NetworkConnection, NetworkConnectionsResponse, NetworkStatsResponse},
};
use super::success_response;

/// List network connections
pub async fn list_connections(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual connections from network monitor
    let active_connections = engine.network.get_connections();
    
    let connections: Vec<NetworkConnection> = active_connections.iter().map(|conn| {
        NetworkConnection {
            id: conn.id.clone(),
            process_name: conn.process_name.clone(),
            process_id: conn.process_id,
            local_address: conn.local_addr.to_string(),
            local_port: conn.local_port,
            remote_address: conn.remote_addr.to_string(),
            remote_port: conn.remote_port,
            protocol: format!("{:?}", conn.protocol),
            state: format!("{:?}", conn.state),
            bytes_sent: conn.bytes_sent,
            bytes_received: conn.bytes_received,
            established_at: DateTime::from_timestamp(
                conn.established_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                0
            ).unwrap_or(Utc::now()),
        }
    }).collect();
    
    let total = connections.len();

    let response = NetworkConnectionsResponse {
        connections,
        total,
    };

    success_response(response)
}

/// Get network statistics
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual stats from network monitor
    let stats = engine.network.get_statistics();
    
    let response = NetworkStatsResponse {
        total_connections: stats.total_connections,
        active_connections: stats.active_connections,
        blocked_connections: stats.blocked_connections,
        total_bytes_sent: stats.bytes_sent,
        total_bytes_received: stats.bytes_received,
    };

    success_response(response)
}

/// Scan network
pub async fn scan_network(State(state): State<AppState>) -> Response {
    let mut engine = state.engine.write().await;
    
    // Start network scan
    match engine.network.scan_network().await {
        Ok(scan_id) => {
            success_response(serde_json::json!({
                "message": "Network scan started",
                "scan_id": scan_id,
            }))
        }
        Err(e) => {
            success_response(serde_json::json!({
                "message": "Network scan failed to start",
                "error": e.to_string(),
            }))
        }
    }
}