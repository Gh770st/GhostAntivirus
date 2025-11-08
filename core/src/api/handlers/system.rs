// System handlers
use axum::{
    extract::State,
    response::Response,
};
use sysinfo::{System, SystemExt, CpuExt, NetworkExt, NetworksExt};

use crate::api::{
    AppState,
    models::{SystemInfoResponse, SystemStatsResponse, NetworkUsage},
};
use super::success_response;

/// Get system information
pub async fn get_info(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual system info using sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let info = SystemInfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_cores: sys.cpus().len(),
        total_memory: sys.total_memory(),
        available_memory: sys.available_memory(),
        uptime: sys.uptime(),
    };

    success_response(info)
}

/// Get system statistics
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual system stats using sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Calculate CPU usage
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    
    // Calculate memory usage percentage
    let memory_usage = if sys.total_memory() > 0 {
        ((sys.total_memory() - sys.available_memory()) as f64 / sys.total_memory() as f64) * 100.0
    } else {
        0.0
    };
    
    // Get disk usage (simplified - just root partition)
    let disk_usage = 0.0; // Would need additional logic for accurate disk usage
    
    // Get network statistics
    let networks = sys.networks();
    let mut total_bytes_sent = 0u64;
    let mut total_bytes_received = 0u64;
    let mut total_packets_sent = 0u64;
    let mut total_packets_received = 0u64;
    
    for (_interface_name, network) in networks {
        total_bytes_sent += network.total_transmitted();
        total_bytes_received += network.total_received();
        total_packets_sent += network.total_packets_transmitted();
        total_packets_received += network.total_packets_received();
    }
    
    let stats = SystemStatsResponse {
        cpu_usage: cpu_usage as f64,
        memory_usage,
        disk_usage,
        network_usage: NetworkUsage {
            bytes_sent: total_bytes_sent,
            bytes_received: total_bytes_received,
            packets_sent: total_packets_sent,
            packets_received: total_packets_received,
        },
    };

    success_response(stats)
}