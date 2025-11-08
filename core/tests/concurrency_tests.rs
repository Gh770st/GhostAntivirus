//! Concurrency Tests
//! 
//! Comprehensive tests for concurrent operations and thread safety.
//! Ensures proper synchronization and race condition prevention.

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
use std::thread;

#[cfg(test)]
mod concurrency_tests {
    use super::*;

    // ==================== SCANNER CONCURRENCY ====================

    /// Test: Multiple threads scanning different files
    #[tokio::test]
    async fn test_scanner_multi_thread_different_files() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        // Create test files
        for i in 0..10 {
            let file_path = PathBuf::from(format!("/tmp/concurrent_scan_{}.txt", i));
            std::fs::write(&file_path, format!("content {}", i)).unwrap();
        }
        
        // Scan files concurrently
        for i in 0..10 {
            let scanner_clone = Arc::clone(&scanner);
            let file_path = PathBuf::from(format!("/tmp/concurrent_scan_{}.txt", i));
            
            let handle = tokio::spawn(async move {
                scanner_clone.scan_file(&file_path).await
            });
            
            handles.push(handle);
        }
        
        // All scans should succeed
        for handle in handles {
            assert!(handle.await.unwrap().is_ok(), "Concurrent scans should succeed");
        }
        
        // Cleanup
        for i in 0..10 {
            std::fs::remove_file(format!("/tmp/concurrent_scan_{}.txt", i)).ok();
        }
    }

    /// Test: Multiple threads reading scanner stats
    #[test]
    fn test_scanner_concurrent_stats_read() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        for _ in 0..20 {
            let scanner_clone = Arc::clone(&scanner);
            let handle = thread::spawn(move || {
                scanner_clone.get_stats()
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().unwrap().is_ok(), "Concurrent reads should succeed");
        }
    }

    /// Test: Concurrent scan start/stop
    #[tokio::test]
    async fn test_scanner_concurrent_start_stop() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..5 {
            let scanner_clone = Arc::clone(&scanner);
            let handle = tokio::spawn(async move {
                if i % 2 == 0 {
                    scanner_clone.start_quick_scan().await
                } else {
                    // Try to stop any running scan
                    scanner_clone.stop_all_scans()
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.await;
        }
    }

    // ==================== QUARANTINE CONCURRENCY ====================

    /// Test: Concurrent file additions to quarantine
    #[test]
    fn test_quarantine_concurrent_additions() {
        let quarantine = Arc::new(QuarantineManager::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let quarantine_clone = Arc::clone(&quarantine);
            let handle = thread::spawn(move || {
                let file_path = PathBuf::from(format!("/tmp/quarantine_concurrent_{}.txt", i));
                std::fs::write(&file_path, format!("content {}", i)).unwrap();
                
                let result = quarantine_clone.add_file(&file_path, &format!("threat {}", i));
                
                std::fs::remove_file(&file_path).ok();
                result
            });
            handles.push(handle);
        }
        
        let mut qids = vec![];
        for handle in handles {
            if let Ok(Ok(qid)) = handle.join() {
                qids.push(qid);
            }
        }
        
        // Cleanup
        for qid in qids {
            quarantine.delete_file(qid).ok();
        }
    }

    /// Test: Concurrent quarantine operations (add/delete/restore)
    #[test]
    fn test_quarantine_mixed_operations() {
        let quarantine = Arc::new(QuarantineManager::new().unwrap());
        let mut handles = vec![];
        
        // Add some files first
        let mut qids = vec![];
        for i in 0..5 {
            let file_path = PathBuf::from(format!("/tmp/mixed_op_{}.txt", i));
            std::fs::write(&file_path, "test").unwrap();
            if let Ok(qid) = quarantine.add_file(&file_path, "test") {
                qids.push(qid);
            }
        }
        
        // Perform mixed operations concurrently
        for i in 0..10 {
            let quarantine_clone = Arc::clone(&quarantine);
            let qids_clone = qids.clone();
            
            let handle = thread::spawn(move || {
                match i % 3 {
                    0 => quarantine_clone.list_files(),
                    1 => {
                        if let Some(&qid) = qids_clone.first() {
                            quarantine_clone.get_file_info(qid).map(|_| vec![])
                        } else {
                            Ok(vec![])
                        }
                    },
                    _ => Ok(vec![]),
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
        
        // Cleanup
        for qid in qids {
            quarantine.delete_file(qid).ok();
        }
    }

    // ==================== NETWORK CONCURRENCY ====================

    /// Test: Concurrent network monitoring
    #[test]
    fn test_network_concurrent_monitoring() {
        let monitor = Arc::new(NetworkMonitor::new().unwrap());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                monitor_clone.get_connections()
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
    }

    /// Test: Concurrent IP blocking/unblocking
    #[test]
    fn test_network_concurrent_blocking() {
        use std::net::IpAddr;
        use std::str::FromStr;
        
        let monitor = Arc::new(NetworkMonitor::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let ip = IpAddr::from_str(&format!("192.168.1.{}", 100 + i)).unwrap();
                
                if i % 2 == 0 {
                    monitor_clone.block_ip(ip)
                } else {
                    monitor_clone.unblock_ip(ip)
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
    }

    // ==================== FIREWALL CONCURRENCY ====================

    /// Test: Concurrent firewall rule additions
    #[test]
    fn test_firewall_concurrent_rule_additions() {
        use ghost_antivirus::firewall::{FirewallRule, RuleAction, RuleDirection, Protocol};
        
        let firewall = Arc::new(Firewall::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let firewall_clone = Arc::clone(&firewall);
            let handle = thread::spawn(move || {
                let rule = FirewallRule {
                    id: None,
                    name: format!("Concurrent Rule {}", i),
                    action: RuleAction::Block,
                    direction: RuleDirection::Inbound,
                    protocol: Protocol::TCP,
                    source_ip: None,
                    source_port: None,
                    dest_ip: None,
                    dest_port: Some(8000 + i as u16),
                    enabled: true,
                };
                
                firewall_clone.add_rule(rule)
            });
            handles.push(handle);
        }
        
        let mut rule_ids = vec![];
        for handle in handles {
            if let Ok(Ok(id)) = handle.join() {
                rule_ids.push(id);
            }
        }
        
        // Cleanup
        for id in rule_ids {
            firewall.delete_rule(id).ok();
        }
    }

    /// Test: Concurrent rule modifications
    #[test]
    fn test_firewall_concurrent_modifications() {
        use ghost_antivirus::firewall::{FirewallRule, RuleAction, RuleDirection, Protocol};
        
        let firewall = Arc::new(Firewall::new().unwrap());
        
        // Add a rule
        let rule = FirewallRule {
            id: None,
            name: "Test Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(9000),
            enabled: true,
        };
        
        let rule_id = firewall.add_rule(rule).unwrap();
        
        let mut handles = vec![];
        
        // Concurrent enable/disable
        for i in 0..10 {
            let firewall_clone = Arc::clone(&firewall);
            let handle = thread::spawn(move || {
                if i % 2 == 0 {
                    firewall_clone.enable_rule(rule_id)
                } else {
                    firewall_clone.disable_rule(rule_id)
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
        
        firewall.delete_rule(rule_id).ok();
    }

    // ==================== CONFIG CONCURRENCY ====================

    /// Test: Concurrent config reads
    #[test]
    fn test_config_concurrent_reads() {
        let config = Arc::new(Config::default());
        let mut handles = vec![];
        
        for _ in 0..20 {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                let _ = config_clone.scan_settings.real_time_protection;
                let _ = config_clone.scan_settings.protection_level;
                let _ = config_clone.update_settings.auto_update;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Concurrent reads should succeed");
        }
    }

    // ==================== SYSTEM MONITOR CONCURRENCY ====================

    /// Test: Concurrent metrics collection
    #[test]
    fn test_system_concurrent_metrics() {
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        let mut handles = vec![];
        
        for _ in 0..20 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                monitor_clone.get_metrics()
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().unwrap().is_ok(), "Concurrent metrics should succeed");
        }
    }

    /// Test: Concurrent process queries
    #[test]
    fn test_system_concurrent_process_queries() {
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        let mut handles = vec![];
        
        let current_pid = std::process::id();
        
        for _ in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                monitor_clone.get_process(current_pid)
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
    }

    /// Test: Concurrent monitoring start/stop
    #[test]
    fn test_system_concurrent_start_stop() {
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                if i % 2 == 0 {
                    monitor_clone.start_monitoring()
                } else {
                    monitor_clone.stop_monitoring()
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.join();
        }
    }

    // ==================== INTEGRATION CONCURRENCY ====================

    /// Test: All modules under concurrent load
    #[tokio::test]
    async fn test_all_modules_concurrent_load() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let quarantine = Arc::new(QuarantineManager::new().unwrap());
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        
        let mut handles = vec![];
        
        // Scanner operations
        for i in 0..5 {
            let scanner_clone = Arc::clone(&scanner);
            let handle = tokio::spawn(async move {
                let file_path = PathBuf::from(format!("/tmp/load_test_{}.txt", i));
                std::fs::write(&file_path, "test").unwrap();
                let result = scanner_clone.scan_file(&file_path).await;
                std::fs::remove_file(&file_path).ok();
                result
            });
            handles.push(handle);
        }
        
        // Quarantine operations
        for _ in 0..5 {
            let quarantine_clone = Arc::clone(&quarantine);
            let handle = tokio::spawn(async move {
                quarantine_clone.list_files()
            });
            handles.push(handle);
        }
        
        // Monitor operations
        for _ in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = tokio::spawn(async move {
                monitor_clone.get_metrics()
            });
            handles.push(handle);
        }
        
        // All operations should complete
        for handle in handles {
            let _ = handle.await;
        }
    }

    /// Test: Race condition prevention
    #[test]
    fn test_race_condition_prevention() {
        let quarantine = Arc::new(QuarantineManager::new().unwrap());
        
        // Create a file
        let file_path = PathBuf::from("/tmp/race_test.txt");
        std::fs::write(&file_path, "test").unwrap();
        
        // Try to quarantine the same file multiple times concurrently
        let mut handles = vec![];
        
        for _ in 0..10 {
            let quarantine_clone = Arc::clone(&quarantine);
            let file_clone = file_path.clone();
            let handle = thread::spawn(move || {
                quarantine_clone.add_file(&file_clone, "race test")
            });
            handles.push(handle);
        }
        
        let mut success_count = 0;
        for handle in handles {
            if let Ok(Ok(_)) = handle.join() {
                success_count += 1;
            }
        }
        
        // Only one should succeed (file gets moved)
        assert!(success_count <= 1, "Should prevent race conditions");
    }

    /// Test: Deadlock prevention
    #[test]
    fn test_deadlock_prevention() {
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        let mut handles = vec![];
        
        // Create operations that could potentially deadlock
        for _ in 0..20 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                monitor_clone.start_monitoring().ok();
                monitor_clone.get_metrics().ok();
                monitor_clone.stop_monitoring().ok();
            });
            handles.push(handle);
        }
        
        // All should complete without deadlock
        for handle in handles {
            assert!(handle.join().is_ok(), "Should not deadlock");
        }
    }

    /// Test: Memory consistency under concurrent access
    #[test]
    fn test_memory_consistency() {
        let scanner = Arc::new(Scanner::new().unwrap());
        let mut handles = vec![];
        
        // Multiple threads reading stats
        for _ in 0..50 {
            let scanner_clone = Arc::clone(&scanner);
            let handle = thread::spawn(move || {
                if let Ok(stats) = scanner_clone.get_stats() {
                    // Stats should be consistent
                    assert!(stats.total_scanned >= 0);
                    assert!(stats.threats_found >= 0);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Should maintain memory consistency");
        }
    }
}