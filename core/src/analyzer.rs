//! Behavior Analyzer Module
//! 
//! Analyzes process behavior patterns to detect malicious activities
//! including ransomware, data exfiltration, and privilege escalation.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use log::{info, warn, debug};

use crate::config::Config;
use crate::scanner::{ThreatType, Severity};

/// Process identifier type
pub type ProcessId = u32;

/// Behavior pattern types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BehaviorType {
    FileAccess,
    FileModification,
    FileEncryption,
    NetworkConnection,
    RegistryModification,
    ProcessCreation,
    PrivilegeEscalation,
    DataExfiltration,
}

/// Individual behavior record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub behavior_type: BehaviorType,
    pub timestamp: SystemTime,
    pub details: String,
    pub severity: u8, // 0-10
}

/// Behavior pattern for detection
#[derive(Debug, Clone)]
pub struct BehaviorPattern {
    pub name: String,
    pub behaviors: Vec<BehaviorType>,
    pub time_window: Duration,
    pub threshold: usize,
    pub risk_score: f32,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub score: f32, // 0.0 - 1.0
    pub level: RiskLevel,
    pub reasons: Vec<String>,
    pub detected_patterns: Vec<String>,
}

/// Risk level classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Alert generated for suspicious behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub process_id: ProcessId,
    pub process_name: String,
    pub risk_score: RiskScore,
    pub timestamp: SystemTime,
    pub recommended_action: Action,
}

/// Recommended action for detected threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Monitor,
    Quarantine,
    Terminate,
    Block,
}

/// Detected threat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedThreat {
    pub id: String,
    pub file_path: PathBuf,
    pub threat_type: ThreatType,
    pub threat_name: String,
    pub severity: Severity,
    pub detected_at: u64,
    pub action_taken: String,
    pub file_hash: String,
}

/// Behavior Analyzer
#[allow(dead_code)]
pub struct BehaviorAnalyzer {
    patterns: Vec<BehaviorPattern>,
    risk_threshold: f32,
    history: HashMap<ProcessId, Vec<Behavior>>,
    detected_threats: Arc<Mutex<Vec<DetectedThreat>>>,
    config: Config,
}

impl BehaviorAnalyzer {
    /// Create new behavior analyzer
    pub fn new(config: Config) -> Result<Self> {
        info!("Initializing Behavior Analyzer");
        
        let patterns = Self::load_default_patterns();
        let risk_threshold = config.analyzer.risk_threshold;
        let detected_threats = Arc::new(Mutex::new(Vec::new()));
        
        Ok(Self {
            patterns,
            risk_threshold,
            history: HashMap::new(),
            detected_threats,
            config,
        })
    }
    
    /// Load default behavior patterns
    fn load_default_patterns() -> Vec<BehaviorPattern> {
        vec![
            // Ransomware pattern
            BehaviorPattern {
                name: "Ransomware Activity".to_string(),
                behaviors: vec![
                    BehaviorType::FileEncryption,
                    BehaviorType::FileModification,
                    BehaviorType::NetworkConnection,
                ],
                time_window: Duration::from_secs(60),
                threshold: 10,
                risk_score: 0.9,
            },
            // Data exfiltration pattern
            BehaviorPattern {
                name: "Data Exfiltration".to_string(),
                behaviors: vec![
                    BehaviorType::FileAccess,
                    BehaviorType::NetworkConnection,
                ],
                time_window: Duration::from_secs(120),
                threshold: 50,
                risk_score: 0.8,
            },
            // Privilege escalation pattern
            BehaviorPattern {
                name: "Privilege Escalation".to_string(),
                behaviors: vec![
                    BehaviorType::PrivilegeEscalation,
                    BehaviorType::RegistryModification,
                    BehaviorType::ProcessCreation,
                ],
                time_window: Duration::from_secs(30),
                threshold: 3,
                risk_score: 0.85,
            },
        ]
    }
    
    /// Record a behavior for a process
    pub fn record_behavior(&mut self, process_id: ProcessId, behavior: Behavior) {
        debug!("Recording behavior for process {}: {:?}", process_id, behavior.behavior_type);
        
        self.history
            .entry(process_id)
            .or_insert_with(Vec::new)
            .push(behavior);
        
        // Clean old behaviors
        self.cleanup_old_behaviors(process_id);
    }
    
    /// Analyze process behavior and calculate risk score
    pub fn analyze_process(&mut self, process_id: ProcessId, process_name: &str) -> Result<RiskScore> {
        debug!("Analyzing process {} ({})", process_id, process_name);
        
        let behaviors = self.history.get(&process_id)
            .context("No behavior history for process")?;
        
        let mut detected_patterns = Vec::new();
        let mut reasons = Vec::new();
        let mut total_risk = 0.0;
        
        // Check for ransomware
        if self.detect_ransomware(behaviors) {
            detected_patterns.push("Ransomware Activity".to_string());
            reasons.push("Multiple file encryption operations detected".to_string());
            total_risk += 0.9;
        }
        
        // Check for data exfiltration
        if self.detect_data_exfiltration(behaviors) {
            detected_patterns.push("Data Exfiltration".to_string());
            reasons.push("Suspicious data transfer to external network".to_string());
            total_risk += 0.8;
        }
        
        // Check for privilege escalation
        if self.detect_privilege_escalation(behaviors) {
            detected_patterns.push("Privilege Escalation".to_string());
            reasons.push("Attempt to gain elevated privileges".to_string());
            total_risk += 0.85;
        }
        
        // Calculate final risk score
        let score = self.calculate_risk_score(behaviors, total_risk);
        let level = Self::classify_risk_level(score);
        
        Ok(RiskScore {
            score,
            level,
            reasons,
            detected_patterns,
        })
    }
    
    /// Detect ransomware behavior patterns
    pub fn detect_ransomware(&self, behaviors: &[Behavior]) -> bool {
        let recent_behaviors = self.get_recent_behaviors(behaviors, Duration::from_secs(60));
        
        let encryption_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::FileEncryption)
            .count();
        
        let modification_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::FileModification)
            .count();
        
        // Ransomware typically encrypts many files quickly
        encryption_count >= 10 || (encryption_count >= 5 && modification_count >= 20)
    }
    
    /// Detect data exfiltration patterns
    pub fn detect_data_exfiltration(&self, behaviors: &[Behavior]) -> bool {
        let recent_behaviors = self.get_recent_behaviors(behaviors, Duration::from_secs(120));
        
        let file_access_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::FileAccess)
            .count();
        
        let network_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::NetworkConnection)
            .count();
        
        // Data exfiltration: many file accesses followed by network activity
        file_access_count >= 50 && network_count >= 5
    }
    
    /// Detect privilege escalation attempts
    pub fn detect_privilege_escalation(&self, behaviors: &[Behavior]) -> bool {
        let recent_behaviors = self.get_recent_behaviors(behaviors, Duration::from_secs(30));
        
        let escalation_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::PrivilegeEscalation)
            .count();
        
        let registry_count = recent_behaviors.iter()
            .filter(|b| b.behavior_type == BehaviorType::RegistryModification)
            .count();
        
        // Privilege escalation with registry modifications
        escalation_count >= 1 || (registry_count >= 3)
    }
    
    /// Calculate overall risk score
    pub fn calculate_risk_score(&self, behaviors: &[Behavior], pattern_risk: f32) -> f32 {
        if behaviors.is_empty() {
            return 0.0;
        }
        
        // Calculate average severity
        let avg_severity: f32 = behaviors.iter()
            .map(|b| b.severity as f32)
            .sum::<f32>() / behaviors.len() as f32;
        
        // Normalize severity to 0.0-1.0
        let severity_score = avg_severity / 10.0;
        
        // Combine pattern risk and severity
        let combined_score = (pattern_risk * 0.7) + (severity_score * 0.3);
        
        // Clamp to 0.0-1.0
        combined_score.min(1.0).max(0.0)
    }
    
    /// Classify risk level based on score
    fn classify_risk_level(score: f32) -> RiskLevel {
        match score {
            s if s >= 0.8 => RiskLevel::Critical,
            s if s >= 0.6 => RiskLevel::High,
            s if s >= 0.3 => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }
    
    /// Generate alert for suspicious process
    pub fn generate_alert(&self, process_id: ProcessId, process_name: String, risk_score: RiskScore) -> Alert {
        let recommended_action = match risk_score.level {
            RiskLevel::Critical => Action::Terminate,
            RiskLevel::High => Action::Quarantine,
            RiskLevel::Medium => Action::Block,
            RiskLevel::Low => Action::Monitor,
        };
        
        warn!("Alert generated for process {} ({}): {:?} - Score: {:.2}", 
              process_id, process_name, risk_score.level, risk_score.score);
        
        Alert {
            process_id,
            process_name,
            risk_score,
            timestamp: SystemTime::now(),
            recommended_action,
        }
    }
    
    /// Get behaviors within time window
    fn get_recent_behaviors(&self, behaviors: &[Behavior], window: Duration) -> Vec<Behavior> {
        let now = SystemTime::now();
        behaviors.iter()
            .filter(|b| {
                if let Ok(elapsed) = now.duration_since(b.timestamp) {
                    elapsed <= window
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
    
    /// Clean up old behavior records
    fn cleanup_old_behaviors(&mut self, process_id: ProcessId) {
        if let Some(behaviors) = self.history.get_mut(&process_id) {
            let cutoff = SystemTime::now() - Duration::from_secs(300); // Keep 5 minutes
            behaviors.retain(|b| b.timestamp >= cutoff);
        }
    }
    
    /// Clear all behavior history
    pub fn clear_history(&mut self) {
        info!("Clearing behavior history");
        self.history.clear();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> AnalyzerStats {
        let total_processes = self.history.len();
        let total_behaviors: usize = self.history.values()
            .map(|v| v.len())
            .sum();
        
        AnalyzerStats {
            total_processes,
            total_behaviors,
            patterns_loaded: self.patterns.len(),
        }
    }
    
    /// Add detected threat
    pub fn add_threat(&self, threat: DetectedThreat) {
        let mut threats = self.detected_threats.lock().unwrap();
        let threat_name = threat.threat_name.clone();
        let file_path = threat.file_path.clone();
        threats.push(threat);
        info!("Threat added: {} - {}", threat_name, file_path.display());
    }
    
    /// Get all detected threats
    pub fn get_detected_threats(&self) -> Vec<DetectedThreat> {
        self.detected_threats.lock().unwrap().clone()
    }
    
    /// Clear detected threats
    pub fn clear_threats(&self) {
        let mut threats = self.detected_threats.lock().unwrap();
        threats.clear();
        info!("All threats cleared");
    }
    
    /// Remove specific threat by ID
    pub fn remove_threat(&self, threat_id: &str) -> Result<()> {
        let mut threats = self.detected_threats.lock().unwrap();
        let initial_len = threats.len();
        threats.retain(|t| t.id != threat_id);
        
        if threats.len() < initial_len {
            info!("Threat removed: {}", threat_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Threat not found: {}", threat_id))
        }
    }
}

/// Analyzer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerStats {
    pub total_processes: usize,
    pub total_behaviors: usize,
    pub patterns_loaded: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> Config {
        Config::default()
    }
    
    #[test]
    fn test_analyzer_creation() {
        let config = create_test_config();
        let analyzer = BehaviorAnalyzer::new(config);
        assert!(analyzer.is_ok());
    }
    
    #[test]
    fn test_ransomware_detection() {
        let config = create_test_config();
        let analyzer = BehaviorAnalyzer::new(config).unwrap();
        
        let mut behaviors = Vec::new();
        for _ in 0..15 {
            behaviors.push(Behavior {
                behavior_type: BehaviorType::FileEncryption,
                timestamp: SystemTime::now(),
                details: "Encrypted file".to_string(),
                severity: 8,
            });
        }
        
        assert!(analyzer.detect_ransomware(&behaviors));
    }
    
    #[test]
    fn test_risk_score_calculation() {
        let config = create_test_config();
        let analyzer = BehaviorAnalyzer::new(config).unwrap();
        
        let behaviors = vec![
            Behavior {
                behavior_type: BehaviorType::FileAccess,
                timestamp: SystemTime::now(),
                details: "Test".to_string(),
                severity: 5,
            },
        ];
        
        let score = analyzer.calculate_risk_score(&behaviors, 0.5);
        assert!(score >= 0.0 && score <= 1.0);
    }
    
    #[test]
    fn test_risk_level_classification() {
        assert_eq!(BehaviorAnalyzer::classify_risk_level(0.9), RiskLevel::Critical);
        assert_eq!(BehaviorAnalyzer::classify_risk_level(0.7), RiskLevel::High);
        assert_eq!(BehaviorAnalyzer::classify_risk_level(0.4), RiskLevel::Medium);
        assert_eq!(BehaviorAnalyzer::classify_risk_level(0.1), RiskLevel::Low);
    }
}