// Quarantine handlers
use axum::{
    extract::{State, Path},
    response::Response,
    http::StatusCode,
    Json,
};
use chrono::{Utc, DateTime};

use crate::api::{
    AppState,
    models::{QuarantineFile, QuarantineListResponse, QuarantineStatsResponse},
};
use super::{success_response, error_response};

/// List quarantined files
pub async fn list_files(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual quarantined files
    let entries = engine.quarantine.list_files();
    
    let files: Vec<QuarantineFile> = entries.iter().map(|entry| {
        QuarantineFile {
            id: entry.id.clone(),
            original_path: entry.original_path.to_string_lossy().to_string(),
            quarantine_path: entry.quarantine_path.to_string_lossy().to_string(),
            quarantined_at: DateTime::from_timestamp(
                entry.quarantined_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                0
            ).unwrap_or(Utc::now()),
            threat_type: entry.threat_type.clone(),
            size: entry.file_size,
            hash: entry.file_hash.clone(),
        }
    }).collect();
    
    let total = files.len();

    let response = QuarantineListResponse {
        files,
        total,
    };

    success_response(response)
}

/// Get quarantined file details
pub async fn get_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual file details
    match engine.quarantine.get_file_info(&id) {
        Some(entry) => {
            let file = QuarantineFile {
                id: entry.id.clone(),
                original_path: entry.original_path.to_string_lossy().to_string(),
                quarantine_path: entry.quarantine_path.to_string_lossy().to_string(),
                quarantined_at: DateTime::from_timestamp(
                    entry.quarantined_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0
                ).unwrap_or(Utc::now()),
                threat_type: entry.threat_type.clone(),
                size: entry.file_size,
                hash: entry.file_hash.clone(),
            };
            success_response(file)
        }
        None => {
            error_response(StatusCode::NOT_FOUND, format!("Quarantined file not found: {}", id))
        }
    }
}

/// Restore quarantined file
pub async fn restore_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Restore the file
    match engine.quarantine.restore_file(&id) {
        Ok(restored_path) => {
            success_response(serde_json::json!({
                "message": "File restored successfully",
                "file_id": id,
                "restored_path": restored_path.to_string_lossy(),
            }))
        }
        Err(e) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to restore file: {}", e))
        }
    }
}

/// Delete quarantined file
pub async fn delete_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Delete the file
    match engine.quarantine.delete_file(&id) {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "File deleted successfully",
                "file_id": id,
            }))
        }
        Err(e) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete file: {}", e))
        }
    }
}

/// Get quarantine statistics
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual stats
    let stats = engine.quarantine.get_statistics();
    let entries = engine.quarantine.list_files();
    
    // Find oldest and newest files
    let oldest_file = entries.iter()
        .min_by_key(|e| e.quarantined_at)
        .map(|e| DateTime::from_timestamp(
            e.quarantined_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            0
        ).unwrap_or(Utc::now()));
    
    let newest_file = entries.iter()
        .max_by_key(|e| e.quarantined_at)
        .map(|e| DateTime::from_timestamp(
            e.quarantined_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            0
        ).unwrap_or(Utc::now()));
    
    let response = QuarantineStatsResponse {
        total_files: stats.total_files,
        total_size: stats.total_size,
        oldest_file,
        newest_file,
    };

    success_response(response)
}