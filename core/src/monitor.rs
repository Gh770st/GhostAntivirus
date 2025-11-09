//! Process monitoring for real-time protection

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;
use log::{info, warn, error};
use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use crate::config::Config;

/// Process monitor for real-time protection
pub struct ProcessMonitor {
    config: Config,
    system: Arc<Mutex<System>>,
    is_running: Arc<Mutex<bool>>,
    process_history: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: Option<PathBuf>,
    pub cmd: Vec<String>,
    pub start_time: u64,
    pub parent_pid: Option<u32>,
    pub user_id: Option<u32>,
    pub memory_kb: u64,
    pub cpu_percent: f32,
    pub threat_score: u8,
}

/// Process event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessEvent {
    Started(ProcessInfo),
    Terminated(u32),
    SuspiciousActivity(ProcessInfo, String),
}

impl ProcessMonitor {
    /// Create new process monitor
    pub fn new(config: Config) -> Result<Self> {
        let system = System::new_all();
        
        Ok(Self {
            config,
            system: Arc::new(Mutex::new(system)),
            is_running: Arc::new(Mutex::new(false)),
            process_history: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Start process monitoring
    pub async fn start(&self) -> Result<()> {
        if !self.config.monitor.enabled {
            info!("Process monitoring is disabled in configuration");
            return Ok(());
        }
        
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Ok(());
        }
        
        *is_running = true;
        info!("Starting process monitoring");
        
        let system = self.system.clone();
        let process_history = self.process_history.clone();
        let config = self.config.clone();
        let running = self.is_running.clone();
        
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_millis(config.monitor.monitor_interval_ms));
            
            loop {
                // Check if we should stop
                {
                    let running = running.lock().unwrap();
                    if !*running {
                        info!("Process monitoring stopped");
                        break;
                    }
                }
                
                ticker.tick().await;
                
                // Update system information
                {
                    let mut sys = system.lock().unwrap();
                    sys.refresh_all();
                }
                
                // Check for new processes
                if let Err(e) = Self::check_processes(&system, &process_history, &config).await {
                    error!("Error checking processes: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Stop process monitoring
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping process monitoring");
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        Ok(())
    }
    
    /// Check if monitor is running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
    
    /// Get current process list
    pub fn get_processes(&self) -> Result<Vec<ProcessInfo>> {
        let system = self.system.lock().unwrap();
        let mut processes = Vec::new();
        
        for (pid, process) in system.processes() {
            let process_info = ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                path: Some(process.exe().to_path_buf()),
                cmd: process.cmd().to_vec(),
                start_time: process.start_time(),
                parent_pid: process.parent().map(|p| p.as_u32()),
                user_id: process.user_id().map(|uid| **uid as u32),
                memory_kb: process.memory(),
                cpu_percent: process.cpu_usage(),
                threat_score: 0, // Will be calculated
            };
            
            processes.push(process_info);
        }
        
        Ok(processes)
    }
    
    /// Analyze process for suspicious behavior
    pub fn analyze_process(&self, process_info: &ProcessInfo) -> Result<u8> {
        let mut threat_score = 0;
        
        // Check for suspicious process names
        let suspicious_names = vec![
            "svchost.exe", // Often spoofed
            "explorer.exe", // Often spoofed
            "winlogon.exe", // System critical
            "csrss.exe",    // System critical
        ];
        
        if suspicious_names.contains(&process_info.name.as_str()) {
            if let Some(path) = &process_info.path {
                // Check if it's running from unusual location
                if !path.starts_with("C:\\Windows\\System32") && 
                   !path.starts_with("/usr/bin") && 
                   !path.starts_with("/bin") {
                    threat_score += 30;
                    warn!("Suspicious process {} running from: {}", process_info.name, path.display());
                }
            }
        }
        
        // Check for high CPU usage
        if process_info.cpu_percent > 90.0 {
            threat_score += 20;
        }
        
        // Check for high memory usage
        if process_info.memory_kb > 1024 * 1024 { // > 1GB
            threat_score += 15;
        }
        
        // Check for no parent process (potential rootkit)
        if process_info.parent_pid.is_none() && process_info.pid != 1 {
            threat_score += 25;
        }
        
        // Check for suspicious command line arguments
        let suspicious_args = vec![
            "-s", "-f", "-hidden", "-silent", "-background"
        ];
        
        for arg in &process_info.cmd {
            if suspicious_args.contains(&arg.as_str()) {
                threat_score += 10;
                break;
            }
        }
        
        Ok(threat_score.min(100))
    }
    
    /// Check processes for new and suspicious activity
    async fn check_processes(
        system: &Arc<Mutex<System>>,
        process_history: &Arc<Mutex<HashMap<u32, ProcessInfo>>>,
        config: &Config,
    ) -> Result<()> {
        let sys = system.lock().unwrap();
        let mut history = process_history.lock().unwrap();
        let mut new_processes = Vec::new();
        
        // Find new processes
        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();
            
            if !history.contains_key(&pid_u32) {
                let process_info = ProcessInfo {
                    pid: pid_u32,
                    name: process.name().to_string(),
                    path: Some(process.exe().to_path_buf()),
                    cmd: process.cmd().to_vec(),
                    start_time: process.start_time(),
                    parent_pid: process.parent().map(|p| p.as_u32()),
                    user_id: process.user_id().map(|uid| **uid as u32),
                    memory_kb: process.memory(),
                    cpu_percent: process.cpu_usage(),
                    threat_score: 0,
                };
                
                new_processes.push(process_info.clone());
                history.insert(pid_u32, process_info);
            }
        }
        
        // Remove terminated processes
        let current_pids: std::collections::HashSet<u32> = 
            sys.processes().keys().map(|pid| pid.as_u32()).collect();
        
        history.retain(|pid, _| current_pids.contains(pid));
        
        // Analyze new processes
        for process_info in new_processes {
            let monitor = ProcessMonitor {
                config: config.clone(),
                system: system.clone(),
                is_running: Arc::new(Mutex::new(true)),
                process_history: process_history.clone(),
            };
            
            let threat_score = monitor.analyze_process(&process_info)?;
            
            if threat_score >= config.monitor.suspicious_process_threshold as u8 {
                warn!("Suspicious process detected: {} (PID: {}, Score: {})", 
                      process_info.name, process_info.pid, threat_score);
                
                // Log threat detection
                info!("Threat detected - Process: {} (PID: {}, Score: {})", 
                     process_info.name, process_info.pid, threat_score);
                
                // In production, send alert via WebSocket or notification system
                // For now, just log the detection
            }
            
            // Update threat score in history
            if let Some(stored_process) = history.get_mut(&process_info.pid) {
                stored_process.threat_score = threat_score;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[tokio::test]
    async fn test_monitor_creation() {
        let config = Config::default();
        let monitor = ProcessMonitor::new(config);
        assert!(monitor.is_ok());
    }
    
    #[test]
    fn test_process_analysis() {
        let config = Config::default();
        let monitor = ProcessMonitor::new(config).unwrap();
        
        let process_info = ProcessInfo {
            pid: 1234,
            name: "test.exe".to_string(),
            path: Some(PathBuf::from("/tmp/test.exe")),
            cmd: vec!["test.exe".to_string(), "-hidden".to_string()],
            start_time: 0,
            parent_pid: Some(1),
            user_id: Some(1000),
            memory_kb: 1024,
            cpu_percent: 50.0,
            threat_score: 0,
        };
        
        let score = monitor.analyze_process(&process_info).unwrap();
        assert!(score > 0);
    }
}