//! System Module Tests
//! 
//! Comprehensive test suite for the system monitoring module.
//! Tests metrics collection, resource monitoring, and system information.

use ghost_core::system::{SystemMonitor, SystemMetrics, ResourceUsage};

#[cfg(test)]
mod system_tests {
    use super::*;

    /// Test: System monitor initialization
    #[test]
    fn test_system_monitor_new() {
        let monitor = SystemMonitor::new();
        assert!(monitor.is_ok(), "System monitor should initialize successfully");
    }

    /// Test: Get system metrics
    #[test]
    fn test_get_system_metrics() {
        let monitor = SystemMonitor::new().unwrap();
        let metrics = monitor.get_metrics();
        
        assert!(metrics.is_ok(), "Should retrieve system metrics successfully");
        
        let metrics = metrics.unwrap();
        assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0, "CPU usage should be valid percentage");
        assert!(metrics.memory_used > 0, "Memory used should be positive");
        assert!(metrics.memory_total > 0, "Total memory should be positive");
    }

    /// Test: Get CPU usage
    #[test]
    fn test_get_cpu_usage() {
        let monitor = SystemMonitor::new().unwrap();
        let cpu_usage = monitor.get_cpu_usage();
        
        assert!(cpu_usage.is_ok(), "Should retrieve CPU usage successfully");
        
        let usage = cpu_usage.unwrap();
        assert!(usage >= 0.0 && usage <= 100.0, "CPU usage should be between 0 and 100");
    }

    /// Test: Get memory usage
    #[test]
    fn test_get_memory_usage() {
        let monitor = SystemMonitor::new().unwrap();
        let memory = monitor.get_memory_usage();
        
        assert!(memory.is_ok(), "Should retrieve memory usage successfully");
        
        let (used, total) = memory.unwrap();
        assert!(used > 0, "Used memory should be positive");
        assert!(total > 0, "Total memory should be positive");
        assert!(used <= total, "Used memory should not exceed total");
    }

    /// Test: Get disk usage
    #[test]
    fn test_get_disk_usage() {
        let monitor = SystemMonitor::new().unwrap();
        let disk = monitor.get_disk_usage();
        
        assert!(disk.is_ok(), "Should retrieve disk usage successfully");
        
        let (used, total) = disk.unwrap();
        assert!(used >= 0, "Used disk space should be non-negative");
        assert!(total > 0, "Total disk space should be positive");
        assert!(used <= total, "Used disk should not exceed total");
    }

    /// Test: Get system uptime
    #[test]
    fn test_get_uptime() {
        let monitor = SystemMonitor::new().unwrap();
        let uptime = monitor.get_uptime();
        
        assert!(uptime.is_ok(), "Should retrieve uptime successfully");
        assert!(uptime.unwrap() > 0, "Uptime should be positive");
    }

    /// Test: Get process count
    #[test]
    fn test_get_process_count() {
        let monitor = SystemMonitor::new().unwrap();
        let count = monitor.get_process_count();
        
        assert!(count.is_ok(), "Should retrieve process count successfully");
        assert!(count.unwrap() > 0, "Should have at least one process");
    }

    /// Test: Get system information
    #[test]
    fn test_get_system_info() {
        let monitor = SystemMonitor::new().unwrap();
        let info = monitor.get_system_info();
        
        assert!(info.is_ok(), "Should retrieve system info successfully");
        
        let info = info.unwrap();
        assert!(!info.os_name.is_empty(), "OS name should not be empty");
        assert!(!info.os_version.is_empty(), "OS version should not be empty");
        assert!(!info.hostname.is_empty(), "Hostname should not be empty");
    }

    /// Test: Get CPU info
    #[test]
    fn test_get_cpu_info() {
        let monitor = SystemMonitor::new().unwrap();
        let cpu_info = monitor.get_cpu_info();
        
        assert!(cpu_info.is_ok(), "Should retrieve CPU info successfully");
        
        let info = cpu_info.unwrap();
        assert!(info.core_count > 0, "Should have at least one CPU core");
        assert!(!info.brand.is_empty(), "CPU brand should not be empty");
    }

    /// Test: Get network interfaces
    #[test]
    fn test_get_network_interfaces() {
        let monitor = SystemMonitor::new().unwrap();
        let interfaces = monitor.get_network_interfaces();
        
        assert!(interfaces.is_ok(), "Should retrieve network interfaces successfully");
        assert!(interfaces.unwrap().len() > 0, "Should have at least one network interface");
    }

    /// Test: Get resource usage over time
    #[test]
    fn test_get_resource_usage_history() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Collect some samples
        monitor.start_monitoring().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        monitor.stop_monitoring().unwrap();
        
        let history = monitor.get_usage_history();
        assert!(history.is_ok(), "Should retrieve usage history successfully");
    }

    /// Test: Monitor start/stop
    #[test]
    fn test_monitor_lifecycle() {
        let monitor = SystemMonitor::new().unwrap();
        
        let start_result = monitor.start_monitoring();
        assert!(start_result.is_ok(), "Should start monitoring successfully");
        
        let stop_result = monitor.stop_monitoring();
        assert!(stop_result.is_ok(), "Should stop monitoring successfully");
    }

    /// Test: Get current processes
    #[test]
    fn test_get_processes() {
        let monitor = SystemMonitor::new().unwrap();
        let processes = monitor.get_processes();
        
        assert!(processes.is_ok(), "Should retrieve processes successfully");
        assert!(processes.unwrap().len() > 0, "Should have at least one process");
    }

    /// Test: Get process by PID
    #[test]
    fn test_get_process_by_pid() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Get current process
        let current_pid = std::process::id();
        let process = monitor.get_process(current_pid);
        
        assert!(process.is_ok(), "Should retrieve process by PID successfully");
    }

    /// Test: Get top processes by CPU
    #[test]
    fn test_get_top_cpu_processes() {
        let monitor = SystemMonitor::new().unwrap();
        let top_processes = monitor.get_top_cpu_processes(5);
        
        assert!(top_processes.is_ok(), "Should retrieve top CPU processes successfully");
        assert!(top_processes.unwrap().len() <= 5, "Should return at most 5 processes");
    }

    /// Test: Get top processes by memory
    #[test]
    fn test_get_top_memory_processes() {
        let monitor = SystemMonitor::new().unwrap();
        let top_processes = monitor.get_top_memory_processes(5);
        
        assert!(top_processes.is_ok(), "Should retrieve top memory processes successfully");
        assert!(top_processes.unwrap().len() <= 5, "Should return at most 5 processes");
    }

    /// Test: Check system health
    #[test]
    fn test_check_system_health() {
        let monitor = SystemMonitor::new().unwrap();
        let health = monitor.check_health();
        
        assert!(health.is_ok(), "Should check system health successfully");
        
        let health = health.unwrap();
        assert!(health.cpu_healthy, "CPU should be healthy");
        assert!(health.memory_healthy, "Memory should be healthy");
        assert!(health.disk_healthy, "Disk should be healthy");
    }

    /// Test: Get temperature (if available)
    #[test]
    fn test_get_temperature() {
        let monitor = SystemMonitor::new().unwrap();
        let temp = monitor.get_temperature();
        
        // Temperature may not be available on all systems
        if temp.is_ok() {
            let temp_value = temp.unwrap();
            assert!(temp_value > 0.0 && temp_value < 150.0, "Temperature should be reasonable");
        }
    }

    /// Test: Get battery status (if available)
    #[test]
    fn test_get_battery_status() {
        let monitor = SystemMonitor::new().unwrap();
        let battery = monitor.get_battery_status();
        
        // Battery may not be available on all systems (e.g., desktops)
        if battery.is_ok() {
            let status = battery.unwrap();
            assert!(status.percentage >= 0.0 && status.percentage <= 100.0, "Battery percentage should be valid");
        }
    }

    /// Test: Resource alerts
    #[test]
    fn test_resource_alerts() {
        let monitor = SystemMonitor::new().unwrap();
        
        // Set alert thresholds
        monitor.set_cpu_threshold(90.0).unwrap();
        monitor.set_memory_threshold(90.0).unwrap();
        
        let alerts = monitor.get_alerts();
        assert!(alerts.is_ok(), "Should retrieve alerts successfully");
    }

    /// Test: System load average
    #[test]
    fn test_get_load_average() {
        let monitor = SystemMonitor::new().unwrap();
        let load = monitor.get_load_average();
        
        assert!(load.is_ok(), "Should retrieve load average successfully");
        
        let (one, five, fifteen) = load.unwrap();
        assert!(one >= 0.0, "1-minute load should be non-negative");
        assert!(five >= 0.0, "5-minute load should be non-negative");
        assert!(fifteen >= 0.0, "15-minute load should be non-negative");
    }

    /// Test: Disk I/O statistics
    #[test]
    fn test_get_disk_io() {
        let monitor = SystemMonitor::new().unwrap();
        let io = monitor.get_disk_io();
        
        assert!(io.is_ok(), "Should retrieve disk I/O stats successfully");
        
        let (read, write) = io.unwrap();
        assert!(read >= 0, "Disk read should be non-negative");
        assert!(write >= 0, "Disk write should be non-negative");
    }

    /// Test: Network I/O statistics
    #[test]
    fn test_get_network_io() {
        let monitor = SystemMonitor::new().unwrap();
        let io = monitor.get_network_io();
        
        assert!(io.is_ok(), "Should retrieve network I/O stats successfully");
        
        let (received, sent) = io.unwrap();
        assert!(received >= 0, "Bytes received should be non-negative");
        assert!(sent >= 0, "Bytes sent should be non-negative");
    }

    /// Test: Concurrent monitoring
    #[test]
    fn test_concurrent_monitoring() {
        use std::sync::Arc;
        use std::thread;
        
        let monitor = Arc::new(SystemMonitor::new().unwrap());
        let mut handles = vec![];
        
        for _ in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let _metrics = monitor_clone.get_metrics();
                let _cpu = monitor_clone.get_cpu_usage();
                let _memory = monitor_clone.get_memory_usage();
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Concurrent monitoring should succeed");
        }
    }

    /// Test: Metrics snapshot
    #[test]
    fn test_metrics_snapshot() {
        let monitor = SystemMonitor::new().unwrap();
        let snapshot = monitor.take_snapshot();
        
        assert!(snapshot.is_ok(), "Should take metrics snapshot successfully");
        
        let snapshot = snapshot.unwrap();
        assert!(snapshot.timestamp > 0, "Snapshot should have valid timestamp");
    }

    /// Test: Compare snapshots
    #[test]
    fn test_compare_snapshots() {
        let monitor = SystemMonitor::new().unwrap();
        
        let snapshot1 = monitor.take_snapshot().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let snapshot2 = monitor.take_snapshot().unwrap();
        
        let diff = monitor.compare_snapshots(&snapshot1, &snapshot2);
        assert!(diff.is_ok(), "Should compare snapshots successfully");
    }
}