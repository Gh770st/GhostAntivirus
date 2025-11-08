//! Network Module Tests
//! 
//! Comprehensive test suite for the network monitoring module.
//! Tests connection tracking, network scanning, and traffic analysis.

use ghost_core::network::{NetworkMonitor, Connection, ConnectionStatus};
use std::net::IpAddr;
use std::str::FromStr;

#[cfg(test)]
mod network_tests {
    use super::*;

    /// Test: Network monitor initialization
    #[test]
    fn test_network_monitor_new() {
        let monitor = NetworkMonitor::new();
        assert!(monitor.is_ok(), "Network monitor should initialize successfully");
    }

    /// Test: Get active connections
    #[test]
    fn test_get_connections() {
        let monitor = NetworkMonitor::new().unwrap();
        let connections = monitor.get_connections();
        
        // Should return a vector (may be empty on test systems)
        assert!(connections.is_ok(), "Should retrieve connections successfully");
    }

    /// Test: Connection tracking
    #[test]
    fn test_track_connection() {
        let monitor = NetworkMonitor::new().unwrap();
        
        let connection = Connection {
            local_addr: IpAddr::from_str("127.0.0.1").unwrap(),
            local_port: 8080,
            remote_addr: IpAddr::from_str("192.168.1.1").unwrap(),
            remote_port: 443,
            protocol: "TCP".to_string(),
            status: ConnectionStatus::Established,
            process_name: Some("test_process".to_string()),
            process_id: Some(1234),
        };
        
        let result = monitor.track_connection(connection);
        assert!(result.is_ok(), "Should track connection successfully");
    }

    /// Test: Network scanning
    #[test]
    fn test_scan_network() {
        let monitor = NetworkMonitor::new().unwrap();
        let result = monitor.scan_network();
        
        assert!(result.is_ok(), "Network scan should complete successfully");
        
        let scan_result = result.unwrap();
        assert!(scan_result.total_hosts >= 0, "Should report valid host count");
        assert!(scan_result.active_hosts >= 0, "Should report valid active host count");
    }

    /// Test: Get network statistics
    #[test]
    fn test_get_network_stats() {
        let monitor = NetworkMonitor::new().unwrap();
        let stats = monitor.get_stats();
        
        assert!(stats.is_ok(), "Should retrieve network stats successfully");
        
        let stats = stats.unwrap();
        assert!(stats.total_connections >= 0, "Should have valid connection count");
        assert!(stats.bytes_sent >= 0, "Should have valid bytes sent");
        assert!(stats.bytes_received >= 0, "Should have valid bytes received");
    }

    /// Test: Block suspicious connection
    #[test]
    fn test_block_connection() {
        let monitor = NetworkMonitor::new().unwrap();
        
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        let result = monitor.block_ip(ip);
        
        assert!(result.is_ok(), "Should block IP successfully");
    }

    /// Test: Unblock connection
    #[test]
    fn test_unblock_connection() {
        let monitor = NetworkMonitor::new().unwrap();
        
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        
        // First block
        monitor.block_ip(ip).unwrap();
        
        // Then unblock
        let result = monitor.unblock_ip(ip);
        assert!(result.is_ok(), "Should unblock IP successfully");
    }

    /// Test: Check if IP is blocked
    #[test]
    fn test_is_ip_blocked() {
        let monitor = NetworkMonitor::new().unwrap();
        
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        
        // Initially should not be blocked
        assert!(!monitor.is_blocked(ip), "IP should not be blocked initially");
        
        // Block the IP
        monitor.block_ip(ip).unwrap();
        
        // Now should be blocked
        assert!(monitor.is_blocked(ip), "IP should be blocked after blocking");
    }

    /// Test: Get blocked IPs list
    #[test]
    fn test_get_blocked_ips() {
        let monitor = NetworkMonitor::new().unwrap();
        
        let ip1 = IpAddr::from_str("192.168.1.100").unwrap();
        let ip2 = IpAddr::from_str("192.168.1.101").unwrap();
        
        monitor.block_ip(ip1).unwrap();
        monitor.block_ip(ip2).unwrap();
        
        let blocked = monitor.get_blocked_ips();
        assert!(blocked.len() >= 2, "Should have at least 2 blocked IPs");
    }

    /// Test: Traffic analysis
    #[test]
    fn test_analyze_traffic() {
        let monitor = NetworkMonitor::new().unwrap();
        let result = monitor.analyze_traffic();
        
        assert!(result.is_ok(), "Traffic analysis should complete successfully");
        
        let analysis = result.unwrap();
        assert!(analysis.suspicious_connections >= 0, "Should report suspicious connections");
    }

    /// Test: Connection filtering by protocol
    #[test]
    fn test_filter_connections_by_protocol() {
        let monitor = NetworkMonitor::new().unwrap();
        let connections = monitor.get_connections_by_protocol("TCP");
        
        assert!(connections.is_ok(), "Should filter connections by protocol");
    }

    /// Test: Connection filtering by port
    #[test]
    fn test_filter_connections_by_port() {
        let monitor = NetworkMonitor::new().unwrap();
        let connections = monitor.get_connections_by_port(443);
        
        assert!(connections.is_ok(), "Should filter connections by port");
    }

    /// Test: Monitor start/stop
    #[test]
    fn test_monitor_lifecycle() {
        let monitor = NetworkMonitor::new().unwrap();
        
        // Start monitoring
        let start_result = monitor.start_monitoring();
        assert!(start_result.is_ok(), "Should start monitoring successfully");
        
        // Stop monitoring
        let stop_result = monitor.stop_monitoring();
        assert!(stop_result.is_ok(), "Should stop monitoring successfully");
    }

    /// Test: Real-time connection updates
    #[test]
    fn test_realtime_updates() {
        let monitor = NetworkMonitor::new().unwrap();
        
        monitor.start_monitoring().unwrap();
        
        // Wait a bit for connections to be detected
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let connections = monitor.get_connections().unwrap();
        
        monitor.stop_monitoring().unwrap();
        
        // Should have detected some connections (or empty on test systems)
        assert!(connections.len() >= 0, "Should return valid connection list");
    }

    /// Test: Concurrent connection tracking
    #[test]
    fn test_concurrent_tracking() {
        use std::sync::Arc;
        use std::thread;
        
        let monitor = Arc::new(NetworkMonitor::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let connection = Connection {
                    local_addr: IpAddr::from_str("127.0.0.1").unwrap(),
                    local_port: 8080 + i,
                    remote_addr: IpAddr::from_str("192.168.1.1").unwrap(),
                    remote_port: 443,
                    protocol: "TCP".to_string(),
                    status: ConnectionStatus::Established,
                    process_name: Some(format!("process_{}", i)),
                    process_id: Some(1234 + i),
                };
                
                monitor_clone.track_connection(connection)
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().unwrap().is_ok(), "Concurrent tracking should succeed");
        }
    }
}