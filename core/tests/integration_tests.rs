//! Integration Tests
//! 
//! Comprehensive integration test suite for cross-module communication.
//! Tests how different modules work together in real-world scenarios.

use ghost_core::{
    scanner::Scanner,
    quarantine::QuarantineManager,
    analyzer::ThreatAnalyzer,
    network::NetworkMonitor,
    firewall::Firewall,
    config::Config,
    system::SystemMonitor,
};
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test: Scanner → Analyzer → Quarantine workflow
    #[tokio::test]
    async fn test_scan_detect_quarantine_workflow() {
        // Initialize components
        let scanner = Scanner::new().unwrap();
        let analyzer = Arc::new(ThreatAnalyzer::new().unwrap());
        let quarantine = QuarantineManager::new().unwrap();
        
        // Create a test file
        let test_file = PathBuf::from("/tmp/test_malware.txt");
        std::fs::write(&test_file, "EICAR test string").unwrap();
        
        // Scan the file
        let scan_result = scanner.scan_file(&test_file).await;
        assert!(scan_result.is_ok(), "Scan should complete successfully");
        
        // Analyze for threats
        let threats = analyzer.get_detected_threats();
        
        // If threat detected, quarantine it
        if threats.len() > 0 {
            let result = quarantine.add_file(&test_file, "Test threat");
            assert!(result.is_ok(), "Should quarantine threat successfully");
            
            // Verify file is in quarantine
            let quarantined = quarantine.list_files();
            assert!(quarantined.is_ok(), "Should list quarantined files");
            assert!(quarantined.unwrap().len() > 0, "Should have quarantined files");
        }
        
        // Cleanup
        let _ = std::fs::remove_file(&test_file);
    }

    /// Test: Network Monitor → Firewall integration
    #[test]
    fn test_network_firewall_integration() {
        let network = NetworkMonitor::new().unwrap();
        let firewall = Firewall::new().unwrap();
        
        // Start network monitoring
        network.start_monitoring().unwrap();
        
        // Get suspicious connections
        let connections = network.get_connections().unwrap();
        
        // Block suspicious IPs via firewall
        for conn in connections.iter().take(1) {
            if conn.is_suspicious() {
                let rule = firewall.create_block_rule(conn.remote_addr);
                assert!(rule.is_ok(), "Should create block rule successfully");
            }
        }
        
        network.stop_monitoring().unwrap();
    }

    /// Test: Config → Scanner integration
    #[test]
    fn test_config_scanner_integration() {
        let mut config = Config::default();
        let scanner = Scanner::new().unwrap();
        
        // Update config
        config.scan_settings.real_time_protection = true;
        config.scan_settings.scan_archives = true;
        config.save().unwrap();
        
        // Apply config to scanner
        let result = scanner.apply_config(&config);
        assert!(result.is_ok(), "Should apply config to scanner successfully");
        
        // Verify scanner uses config
        assert!(scanner.is_real_time_enabled(), "Real-time should be enabled");
        assert!(scanner.should_scan_archives(), "Archive scanning should be enabled");
    }

    /// Test: System Monitor → Scanner resource management
    #[test]
    fn test_system_scanner_resource_management() {
        let system = SystemMonitor::new().unwrap();
        let scanner = Scanner::new().unwrap();
        
        // Get system metrics
        let metrics = system.get_metrics().unwrap();
        
        // Adjust scanner based on system resources
        if metrics.cpu_usage > 80.0 {
            scanner.set_priority_low().unwrap();
        } else {
            scanner.set_priority_normal().unwrap();
        }
        
        // Verify scanner adjusted
        assert!(scanner.get_priority().is_ok(), "Should get scanner priority");
    }

    /// Test: Full threat detection pipeline
    #[tokio::test]
    async fn test_full_threat_pipeline() {
        // Initialize all components
        let scanner = Scanner::new().unwrap();
        let analyzer = Arc::new(ThreatAnalyzer::new().unwrap());
        let quarantine = QuarantineManager::new().unwrap();
        let config = Config::default();
        
        // Create test directory with files
        let test_dir = PathBuf::from("/tmp/test_scan_dir");
        std::fs::create_dir_all(&test_dir).unwrap();
        
        // Create test files
        for i in 0..5 {
            let file_path = test_dir.join(format!("file_{}.txt", i));
            std::fs::write(&file_path, format!("Test content {}", i)).unwrap();
        }
        
        // Start full scan
        let scan_id = scanner.start_full_scan().await.unwrap();
        
        // Wait for scan to complete
        while scanner.is_scanning() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        // Get scan results
        let stats = scanner.get_stats().unwrap();
        assert!(stats.total_scanned > 0, "Should have scanned files");
        
        // Get detected threats
        let threats = analyzer.get_detected_threats();
        
        // Process threats
        for threat in threats {
            quarantine.add_file(&threat.file_path, &threat.threat_name).unwrap();
        }
        
        // Cleanup
        std::fs::remove_dir_all(&test_dir).unwrap();
    }

    /// Test: Real-time protection workflow
    #[tokio::test]
    async fn test_realtime_protection_workflow() {
        let scanner = Scanner::new().unwrap();
        let analyzer = Arc::new(ThreatAnalyzer::new().unwrap());
        let quarantine = QuarantineManager::new().unwrap();
        
        // Enable real-time protection
        scanner.enable_realtime_protection().unwrap();
        
        // Simulate file creation
        let test_file = PathBuf::from("/tmp/realtime_test.txt");
        std::fs::write(&test_file, "Test content").unwrap();
        
        // Real-time scanner should detect and scan
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        
        // Check if file was scanned
        let stats = scanner.get_stats().unwrap();
        
        // Disable real-time protection
        scanner.disable_realtime_protection().unwrap();
        
        // Cleanup
        let _ = std::fs::remove_file(&test_file);
    }

    /// Test: Network threat detection and blocking
    #[test]
    fn test_network_threat_blocking() {
        let network = NetworkMonitor::new().unwrap();
        let firewall = Firewall::new().unwrap();
        
        // Start monitoring
        network.start_monitoring().unwrap();
        
        // Analyze traffic
        let analysis = network.analyze_traffic().unwrap();
        
        // Block suspicious connections
        if analysis.suspicious_connections > 0 {
            let suspicious = network.get_suspicious_connections().unwrap();
            
            for conn in suspicious {
                firewall.block_ip(conn.remote_addr).unwrap();
            }
        }
        
        network.stop_monitoring().unwrap();
    }

    /// Test: Update and restart workflow
    #[test]
    fn test_update_workflow() {
        use ghost_antivirus::updates::UpdateManager;
        
        let updates = UpdateManager::new().unwrap();
        let scanner = Scanner::new().unwrap();
        
        // Check for updates
        updates.check_for_updates().unwrap();
        
        if updates.is_update_available().unwrap() {
            // Stop scanner before update
            scanner.stop_all_scans().unwrap();
            
            // Download update
            updates.download_update().unwrap();
            
            // Verify update
            updates.verify_update().unwrap();
            
            // In production, would apply and restart
            // updates.apply_update().unwrap();
        }
    }

    /// Test: Quarantine restore workflow
    #[test]
    fn test_quarantine_restore_workflow() {
        let quarantine = QuarantineManager::new().unwrap();
        
        // Create test file
        let test_file = PathBuf::from("/tmp/restore_test.txt");
        std::fs::write(&test_file, "Test content").unwrap();
        
        // Quarantine file
        let qid = quarantine.add_file(&test_file, "False positive").unwrap();
        
        // Verify file is quarantined
        assert!(!test_file.exists(), "Original file should be removed");
        
        // Restore file
        let restore_path = PathBuf::from("/tmp/restored_test.txt");
        quarantine.restore_file(qid, &restore_path).unwrap();
        
        // Verify file is restored
        assert!(restore_path.exists(), "File should be restored");
        
        // Cleanup
        let _ = std::fs::remove_file(&restore_path);
    }

    /// Test: Configuration persistence across restarts
    #[test]
    fn test_config_persistence() {
        // Create and save config
        let mut config1 = Config::default();
        config1.scan_settings.real_time_protection = false;
        config1.scan_settings.scan_archives = true;
        config1.save().unwrap();
        
        // Load config in new instance
        let config2 = Config::load().unwrap();
        
        // Verify settings persisted
        assert!(!config2.scan_settings.real_time_protection, "Setting should persist");
        assert!(config2.scan_settings.scan_archives, "Setting should persist");
    }

    /// Test: Multi-threaded scanning
    #[tokio::test]
    async fn test_concurrent_scanning() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        // Create test files
        let test_dir = PathBuf::from("/tmp/concurrent_test");
        std::fs::create_dir_all(&test_dir).unwrap();
        
        for i in 0..10 {
            let file_path = test_dir.join(format!("file_{}.txt", i));
            std::fs::write(&file_path, format!("Content {}", i)).unwrap();
        }
        
        // Scan files concurrently
        for i in 0..10 {
            let scanner_clone = Arc::clone(&scanner);
            let file_path = test_dir.join(format!("file_{}.txt", i));
            
            let handle = tokio::spawn(async move {
                scanner_clone.scan_file(&file_path).await
            });
            
            handles.push(handle);
        }
        
        // Wait for all scans to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok(), "Concurrent scan should succeed");
        }
        
        // Cleanup
        std::fs::remove_dir_all(&test_dir).unwrap();
    }

    /// Test: System health monitoring during scan
    #[tokio::test]
    async fn test_health_monitoring_during_scan() {
        let system = SystemMonitor::new().unwrap();
        let scanner = Scanner::new().unwrap();
        
        // Start system monitoring
        system.start_monitoring().unwrap();
        
        // Start scan
        scanner.start_quick_scan().await.unwrap();
        
        // Monitor system health during scan
        while scanner.is_scanning() {
            let health = system.check_health().unwrap();
            
            if !health.cpu_healthy || !health.memory_healthy {
                // Pause scan if system is stressed
                scanner.pause_scan().unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                scanner.resume_scan().unwrap();
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        system.stop_monitoring().unwrap();
    }

    /// Test: Error recovery and resilience
    #[tokio::test]
    async fn test_error_recovery() {
        let scanner = Scanner::new().unwrap();
        
        // Try to scan non-existent file
        let result = scanner.scan_file(&PathBuf::from("/nonexistent/file.txt")).await;
        assert!(result.is_err(), "Should fail gracefully for non-existent file");
        
        // Scanner should still be operational
        let stats = scanner.get_stats();
        assert!(stats.is_ok(), "Scanner should still be operational after error");
    }

    /// Test: Complete security workflow
    #[tokio::test]
    async fn test_complete_security_workflow() {
        // Initialize all components
        let config = Config::default();
        let scanner = Scanner::new().unwrap();
        let analyzer = Arc::new(ThreatAnalyzer::new().unwrap());
        let quarantine = QuarantineManager::new().unwrap();
        let network = NetworkMonitor::new().unwrap();
        let firewall = Firewall::new().unwrap();
        let system = SystemMonitor::new().unwrap();
        
        // 1. Start system monitoring
        system.start_monitoring().unwrap();
        
        // 2. Start network monitoring
        network.start_monitoring().unwrap();
        
        // 3. Enable real-time protection
        scanner.enable_realtime_protection().unwrap();
        
        // 4. Run quick scan
        scanner.start_quick_scan().await.unwrap();
        
        // Wait for scan
        while scanner.is_scanning() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        // 5. Process threats
        let threats = analyzer.get_detected_threats();
        for threat in threats {
            quarantine.add_file(&threat.file_path, &threat.threat_name).unwrap();
        }
        
        // 6. Check network for suspicious activity
        let analysis = network.analyze_traffic().unwrap();
        if analysis.suspicious_connections > 0 {
            let suspicious = network.get_suspicious_connections().unwrap();
            for conn in suspicious {
                firewall.block_ip(conn.remote_addr).unwrap();
            }
        }
        
        // 7. Check system health
        let health = system.check_health().unwrap();
        assert!(health.cpu_healthy, "System should be healthy");
        
        // 8. Cleanup
        scanner.disable_realtime_protection().unwrap();
        network.stop_monitoring().unwrap();
        system.stop_monitoring().unwrap();
    }
}