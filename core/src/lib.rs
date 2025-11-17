//! GhostAntivirus Core Engine
//! 
//! High-performance malware protection engine with real-time monitoring,
//! threat detection, and system protection capabilities.

pub mod config;
pub mod scanner;
pub mod monitor;
pub mod analyzer;
pub mod quarantine;
pub mod updater;
pub mod utils;
pub mod ai;
pub mod network;
pub mod firewall;
pub mod crypto;
pub mod api;
pub mod signatures;
pub mod system;
pub mod updates;
pub mod websocket;

// Re-export commonly used types
pub use analyzer::{BehaviorAnalyzer, RiskScore, Alert};
pub use quarantine::{QuarantineManager, QuarantineEntry};
pub use updater::{UpdateManager, Update, Version};
pub use ai::{AIIntegration, AIResponse};
pub use network::{NetworkMonitor, Connection, TrafficStats};
pub use firewall::{FirewallIntegration, FirewallRule, Action};
pub use crypto::{EncryptionKey, Algorithm};
   pub use signatures::{SignatureDatabase, Signature, DatabaseStats};
   pub use websocket::{WSBroadcaster, WSMessage};

use anyhow::Result;
use log::info;

/// Main engine structure
pub struct GhostEngine {
    pub config: config::Config,
    pub scanner: scanner::Scanner,
    pub monitor: monitor::ProcessMonitor,
    pub analyzer: analyzer::BehaviorAnalyzer,
    pub quarantine: quarantine::QuarantineManager,
    pub updater: updater::UpdateManager,
    pub ai: ai::AIIntegration,
    pub network: network::NetworkMonitor,
    pub firewall: firewall::FirewallIntegration,
}

impl GhostEngine {
    /// Create new GhostAntivirus engine instance
    pub fn new() -> Result<Self> {
        let config = config::Config::load()?;
        
        info!("Initializing GhostAntivirus Core Engine v{}", env!("CARGO_PKG_VERSION"));
        
        let scanner = scanner::Scanner::new(config.clone())?;
        let monitor = monitor::ProcessMonitor::new(config.clone())?;
        let analyzer = analyzer::BehaviorAnalyzer::new(config.clone())?;
        let quarantine = quarantine::QuarantineManager::new(&config.quarantine.path)?;
        let updater = updater::UpdateManager::new(&config)?;
        let ai = ai::AIIntegration::new(&config)?;
        let network = network::NetworkMonitor::new(config.clone())?;
        let firewall = firewall::FirewallIntegration::new()?;
        
        Ok(Self {
            config,
            scanner,
            monitor,
            analyzer,
            quarantine,
            updater,
            ai,
            network,
            firewall,
        })
    }
    
    /// Start the engine protection
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting GhostAntivirus protection...");
        
        // Start real-time monitoring
        self.monitor.start().await?;
        
        // Initialize AI engine connection
        self.ai.initialize().await?;
        
        info!("GhostAntivirus protection started successfully");
        Ok(())
    }
    
    /// Stop the engine protection
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping GhostAntivirus protection...");
        
        self.monitor.stop().await?;
        
        info!("GhostAntivirus protection stopped");
        Ok(())
    }
    
    /// Perform quick system scan
    pub async fn quick_scan(&self) -> Result<scanner::ScanResult> {
        info!("Starting quick system scan...");
        let result = self.scanner.quick_scan().await?;
        info!("Quick scan completed: {} files scanned", result.files_scanned);
        Ok(result)
    }
    
    /// Perform full system scan
    pub async fn full_scan(&self) -> Result<scanner::ScanResult> {
        info!("Starting full system scan...");
        let result = self.scanner.full_scan().await?;
        info!("Full scan completed: {} files scanned", result.files_scanned);
        Ok(result)
    }
    
    /// Scan specific file or directory
    pub async fn scan_path(&self, path: &str) -> Result<scanner::ScanResult> {
        info!("Scanning path: {}", path);
        let result = self.scanner.scan_path(path).await?;
        Ok(result)
    }
    
    /// Get engine status
    pub fn get_status(&self) -> EngineStatus {
        EngineStatus {
            version: env!("CARGO_PKG_VERSION").to_string(),
            is_running: self.monitor.is_running(),
            total_files_scanned: self.scanner.get_stats().total_files_scanned,
            threats_detected: self.scanner.get_stats().threats_detected,
            quarantine_count: self.quarantine.get_count(),
        }
    }
}

/// Engine status information
#[derive(Debug, serde::Serialize)]
pub struct EngineStatus {
    pub version: String,
    pub is_running: bool,
    pub total_files_scanned: u64,
    pub threats_detected: u64,
    pub quarantine_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_creation() {
        let engine = GhostEngine::new();
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_engine_status() {
        let engine = GhostEngine::new().unwrap();
        let status = engine.get_status();
        assert_eq!(status.version, "3.0.0");
    }
}