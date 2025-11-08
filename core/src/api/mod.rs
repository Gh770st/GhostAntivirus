// API module for REST endpoints
pub mod routes;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod websocket;

use axum::{
    Router,
    routing::{get, post, put, delete},
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::GhostEngine;

/// API Server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub enable_cors: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            enable_cors: true,
        }
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<GhostEngine>>,
    pub config: ApiConfig,
}

/// Create the API router with all routes
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // Authentication
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/refresh", post(handlers::auth::refresh_token))
        
        // Scanner endpoints
        .route("/api/scan/stats", get(handlers::scanner::get_stats))
        .route("/api/scan/start", post(handlers::scanner::start_scan))
        .route("/api/scan/stop", post(handlers::scanner::stop_scan))
        .route("/api/scan/pause", post(handlers::scanner::pause_scan))
        .route("/api/scan/resume", post(handlers::scanner::resume_scan))
        .route("/api/scan/results", get(handlers::scanner::get_results))
        
        // Threats endpoints
        .route("/api/threats", get(handlers::threats::list_threats))
        .route("/api/threats/:id", get(handlers::threats::get_threat))
        .route("/api/threats/:id/quarantine", post(handlers::threats::quarantine_threat))
        .route("/api/threats/:id/remove", delete(handlers::threats::remove_threat))
        .route("/api/threats/:id/restore", post(handlers::threats::restore_threat))
        
        // Quarantine endpoints
        .route("/api/quarantine", get(handlers::quarantine::list_files))
        .route("/api/quarantine/:id", get(handlers::quarantine::get_file))
        .route("/api/quarantine/:id/restore", post(handlers::quarantine::restore_file))
        .route("/api/quarantine/:id/delete", delete(handlers::quarantine::delete_file))
        .route("/api/quarantine/stats", get(handlers::quarantine::get_stats))
        
        // Firewall endpoints
        .route("/api/firewall/rules", get(handlers::firewall::list_rules))
        .route("/api/firewall/rules", post(handlers::firewall::add_rule))
        .route("/api/firewall/rules/:id", put(handlers::firewall::update_rule))
        .route("/api/firewall/rules/:id", delete(handlers::firewall::delete_rule))
        .route("/api/firewall/stats", get(handlers::firewall::get_stats))
        
        // Network endpoints
        .route("/api/network/connections", get(handlers::network::list_connections))
        .route("/api/network/stats", get(handlers::network::get_stats))
        .route("/api/network/scan", post(handlers::network::scan_network))
        
        // Settings endpoints
        .route("/api/settings", get(handlers::settings::get_settings))
        .route("/api/settings", put(handlers::settings::update_settings))
        
        // System endpoints
        .route("/api/system/info", get(handlers::system::get_info))
        .route("/api/system/stats", get(handlers::system::get_stats))
        
        // Updates endpoints
        .route("/api/updates/check", get(handlers::updates::check_updates))
        .route("/api/updates/apply", post(handlers::updates::apply_update))
        
        // WebSocket endpoint for real-time updates
        .route("/ws", get(handlers::websocket::ws_handler))
        
        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/// Start the API server
pub async fn start_server(engine: GhostEngine, config: ApiConfig) -> anyhow::Result<()> {
    let state = AppState {
        engine: Arc::new(RwLock::new(engine)),
        config: config.clone(),
    };

    let app = create_router(state);
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("API server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert!(config.enable_cors);
    }
}