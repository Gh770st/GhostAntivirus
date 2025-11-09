//! System Monitoring Module
//! 
//! Provides system resource monitoring, metrics collection, and performance tracking.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt};
use std::sync::{Arc, Mutex};

/// System monitor for tracking resource usage
pub struct SystemMonitor {
    system: Arc<Mutex<System>>,
}

/// System metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: ResourceUsage,
    pub disk_usage: Vec<DiskUsage>,
    pub network_usage: NetworkUsage,
    pub process_count: usize,
    pub uptime: u64,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub percentage: f32,
}

/// Disk usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskUsage {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub percentage: f32,
}

/// Network usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    pub received: u64,
    pub transmitted: u64,
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();
        
        Ok(Self {
            system: Arc::new(Mutex::new(system)),
        })
    }

    /// Get current system metrics
    pub fn get_metrics(&self) -> Result<SystemMetrics> {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();

        // CPU usage
        let cpu_usage = system.global_cpu_info().cpu_usage();

        // Memory usage
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let available_memory = system.available_memory();
        let memory_percentage = (used_memory as f32 / total_memory as f32) * 100.0;

        let memory_usage = ResourceUsage {
            total: total_memory,
            used: used_memory,
            available: available_memory,
            percentage: memory_percentage,
        };

        // Disk usage
        let disk_usage: Vec<DiskUsage> = system.disks().iter().map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let percentage = (used as f32 / total as f32) * 100.0;

            DiskUsage {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total,
                available,
                used,
                percentage,
            }
        }).collect();

        // Network usage
        let network_usage = NetworkUsage {
            received: system.networks().iter().map(|(_, data)| data.received()).sum(),
            transmitted: system.networks().iter().map(|(_, data)| data.transmitted()).sum(),
        };

        // Process count
        let process_count = system.processes().len();

        // System uptime
        let uptime = system.uptime();

        Ok(SystemMetrics {
            cpu_usage,
            memory_usage,
            disk_usage,
            network_usage,
            process_count,
            uptime,
        })
    }

    /// Get CPU usage percentage
    pub fn get_cpu_usage(&self) -> Result<f32> {
        let mut system = self.system.lock().unwrap();
        system.refresh_cpu();
        Ok(system.global_cpu_info().cpu_usage())
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> Result<ResourceUsage> {
        let mut system = self.system.lock().unwrap();
        system.refresh_memory();

        let total = system.total_memory();
        let used = system.used_memory();
        let available = system.available_memory();
        let percentage = (used as f32 / total as f32) * 100.0;

        Ok(ResourceUsage {
            total,
            used,
            available,
            percentage,
        })
    }

    /// Get disk usage for all disks
    pub fn get_disk_usage(&self) -> Result<Vec<DiskUsage>> {
        let mut system = self.system.lock().unwrap();
        system.refresh_disks_list();

        let disk_usage: Vec<DiskUsage> = system.disks().iter().map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let percentage = (used as f32 / total as f32) * 100.0;

            DiskUsage {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total,
                available,
                used,
                percentage,
            }
        }).collect();

        Ok(disk_usage)
    }

    /// Get system uptime in seconds
    pub fn get_uptime(&self) -> Result<u64> {
        let system = self.system.lock().unwrap();
        Ok(system.uptime())
    }

    /// Get process count
    pub fn get_process_count(&self) -> Result<usize> {
        let mut system = self.system.lock().unwrap();
        system.refresh_processes();
        Ok(system.processes().len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_new() {
        let monitor = SystemMonitor::new();
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_get_metrics() {
        let monitor = SystemMonitor::new().unwrap();
        let metrics = monitor.get_metrics();
        assert!(metrics.is_ok());
        
        let metrics = metrics.unwrap();
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage.total > 0);
        assert!(metrics.process_count > 0);
    }

    #[test]
    fn test_get_cpu_usage() {
        let monitor = SystemMonitor::new().unwrap();
        let cpu_usage = monitor.get_cpu_usage();
        assert!(cpu_usage.is_ok());
        assert!(cpu_usage.unwrap() >= 0.0);
    }

    #[test]
    fn test_get_memory_usage() {
        let monitor = SystemMonitor::new().unwrap();
        let memory = monitor.get_memory_usage();
        assert!(memory.is_ok());
        
        let memory = memory.unwrap();
        assert!(memory.total > 0);
        assert!(memory.used <= memory.total);
    }
}