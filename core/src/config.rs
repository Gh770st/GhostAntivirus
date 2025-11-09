//! Configuration management for GhostAntivirus

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub scanner: ScannerConfig,
    pub monitor: MonitorConfig,
    pub quarantine: QuarantineConfig,
    pub analyzer: AnalyzerConfig,
    pub updater: UpdaterConfig,
    pub ai: AIConfig,
    pub updates: UpdatesConfig,
    pub network: NetworkConfig,
    #[serde(default)]
    pub notifications: NotificationConfig,
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub log_level: String,
    pub data_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub max_cpu_usage: u8,
    pub max_memory_mb: u32,
}

/// Scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub enable_heuristics: bool,
    pub enable_behavioral: bool,
    pub enable_ai: bool,
    pub max_file_size_mb: u64,
    pub scan_archives: bool,
    pub excluded_extensions: Vec<String>,
    pub excluded_paths: Vec<PathBuf>,
    pub thread_pool_size: usize,
    #[serde(default)]
    pub auto_scan: bool,
    #[serde(default)]
    pub schedule: Option<String>,
}

/// Process monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub enabled: bool,
    pub scan_new_processes: bool,
    pub scan_process_memory: bool,
    pub suspicious_process_threshold: u8,
    pub monitor_interval_ms: u64,
}

/// Quarantine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineConfig {
    pub path: PathBuf,
    pub max_size_gb: u32,
    pub auto_delete_days: u32,
    pub encryption_enabled: bool,
}

/// AI Engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub api_url: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub batch_size: usize,
    pub cache_ttl_seconds: u64,
}

/// Updates configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatesConfig {
    pub auto_check: bool,
    pub check_interval_hours: u32,
    pub beta_channel: bool,
    pub download_url: String,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub firewall_enabled: bool,
    pub vpn_enabled: bool,
    pub block_suspicious_ips: bool,
    pub monitor_network_traffic: bool,
}

/// Analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    pub enabled: bool,
    pub risk_threshold: f32,
    pub history_retention_seconds: u64,
}

/// Updater configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdaterConfig {
    pub enabled: bool,
    pub update_url: String,
    pub update_path: String,
    pub check_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            scanner: ScannerConfig::default(),
            monitor: MonitorConfig::default(),
            quarantine: QuarantineConfig::default(),
            analyzer: AnalyzerConfig::default(),
            updater: UpdaterConfig::default(),
            ai: AIConfig::default(),
            updates: UpdatesConfig::default(),
            network: NetworkConfig::default(),
                    notifications: NotificationConfig::default(),
}
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            data_dir: PathBuf::from("/var/lib/ghost-antivirus"),
            temp_dir: PathBuf::from("/tmp/ghost-antivirus"),
            max_cpu_usage: 80,
            max_memory_mb: 1024,
        }
    }
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            enable_heuristics: true,
            enable_behavioral: true,
            enable_ai: true,
            max_file_size_mb: 100,
            scan_archives: true,
            excluded_extensions: vec![
                ".tmp".to_string(),
                ".log".to_string(),
                ".cache".to_string(),
            ],
            excluded_paths: vec![
                PathBuf::from("/proc"),
                PathBuf::from("/sys"),
                PathBuf::from("/dev"),
            ],
            thread_pool_size: num_cpus::get(),
                    auto_scan: false,
            schedule: None,
}
    }
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_new_processes: true,
            scan_process_memory: false,
            suspicious_process_threshold: 3,
            monitor_interval_ms: 1000,
        }
    }
}

impl Default for QuarantineConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("/var/lib/ghost-antivirus/quarantine"),
            max_size_gb: 10,
            auto_delete_days: 30,
            encryption_enabled: true,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "http://localhost:8000".to_string(),
            endpoint: "http://localhost:8000".to_string(),
            api_key: None,
            timeout_seconds: 30,
            batch_size: 100,
            cache_ttl_seconds: 3600,
        }
    }
}

impl Default for UpdatesConfig {
    fn default() -> Self {
        Self {
            auto_check: true,
            check_interval_hours: 24,
            beta_channel: false,
            download_url: "https://api.ghostantivirus.com".to_string(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            firewall_enabled: true,
            vpn_enabled: false,
            block_suspicious_ips: true,
            monitor_network_traffic: true,
        }
    }
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            risk_threshold: 0.7,
            history_retention_seconds: 300,
        }
    }
}

impl Default for UpdaterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            update_url: "https://updates.ghostantivirus.com".to_string(),
            update_path: "/var/lib/ghost-antivirus/updates".to_string(),
            check_interval: 86400, // 24 hours
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub email: Option<String>,
    pub webhook_url: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            email: None,
            webhook_url: None,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// Get configuration file path
    pub fn get_config_path() -> PathBuf {
        if cfg!(target_os = "windows") {
            PathBuf::from("C:\\ProgramData\\GhostAntivirus\\config.toml")
        } else {
            PathBuf::from("/etc/ghost-antivirus/config.toml")
        }
    }
    
    /// Get runtime directories
    pub fn ensure_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.general.data_dir)?;
        fs::create_dir_all(&self.general.temp_dir)?;
        fs::create_dir_all(&self.quarantine.path)?;
        Ok(())
    }
}

// Add num_cpus dependency for thread count
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.log_level, "info");
        assert_eq!(config.scanner.enable_heuristics, true);
    }
    
    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(config.general.log_level, deserialized.general.log_level);
    }
}