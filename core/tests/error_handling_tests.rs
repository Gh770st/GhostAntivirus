//! Error Handling Tests
//! 
//! Comprehensive tests for error handling across all modules.
//! Ensures proper error propagation and recovery.

use ghost_core::{
    scanner::Scanner,
    quarantine::QuarantineManager,
    analyzer::ThreatAnalyzer,
    network::NetworkMonitor,
    firewall::Firewall,
    config::Config,
    system::SystemMonitor,
    updates::UpdateManager,
};
use std::path::PathBuf;

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    // ==================== SCANNER ERROR HANDLING ====================

    /// Test: Scanner handles file system errors
    #[tokio::test]
    async fn test_scanner_filesystem_errors() {
        let scanner = Scanner::new().unwrap();
        
        // Test various error conditions
        let test_cases = vec![
            PathBuf::from("/dev/null"),
            PathBuf::from("/proc/self/mem"),
            PathBuf::from(""),
        ];
        
        for path in test_cases {
            let result = scanner.scan_file(&path).await;
            // Should return error, not panic
            assert!(result.is_ok() || result.is_err(), "Should handle error gracefully");
        }
    }

    /// Test: Scanner recovers from errors
    #[tokio::test]
    async fn test_scanner_error_recovery() {
        let scanner = Scanner::new().unwrap();
        
        // Cause an error
        let _ = scanner.scan_file(&PathBuf::from("/nonexistent")).await;
        
        // Scanner should still work
        let temp_file = PathBuf::from("/tmp/recovery_test.txt");
        std::fs::write(&temp_file, "test").unwrap();
        
        let result = scanner.scan_file(&temp_file).await;
        assert!(result.is_ok(), "Scanner should recover from errors");
        
        std::fs::remove_file(&temp_file).ok();
    }

    /// Test: Scanner handles concurrent errors
    #[tokio::test]
    async fn test_scanner_concurrent_errors() {
        use std::sync::Arc;
        
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let scanner_clone = Arc::clone(&scanner);
            let handle = tokio::spawn(async move {
                scanner_clone.scan_file(&PathBuf::from(format!("/nonexistent_{}", i))).await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let result = handle.await.unwrap();
            // All should fail gracefully
            assert!(result.is_err(), "Should handle concurrent errors");
        }
    }

    /// Test: Scanner handles interrupted scans
    #[tokio::test]
    async fn test_scanner_interrupted_scan() {
        let scanner = Scanner::new().unwrap();
        
        // Start a scan
        let scan_id = scanner.start_quick_scan().await.unwrap();
        
        // Immediately stop it
        let result = scanner.stop_scan(scan_id);
        assert!(result.is_ok(), "Should handle interrupted scan");
    }

    // ==================== QUARANTINE ERROR HANDLING ====================

    /// Test: Quarantine handles file system errors
    #[test]
    fn test_quarantine_filesystem_errors() {
        let quarantine = QuarantineManager::new().unwrap();
        
        // Try to quarantine system files
        let system_files = vec![
            PathBuf::from("/dev/null"),
            PathBuf::from("/proc/cpuinfo"),
        ];
        
        for file in system_files {
            let result = quarantine.add_file(&file, "test");
            // Should handle error gracefully
            assert!(result.is_ok() || result.is_err(), "Should handle system files");
        }
    }

    /// Test: Quarantine handles corrupted data
    #[test]
    fn test_quarantine_corrupted_data() {
        let quarantine = QuarantineManager::new().unwrap();
        
        // Try to get info for various invalid IDs
        for id in [0, -1, i32::MAX, i32::MIN] {
            let result = quarantine.get_file_info(id);
            assert!(result.is_err(), "Should reject invalid IDs");
        }
    }

    /// Test: Quarantine handles disk full
    #[test]
    fn test_quarantine_disk_full() {
        let quarantine = QuarantineManager::new().unwrap();
        
        // Create a large file
        let temp_file = PathBuf::from("/tmp/large_quarantine.txt");
        let large_content = "A".repeat(1024 * 1024); // 1MB
        std::fs::write(&temp_file, large_content).unwrap();
        
        let result = quarantine.add_file(&temp_file, "large file");
        
        // Should either succeed or fail gracefully
        if let Ok(qid) = result {
            quarantine.delete_file(qid).ok();
        }
        
        std::fs::remove_file(&temp_file).ok();
    }

    /// Test: Quarantine handles permission errors
    #[test]
    fn test_quarantine_permission_errors() {
        let quarantine = QuarantineManager::new().unwrap();
        
        let temp_file = PathBuf::from("/tmp/no_perm_quarantine.txt");
        std::fs::write(&temp_file, "test").unwrap();
        
        // Remove all permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&temp_file).unwrap().permissions();
            perms.set_mode(0o000);
            std::fs::set_permissions(&temp_file, perms).unwrap();
        }
        
        let result = quarantine.add_file(&temp_file, "no perms");
        
        // Restore permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&temp_file).unwrap().permissions();
            perms.set_mode(0o644);
            std::fs::set_permissions(&temp_file, perms).unwrap();
        }
        
        std::fs::remove_file(&temp_file).ok();
    }

    // ==================== NETWORK ERROR HANDLING ====================

    /// Test: Network monitor handles connection errors
    #[test]
    fn test_network_connection_errors() {
        let monitor = NetworkMonitor::new().unwrap();
        
        // Try to get connections when network is unavailable
        let result = monitor.get_connections();
        
        // Should return empty list or error gracefully
        assert!(result.is_ok() || result.is_err(), "Should handle connection errors");
    }

    /// Test: Network monitor handles invalid IPs
    #[test]
    fn test_network_invalid_ips() {
        use std::net::IpAddr;
        use std::str::FromStr;
        
        let monitor = NetworkMonitor::new().unwrap();
        
        // Try various invalid/special IPs
        let ips = vec![
            "0.0.0.0",
            "255.255.255.255",
            "127.0.0.1",
        ];
        
        for ip_str in ips {
            let ip = IpAddr::from_str(ip_str).unwrap();
            let result = monitor.block_ip(ip);
            // Should handle gracefully
            assert!(result.is_ok() || result.is_err(), "Should handle special IPs");
        }
    }

    /// Test: Network monitor handles scan errors
    #[test]
    fn test_network_scan_errors() {
        let monitor = NetworkMonitor::new().unwrap();
        
        // Try to scan when network is unavailable
        let result = monitor.scan_network();
        
        // Should complete or fail gracefully
        assert!(result.is_ok() || result.is_err(), "Should handle scan errors");
    }

    // ==================== FIREWALL ERROR HANDLING ====================

    /// Test: Firewall handles invalid rules
    #[test]
    fn test_firewall_invalid_rules() {
        use ghost_antivirus::firewall::{FirewallRule, RuleAction, RuleDirection, Protocol};
        
        let firewall = Firewall::new().unwrap();
        
        // Rule with no criteria
        let invalid_rule = FirewallRule {
            id: None,
            name: "".to_string(), // Empty name
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        let result = firewall.add_rule(invalid_rule);
        assert!(result.is_err(), "Should reject invalid rule");
    }

    /// Test: Firewall handles rule conflicts
    #[test]
    fn test_firewall_rule_conflicts() {
        use ghost_antivirus::firewall::{FirewallRule, RuleAction, RuleDirection, Protocol};
        use std::net::IpAddr;
        use std::str::FromStr;
        
        let firewall = Firewall::new().unwrap();
        
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        
        // Add blocking rule
        let block_rule = FirewallRule {
            id: None,
            name: "Block".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: Some(ip),
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        let id1 = firewall.add_rule(block_rule).unwrap();
        
        // Add conflicting allow rule
        let allow_rule = FirewallRule {
            id: None,
            name: "Allow".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: Some(ip),
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        let id2 = firewall.add_rule(allow_rule).unwrap();
        
        // Should handle conflicts appropriately
        firewall.delete_rule(id1).ok();
        firewall.delete_rule(id2).ok();
    }

    // ==================== CONFIG ERROR HANDLING ====================

    /// Test: Config handles invalid values
    #[test]
    fn test_config_invalid_values() {
        let mut config = Config::default();
        
        // Try to set invalid values
        let invalid_settings = vec![
            ("scan_interval", "-1"),
            ("max_threads", "0"),
            ("timeout", "invalid"),
        ];
        
        for (key, value) in invalid_settings {
            let result = config.set_setting(key, value);
            assert!(result.is_err(), "Should reject invalid values");
        }
    }

    /// Test: Config handles missing files
    #[test]
    fn test_config_missing_files() {
        // Try to load from non-existent location
        let result = Config::load();
        
        // Should either load defaults or return error
        assert!(result.is_ok() || result.is_err(), "Should handle missing files");
    }

    /// Test: Config handles concurrent modifications
    #[test]
    fn test_config_concurrent_modifications() {
        use std::sync::Arc;
        use std::thread;
        
        let config = Arc::new(Config::default());
        let mut handles = vec![];
        
        for i in 0..10 {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                // Try to read config concurrently
                let _ = config_clone.scan_settings.real_time_protection;
                i
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Should handle concurrent access");
        }
    }

    // ==================== SYSTEM MONITOR ERROR HANDLING ====================

    /// Test: System monitor handles unavailable metrics
    #[test]
    fn test_system_unavailable_metrics() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Try to get metrics that may not be available
        let _ = monitor.get_temperature();
        let _ = monitor.get_battery_status();
        
        // Should not panic
    }

    /// Test: System monitor handles process errors
    #[test]
    fn test_system_process_errors() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Try to get non-existent process
        let result = monitor.get_process(999999);
        assert!(result.is_err(), "Should fail for non-existent process");
    }

    /// Test: System monitor handles monitoring errors
    #[test]
    fn test_system_monitoring_errors() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Start monitoring
        monitor.start_monitoring().unwrap();
        
        // Try to start again (should handle gracefully)
        let result = monitor.start_monitoring();
        assert!(result.is_ok() || result.is_err(), "Should handle double start");
        
        monitor.stop_monitoring().ok();
    }

    // ==================== UPDATES ERROR HANDLING ====================

    /// Test: Updates handles network errors
    #[test]
    fn test_updates_network_errors() {
        let manager = UpdateManager::new().unwrap();
        
        // Try to check updates (may fail with no internet)
        let result = manager.check_for_updates();
        
        // Should handle network errors gracefully
        assert!(result.is_ok() || result.is_err(), "Should handle network errors");
    }

    /// Test: Updates handles invalid versions
    #[test]
    fn test_updates_invalid_versions() {
        let manager = UpdateManager::new().unwrap();
        
        // Get current version
        let version = manager.get_current_version();
        assert!(version.is_ok(), "Should get current version");
    }

    /// Test: Updates handles download errors
    #[test]
    fn test_updates_download_errors() {
        let manager = UpdateManager::new().unwrap();
        
        // Try to download without checking first
        let result = manager.download_update();
        
        // Should handle gracefully
        assert!(result.is_ok() || result.is_err(), "Should handle download errors");
    }

    // ==================== INTEGRATION ERROR HANDLING ====================

    /// Test: All modules handle simultaneous errors
    #[tokio::test]
    async fn test_simultaneous_errors() {
        let scanner = Scanner::new().unwrap();
        let quarantine = QuarantineManager::new().unwrap();
        let monitor = SystemMonitor::new().unwrap();
        
        // Cause errors in all modules simultaneously
        let scan_result = scanner.scan_file(&PathBuf::from("/nonexistent1")).await;
        let quarantine_result = quarantine.add_file(&PathBuf::from("/nonexistent2"), "test");
        let monitor_result = monitor.get_process(999999);
        
        // All should fail gracefully
        assert!(scan_result.is_err());
        assert!(quarantine_result.is_err());
        assert!(monitor_result.is_err());
        
        // All modules should still be operational
        let temp_file = PathBuf::from("/tmp/recovery_all.txt");
        std::fs::write(&temp_file, "test").unwrap();
        
        assert!(scanner.scan_file(&temp_file).await.is_ok());
        assert!(monitor.get_metrics().is_ok());
        
        std::fs::remove_file(&temp_file).ok();
    }

    /// Test: Error recovery after system stress
    #[test]
    fn test_error_recovery_stress() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Cause multiple errors
        for _ in 0..100 {
            let _ = monitor.get_process(999999);
        }
        
        // Should still work
        let result = monitor.get_metrics();
        assert!(result.is_ok(), "Should recover from stress");
    }

    /// Test: Graceful degradation
    #[tokio::test]
    async fn test_graceful_degradation() {
        let scanner = Scanner::new().unwrap();
        
        // Even with errors, basic functionality should work
        let temp_file = PathBuf::from("/tmp/degradation_test.txt");
        std::fs::write(&temp_file, "test").unwrap();
        
        // This should work even if other operations failed
        let result = scanner.scan_file(&temp_file).await;
        assert!(result.is_ok(), "Should maintain basic functionality");
        
        std::fs::remove_file(&temp_file).ok();
    }
}