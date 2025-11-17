// Threat handlers
use axum::{
    extract::{State, Path},
    response::Response,
    http::StatusCode,
};
use chrono::{Utc, DateTime};

use crate::api::{
    AppState,
    models::{ThreatInfo, ThreatListResponse, ThreatSeverity, ThreatStatus},
};
use crate::scanner::Severity;
use super::{error_response, success_response};

/// Convert internal Severity to API ThreatSeverity
fn map_severity(severity: &Severity) -> ThreatSeverity {
    match severity {
        Severity::Low => ThreatSeverity::Low,
        Severity::Medium => ThreatSeverity::Medium,
        Severity::High => ThreatSeverity::High,
        Severity::Critical => ThreatSeverity::Critical,
    }
}

/// List all threats
pub async fn list_threats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual threats from analyzer
    let detected_threats = engine.analyzer.get_detected_threats();
    
    let threats: Vec<ThreatInfo> = detected_threats.iter().map(|threat| {
        // Determine status based on whether file is in quarantine
        let status = if engine.quarantine.get_file_info(&threat.id).is_some() {
            ThreatStatus::Quarantined
        } else {
            ThreatStatus::Detected
        };
        
        ThreatInfo {
            id: threat.id.clone(),
            name: threat.threat_name.clone(),
            path: threat.file_path.to_string_lossy().to_string(),
            threat_type: format!("{:?}", threat.threat_type),
            severity: map_severity(&threat.severity),
            detected_at: DateTime::from_timestamp(threat.detected_at as i64, 0).unwrap_or(Utc::now()),
            status,
            hash: threat.file_hash.clone(),
            size: 0, // Size not stored in DetectedThreat, would need to be added
        }
    }).collect();
    
    let total = threats.len();

    let response = ThreatListResponse {
        threats,
        total,
    };

    success_response(response)
}

/// Get threat details
pub async fn get_threat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual threat from analyzer
    let detected_threats = engine.analyzer.get_detected_threats();
    
    match detected_threats.iter().find(|t| t.id == id) {
        Some(threat) => {
            let status = if engine.quarantine.get_file_info(&threat.id).is_some() {
                ThreatStatus::Quarantined
            } else {
                ThreatStatus::Detected
            };
            
            let threat_info = ThreatInfo {
                id: threat.id.clone(),
                name: threat.threat_name.clone(),
                path: threat.file_path.to_string_lossy().to_string(),
                threat_type: format!("{:?}", threat.threat_type),
                severity: map_severity(&threat.severity),
                detected_at: DateTime::from_timestamp(threat.detected_at as i64, 0).unwrap_or(Utc::now()),
                status,
                hash: threat.file_hash.clone(),
                size: 0,
            };
            
            success_response(threat_info)
        }
        None => {
            error_response(StatusCode::NOT_FOUND, format!("Threat not found: {}", id))
        }
    }
}

/// Quarantine a threat
pub async fn quarantine_threat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Find the threat
    let detected_threats = engine.analyzer.get_detected_threats();
    match detected_threats.iter().find(|t| t.id == id) {
        Some(threat) => {
            // Check if file still exists
            if !threat.file_path.exists() {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    "File no longer exists at original location".to_string()
                );
            }
            
            // Quarantine the file
            match engine.quarantine.quarantine_file(&threat.file_path, &threat.threat_name) {
                Ok(quarantine_id) => {
                    success_response(serde_json::json!({
                        "message": "Threat quarantined successfully",
                        "threat_id": id,
                        "quarantine_id": quarantine_id,
                    }))
                }
                Err(e) => {
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to quarantine threat: {}", e)
                    )
                }
            }
        }
        None => {
            error_response(StatusCode::NOT_FOUND, format!("Threat not found: {}", id))
        }
    }
}

/// Remove a threat
pub async fn remove_threat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Remove from analyzer's threat list
    match engine.analyzer.remove_threat(&id) {
        Ok(_) => {
            // Also try to remove from quarantine if it exists there
            let _ = engine.quarantine.delete_file(&id);
            
            success_response(serde_json::json!({
                "message": "Threat removed successfully",
                "threat_id": id,
            }))
        }
        Err(e) => {
            error_response(
                StatusCode::NOT_FOUND,
                format!("Failed to remove threat: {}", e)
            )
        }
    }
}

/// Restore a threat from quarantine
pub async fn restore_threat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Restore from quarantine
    match engine.quarantine.restore_file(&id) {
        Ok(restored_path) => {
            success_response(serde_json::json!({
                "message": "Threat restored successfully",
                "threat_id": id,
                "restored_path": restored_path.to_string_lossy(),
            }))
        }
        Err(e) => {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to restore threat: {}", e)
            )
        }
    }
}