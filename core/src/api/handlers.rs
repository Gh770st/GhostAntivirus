// API handlers module
pub mod auth;
pub mod scanner;
pub mod threats;
pub mod quarantine;
pub mod firewall;
pub mod network;
pub mod settings;
pub mod system;
pub mod updates;
pub mod websocket;

use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::models::{ApiResponse, HealthCheckResponse, ServiceStatus};

/// Health check endpoint
pub async fn health_check() -> Response {
    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: 0, // TODO: Calculate actual uptime
        services: ServiceStatus {
            scanner: true,
            ai_engine: true,
            network_guard: true,
            database: true,
        },
    };

    (StatusCode::OK, Json(ApiResponse::success(response))).into_response()
}

/// Error response helper
pub fn error_response(status: StatusCode, message: String) -> Response {
    let body = json!({
        "success": false,
        "error": message,
        "timestamp": chrono::Utc::now(),
    });

    (status, Json(body)).into_response()
}

/// Success response helper
pub fn success_response<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(ApiResponse::success(data))).into_response()
}