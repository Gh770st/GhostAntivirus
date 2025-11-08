// API routes configuration
use axum::Router;
use crate::api::AppState;

/// Configure all API routes
pub fn configure_routes(state: AppState) -> Router {
    Router::new()
        .merge(health_routes())
        .merge(auth_routes())
        .merge(scanner_routes())
        .merge(threat_routes())
        .merge(quarantine_routes())
        .merge(firewall_routes())
        .merge(network_routes())
        .merge(settings_routes())
        .merge(system_routes())
        .merge(update_routes())
        .merge(websocket_routes())
        .with_state(state)
}

fn health_routes() -> Router<AppState> {
    use axum::routing::get;
    use crate::api::handlers::health_check;
    
    Router::new()
        .route("/health", get(health_check))
}

fn auth_routes() -> Router<AppState> {
    use axum::routing::post;
    use crate::api::handlers::auth;
    
    Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/refresh", post(auth::refresh_token))
}

fn scanner_routes() -> Router<AppState> {
    use axum::routing::{get, post};
    use crate::api::handlers::scanner;
    
    Router::new()
        .route("/api/scan/stats", get(scanner::get_stats))
        .route("/api/scan/start", post(scanner::start_scan))
        .route("/api/scan/stop", post(scanner::stop_scan))
        .route("/api/scan/pause", post(scanner::pause_scan))
        .route("/api/scan/resume", post(scanner::resume_scan))
        .route("/api/scan/results", get(scanner::get_results))
}

fn threat_routes() -> Router<AppState> {
    use axum::routing::{get, post, delete};
    use crate::api::handlers::threats;
    
    Router::new()
        .route("/api/threats", get(threats::list_threats))
        .route("/api/threats/:id", get(threats::get_threat))
        .route("/api/threats/:id/quarantine", post(threats::quarantine_threat))
        .route("/api/threats/:id/remove", delete(threats::remove_threat))
        .route("/api/threats/:id/restore", post(threats::restore_threat))
}

fn quarantine_routes() -> Router<AppState> {
    use axum::routing::{get, post, delete};
    use crate::api::handlers::quarantine;
    
    Router::new()
        .route("/api/quarantine", get(quarantine::list_files))
        .route("/api/quarantine/:id", get(quarantine::get_file))
        .route("/api/quarantine/:id/restore", post(quarantine::restore_file))
        .route("/api/quarantine/:id/delete", delete(quarantine::delete_file))
        .route("/api/quarantine/stats", get(quarantine::get_stats))
}

fn firewall_routes() -> Router<AppState> {
    use axum::routing::{get, post, put, delete};
    use crate::api::handlers::firewall;
    
    Router::new()
        .route("/api/firewall/rules", get(firewall::list_rules))
        .route("/api/firewall/rules", post(firewall::add_rule))
        .route("/api/firewall/rules/:id", put(firewall::update_rule))
        .route("/api/firewall/rules/:id", delete(firewall::delete_rule))
        .route("/api/firewall/stats", get(firewall::get_stats))
}

fn network_routes() -> Router<AppState> {
    use axum::routing::{get, post};
    use crate::api::handlers::network;
    
    Router::new()
        .route("/api/network/connections", get(network::list_connections))
        .route("/api/network/stats", get(network::get_stats))
        .route("/api/network/scan", post(network::scan_network))
}

fn settings_routes() -> Router<AppState> {
    use axum::routing::{get, put};
    use crate::api::handlers::settings;
    
    Router::new()
        .route("/api/settings", get(settings::get_settings))
        .route("/api/settings", put(settings::update_settings))
}

fn system_routes() -> Router<AppState> {
    use axum::routing::get;
    use crate::api::handlers::system;
    
    Router::new()
        .route("/api/system/info", get(system::get_info))
        .route("/api/system/stats", get(system::get_stats))
}

fn update_routes() -> Router<AppState> {
    use axum::routing::{get, post};
    use crate::api::handlers::updates;
    
    Router::new()
        .route("/api/updates/check", get(updates::check_updates))
        .route("/api/updates/apply", post(updates::apply_update))
}

fn websocket_routes() -> Router<AppState> {
    use axum::routing::get;
    use crate::api::handlers::websocket;
    
    Router::new()
        .route("/ws", get(websocket::ws_handler))
}