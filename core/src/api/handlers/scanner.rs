// Scanner handlers
use axum::{
    extract::State,
    response::Response,
    http::StatusCode,
    Json,
};
use chrono::{Utc, DateTime};

use crate::api::{
    AppState,
    models::{
        ScanStatsResponse, StartScanRequest, ScanResultsResponse, 
        ScanResult, ScanType,
    },
};
use super::{error_response, success_response};

/// Get scan statistics
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get real stats from scanner
    let scanner_stats = engine.scanner.get_statistics();
    
    let last_scan = if let Some(timestamp) = scanner_stats.last_scan_time {
        Some(DateTime::from_timestamp(timestamp as i64, 0).unwrap_or(Utc::now()))
    } else {
        None
    };
    
    let stats = ScanStatsResponse {
        total_scanned: scanner_stats.total_files_scanned,
        threats_found: scanner_stats.threats_detected,
        files_quarantined: scanner_stats.files_quarantined,
        last_scan,
        scan_in_progress: scanner_stats.is_scanning,
        scan_progress: scanner_stats.progress_percentage,
    };

    success_response(stats)
}

/// Start a new scan
pub async fn start_scan(
    State(state): State<AppState>,
    Json(payload): Json<StartScanRequest>,
) -> Response {
    let engine = state.engine.read().await;

    // Validate path
    if !std::path::Path::new(&payload.path).exists() {
        return error_response(StatusCode::BAD_REQUEST, "Path does not exist".to_string());
    }

    // Start scan based on type
    let scan_result = engine.scanner.start_scan(
        &payload.path,
        payload.scan_type.clone(),
        payload.deep_scan.unwrap_or(false)
    ).await;

    match scan_result {
        Ok(scan_id) => {
            success_response(serde_json::json!({
                "scan_id": scan_id,
                "message": "Scan started successfully",
                "path": payload.path,
                "scan_type": format!("{:?}", payload.scan_type),
            }))
        }
        Err(e) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// Stop current scan
pub async fn stop_scan(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;

    match engine.scanner.stop_scan().await {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Scan stopped successfully"
            }))
        }
        Err(e) => {
            error_response(StatusCode::BAD_REQUEST, e.to_string())
        }
    }
}

/// Pause current scan
pub async fn pause_scan(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;

    match engine.scanner.pause_scan().await {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Scan paused successfully"
            }))
        }
        Err(e) => {
            error_response(StatusCode::BAD_REQUEST, e.to_string())
        }
    }
}

/// Resume paused scan
pub async fn resume_scan(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;

    match engine.scanner.resume_scan().await {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Scan resumed successfully"
            }))
        }
        Err(e) => {
            error_response(StatusCode::BAD_REQUEST, e.to_string())
        }
    }
}

/// Get scan results
pub async fn get_results(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;

    // Get real results from analyzer
    let threats = engine.analyzer.get_detected_threats();
    
    let results: Vec<ScanResult> = threats.iter().map(|threat| {
        ScanResult {
            id: threat.id.clone(),
            path: threat.file_path.to_string_lossy().to_string(),
            threat_type: Some(format!("{:?}", threat.threat_type)),
            severity: format!("{:?}", threat.severity),
            timestamp: DateTime::from_timestamp(threat.detected_at as i64, 0).unwrap_or(Utc::now()),
            action_taken: threat.action_taken.clone(),
        }
    }).collect();

    let total = results.len();
    
    let response = ScanResultsResponse {
        results,
        total,
        page: 1,
        per_page: 10,
    };

    success_response(response)
}