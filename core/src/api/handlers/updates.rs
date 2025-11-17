// Update handlers
use axum::{
    extract::State,
    response::Response,
    http::StatusCode,
    Json,
};

use crate::api::{
    AppState,
    models::{UpdateCheckResponse, ApplyUpdateRequest},
};
use super::{success_response, error_response};

/// Check for updates
pub async fn check_updates(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Check for actual updates using the updater
    match engine.updater.check_for_updates().await {
        Ok(update_info) => {
            let response = if let Some(update) = update_info {
                UpdateCheckResponse {
                    update_available: true,
                    current_version: env!("CARGO_PKG_VERSION").to_string(),
                    latest_version: Some(update.version.to_string()),
                    release_notes: Some(update.release_notes),
                    download_url: Some(update.download_url),
                }
            } else {
                UpdateCheckResponse {
                    update_available: false,
                    current_version: env!("CARGO_PKG_VERSION").to_string(),
                    latest_version: None,
                    release_notes: None,
                    download_url: None,
                }
            };
            
            success_response(response)
        }
        Err(e) => {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to check for updates: {}", e)
            )
        }
    }
}

/// Apply update
pub async fn apply_update(
    State(state): State<AppState>,
    Json(payload): Json<ApplyUpdateRequest>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Apply the update using the updater
    // Apply the update
    match engine.updater.apply_update_async(&payload.version, payload.auto_restart).await {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Update applied successfully",
                "version": payload.version,
                "restart_required": payload.auto_restart,
            }))
        }
        Err(e) => {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to apply update: {}", e)
            )
        }
    }
}