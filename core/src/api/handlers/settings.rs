// Settings handlers
use axum::{
    extract::State,
    response::Response,
    http::StatusCode,
    Json,
};

use crate::api::{
    AppState,
    models::{SettingsResponse, UpdateSettingsRequest},
};
use super::{success_response, error_response};

/// Get current settings
pub async fn get_settings(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual settings from config
    let config = &engine.config;
    
    let settings = SettingsResponse {
        real_time_protection: engine.monitor.is_running(),
        auto_scan: config.scanner.auto_scan,
        scan_schedule: config.scanner.schedule.clone().unwrap_or_default(),
        auto_update: config.updater.enabled,
        quarantine_days: config.quarantine.auto_delete_days,
        notification_enabled: config.notifications.enabled,
        ai_enabled: config.ai.enabled,
        network_monitoring: engine.network.is_monitoring(),
        firewall_enabled: engine.firewall.is_enabled(),
    };

    success_response(settings)
}

/// Update settings
pub async fn update_settings(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsRequest>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Update settings based on payload
    if let Some(real_time_protection) = payload.real_time_protection {
        if real_time_protection && !engine.monitor.is_running() {
            if let Err(e) = engine.monitor.start().await {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to start real-time protection: {}", e)
                );
            }
        } else if !real_time_protection && engine.monitor.is_running() {
            let _ = engine.monitor.stop().await;
        }
    }
    
    if let Some(ai_enabled) = payload.ai_enabled {
        if ai_enabled {
            engine.ai.enable();
        } else {
            engine.ai.disable();
        }
    }
    
    if let Some(network_monitoring) = payload.network_monitoring {
        if network_monitoring && !engine.network.is_monitoring() {
            if let Err(e) = engine.network.start_monitoring() {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to start network monitoring: {}", e)
                );
            }
        } else if !network_monitoring && engine.network.is_monitoring() {
            engine.network.stop_monitoring();
        }
    }
    
    if let Some(firewall_enabled) = payload.firewall_enabled {
        if firewall_enabled {
            engine.firewall.enable();
        } else {
            engine.firewall.disable();
        }
    }

    success_response(serde_json::json!({
        "message": "Settings updated successfully",
    }))
}