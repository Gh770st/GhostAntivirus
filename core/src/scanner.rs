//! File scanning module for malware detection

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use log::{info, warn, debug, error};
use crate::config::Config;
use uuid::Uuid;

/// Scanner for malware detection
pub struct Scanner {
    config: Config,
    stats: Arc<Mutex<ScanStats>>,
    current_state: Arc<Mutex<Option<ScanState>>>,
}

/// Scan result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub start_time: u64,
    pub end_time: u64,
    pub files_scanned: u64,
    pub threats_found: Vec<ThreatInfo>,
    pub errors: Vec<String>,
}

/// Scan type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanType {
    Quick,
    Full,
    Custom(String),
}

/// Threat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatInfo {
    pub file_path: PathBuf,
    pub threat_type: ThreatType,
    pub threat_name: String,
    pub severity: Severity,
    pub hash: String,
    pub size: u64,
}

/// Threat classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    Virus,
    Trojan,
    Worm,
    Ransomware,
    Spyware,
    Adware,
    Rootkit,
    Backdoor,
    Unknown,
}

/// Threat severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Scan statistics
#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    pub total_files_scanned: u64,
    pub threats_detected: u64,
    pub files_quarantined: u64,
    pub scan_time_seconds: u64,
    pub is_scanning: bool,
    pub progress_percentage: f32,
    pub last_scan_time: Option<u64>,
    pub current_scan_id: Option<String>,
}

/// Scan state for tracking active scans
#[derive(Debug, Clone)]
pub struct ScanState {
    pub scan_id: String,
    pub scan_type: ScanType,
    pub is_paused: bool,
    pub files_scanned: u64,
    pub threats_found: u64,
    pub progress: f32,
    pub start_time: u64,
}

impl Scanner {
    /// Create new scanner instance
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            stats: Arc::new(Mutex::new(ScanStats::default())),
            current_state: Arc::new(Mutex::new(None)),
        })
    }
    
    /// Start a new scan and return scan ID
    pub async fn start_scan(&self, path: &str, scan_type: ScanType, deep_scan: bool) -> Result<String> {
        // Check if a scan is already running
        {
            let state = self.current_state.lock().unwrap();
            if state.is_some() {
                return Err(anyhow!("A scan is already in progress"));
            }
        }
        
        let scan_id = Uuid::new_v4().to_string();
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Initialize scan state
        {
            let mut state = self.current_state.lock().unwrap();
            *state = Some(ScanState {
                scan_id: scan_id.clone(),
                scan_type: scan_type.clone(),
                is_paused: false,
                files_scanned: 0,
                threats_found: 0,
                progress: 0.0,
                start_time,
            });
        }
        
        // Update stats
        {
            let mut stats = self.stats.lock().unwrap();
            stats.is_scanning = true;
            stats.current_scan_id = Some(scan_id.clone());
            stats.progress_percentage = 0.0;
        }
        
        // Start the actual scan in background
        let scanner = self.clone_for_async();
        let path_owned = path.to_string();
        tokio::spawn(async move {
            let result = match scan_type {
                ScanType::Quick => scanner.quick_scan().await,
                ScanType::Full => scanner.full_scan().await,
                ScanType::Custom(_) => scanner.scan_path(&path_owned).await,
            };
            
            // Update final stats
            if let Ok(scan_result) = result {
                let mut stats = scanner.stats.lock().unwrap();
                stats.total_files_scanned += scan_result.files_scanned;
                stats.threats_detected += scan_result.threats_found.len() as u64;
                stats.is_scanning = false;
                stats.progress_percentage = 100.0;
                stats.last_scan_time = Some(scan_result.end_time);
                stats.current_scan_id = None;
            }
            
            // Clear scan state
            let mut state = scanner.current_state.lock().unwrap();
            *state = None;
        });
        
        Ok(scan_id)
    }
    
    /// Stop the current scan
    pub async fn stop_scan(&self) -> Result<()> {
        let mut state = self.current_state.lock().unwrap();
        if state.is_none() {
            return Err(anyhow!("No scan is currently running"));
        }
        
        *state = None;
        
        let mut stats = self.stats.lock().unwrap();
        stats.is_scanning = false;
        stats.current_scan_id = None;
        
        info!("Scan stopped by user");
        Ok(())
    }
    
    /// Pause the current scan
    pub async fn pause_scan(&self) -> Result<()> {
        let mut state = self.current_state.lock().unwrap();
        match state.as_mut() {
            Some(s) => {
                s.is_paused = true;
                info!("Scan paused");
                Ok(())
            }
            None => Err(anyhow!("No scan is currently running")),
        }
    }
    
    /// Resume the paused scan
    pub async fn resume_scan(&self) -> Result<()> {
        let mut state = self.current_state.lock().unwrap();
        match state.as_mut() {
            Some(s) => {
                if !s.is_paused {
                    return Err(anyhow!("Scan is not paused"));
                }
                s.is_paused = false;
                info!("Scan resumed");
                Ok(())
            }
            None => Err(anyhow!("No scan is currently running")),
        }
    }
    
    /// Get current scan statistics
    pub fn get_statistics(&self) -> ScanStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Clone scanner for async operations
    fn clone_for_async(&self) -> Self {
        Self {
            config: self.config.clone(),
            stats: Arc::clone(&self.stats),
            current_state: Arc::clone(&self.current_state),
        }
    }
    
    /// Perform quick system scan
    pub async fn quick_scan(&self) -> Result<ScanResult> {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        info!("Starting quick system scan");
        
        // Scan common system directories
        let scan_paths = vec![
            PathBuf::from("/bin"),
            PathBuf::from("/usr/bin"),
            PathBuf::from("/lib"),
            PathBuf::from("/usr/lib"),
            #[cfg(target_os = "windows")]
            PathBuf::from("C:\\Windows\\System32"),
        ];
        
        let mut result = ScanResult {
            scan_type: ScanType::Quick,
            start_time,
            end_time: 0,
            files_scanned: 0,
            threats_found: Vec::new(),
            errors: Vec::new(),
        };
        
        for path in scan_paths {
            if path.exists() {
                match self.scan_directory(&path, false).await {
                    Ok(mut scan_result) => {
                        result.files_scanned += scan_result.files_scanned;
                        result.threats_found.append(&mut scan_result.threats_found);
                        result.errors.append(&mut scan_result.errors);
                    }
                    Err(e) => {
                        warn!("Failed to scan directory {}: {}", path.display(), e);
                        result.errors.push(format!("Scan error for {}: {}", path.display(), e));
                    }
                }
            }
        }
        
        result.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        info!("Quick scan completed: {} files scanned, {} threats found", 
              result.files_scanned, result.threats_found.len());
        
        Ok(result)
    }
    
    /// Perform full system scan
    pub async fn full_scan(&self) -> Result<ScanResult> {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        info!("Starting full system scan");
        
        let scan_path = if cfg!(target_os = "windows") {
            PathBuf::from("C:")
        } else {
            PathBuf::from("/")
        };
        
        let mut result = self.scan_directory(&scan_path, true).await?;
        result.scan_type = ScanType::Full;
        result.start_time = start_time;
        
        info!("Full scan completed: {} files scanned, {} threats found", 
              result.files_scanned, result.threats_found.len());
        
        Ok(result)
    }
    
    /// Scan specific path
    pub async fn scan_path(&self, path: &str) -> Result<ScanResult> {
        let scan_path = PathBuf::from(path);
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if !scan_path.exists() {
            return Err(anyhow!("Path does not exist: {}", path));
        }
        
        let mut result = if scan_path.is_file() {
            self.scan_file(&scan_path).await?
        } else {
            self.scan_directory(&scan_path, true).await?
        };
        
        result.scan_type = ScanType::Custom(path.to_string());
        result.start_time = start_time;
        
        Ok(result)
    }
    
    /// Scan directory recursively
    async fn scan_directory(&self, dir_path: &Path, recursive: bool) -> Result<ScanResult> {
        let mut result = ScanResult {
            scan_type: ScanType::Custom(dir_path.to_string_lossy().to_string()),
            start_time: 0,
            end_time: 0,
            files_scanned: 0,
            threats_found: Vec::new(),
            errors: Vec::new(),
        };
        
        let walker = if recursive {
            WalkDir::new(dir_path)
        } else {
            WalkDir::new(dir_path).max_depth(1)
        };
        
        let files: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_path_buf())
            .filter(|path| self.should_scan_file(path))
            .collect();
        
        // Scan files in parallel
        let scan_results: Vec<Result<Option<ThreatInfo>>> = files
            .par_iter()
            .map(|path| self.scan_single_file(path))
            .collect();
        
        for scan_result in scan_results {
            result.files_scanned += 1;
            
            match scan_result {
                Ok(Some(threat)) => result.threats_found.push(threat),
                Ok(None) => {}, // No threat found
                Err(e) => result.errors.push(e.to_string()),
            }
        }
        
        Ok(result)
    }
    
    /// Scan single file
    async fn scan_file(&self, file_path: &Path) -> Result<ScanResult> {
        let threat = self.scan_single_file(file_path)?;
        
        let result = ScanResult {
            scan_type: ScanType::Custom(file_path.to_string_lossy().to_string()),
            start_time: 0,
            end_time: 0,
            files_scanned: 1,
            threats_found: threat.map(|t| vec![t]).unwrap_or_default(),
            errors: Vec::new(),
        };
        
        Ok(result)
    }
    
    /// Scan single file for threats
    fn scan_single_file(&self, file_path: &Path) -> Result<Option<ThreatInfo>> {
        // Get file metadata
        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();
        
        // Skip files that are too large
        if file_size > self.config.scanner.max_file_size_mb * 1024 * 1024 {
            debug!("Skipping large file: {} ({} bytes)", file_path.display(), file_size);
            return Ok(None);
        }
        
        // Calculate file hash
        let hash = self.calculate_file_hash(file_path)?;
        
        // Signature-based detection
        if let Some(threat) = self.signature_detection(file_path, &hash, file_size)? {
            return Ok(Some(threat));
        }
        
        // Heuristic detection
        if self.config.scanner.enable_heuristics {
            if let Some(threat) = self.heuristic_detection(file_path, file_size)? {
                return Ok(Some(threat));
            }
        }
        
        // AI-based detection
        if self.config.scanner.enable_ai {
            // AI-based detection
            let mut ai_integration = crate::ai::AIIntegration::new(&self.config)?;
            match ai_integration.analyze_file(file_path) {
                Ok(ai_response) if ai_response.is_malicious => {
                    return Ok(Some(ThreatInfo {
                        file_path: file_path.to_path_buf(),
                        threat_type: ThreatType::Unknown,
                        threat_name: ai_response.threat_type.unwrap_or_else(|| "AI.Detected".to_string()),
                        severity: if ai_response.confidence > 0.8 { Severity::High } 
                                 else if ai_response.confidence > 0.6 { Severity::Medium } 
                                 else { Severity::Low },
                        hash: hash.to_string(),
                           size: file_size,
                    }));
                }
                Ok(_) => {
                    debug!("AI analysis completed, no threat detected");
                }
                Err(e) => {
                    warn!("AI analysis failed for {}: {}", file_path.display(), e);
                }
            }
        }
        
        Ok(None)
    }
    
    /// Parse threat type from string
    fn parse_threat_type(&self, threat_type: &str) -> ThreatType {
        match threat_type.to_lowercase().as_str() {
            "virus" => ThreatType::Virus,
            "trojan" => ThreatType::Trojan,
            "worm" => ThreatType::Worm,
            "spyware" => ThreatType::Spyware,
            "adware" => ThreatType::Adware,
            "rootkit" => ThreatType::Rootkit,
            "ransomware" => ThreatType::Ransomware,
            "pua" => ThreatType::Adware,
            _ => ThreatType::Unknown,
        }
    }
    
    /// Convert severity number to Severity enum
    fn severity_from_number(&self, severity: u8) -> Severity {
        match severity {
            1..=3 => Severity::Low,
            4..=6 => Severity::Medium,
            7..=8 => Severity::High,
            9..=10 => Severity::Critical,
            _ => Severity::Medium,
        }
    }
    
    /// Check if file should be scanned
    fn should_scan_file(&self, file_path: &Path) -> bool {
        // Check excluded extensions
        if let Some(extension) = file_path.extension() {
            let ext_str = extension.to_string_lossy();
            if self.config.scanner.excluded_extensions.contains(&ext_str.to_string()) {
                return false;
            }
        }
        
        // Check excluded paths
        for excluded_path in &self.config.scanner.excluded_paths {
            if file_path.starts_with(excluded_path) {
                return false;
            }
        }
        
        true
    }
    
    /// Calculate SHA256 hash of file
    fn calculate_file_hash(&self, file_path: &Path) -> Result<String> {
        let mut file = fs::File::open(file_path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
    
    /// Signature-based threat detection
    fn signature_detection(&self, file_path: &Path, hash: &str, size: u64) -> Result<Option<ThreatInfo>> {
        // Check for EICAR test signature
        if hash == "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f" {
            return Ok(Some(ThreatInfo {
                file_path: file_path.to_path_buf(),
                threat_type: ThreatType::Virus,
                threat_name: "EICAR-Test-File".to_string(),
                severity: Severity::Low,
                hash: hash.to_string(),
                size,
            }));
        }
        
        // Real signature database lookup
        let signature_db = crate::signatures::SignatureDatabase::new(self.config.clone())?;
        if let Some(signature) = signature_db.scan_file(file_path)? {
            return Ok(Some(ThreatInfo {
                file_path: file_path.to_path_buf(),
                threat_type: self.parse_threat_type(&signature.threat_type),
                threat_name: signature.threat_name.clone(),
                severity: self.severity_from_number(signature.severity),
                hash: signature.hash.clone(),
                size,
            }));
        }
        
        // Fallback pattern matching for demonstration
        let content = fs::read(file_path)?;
        if content.windows(4).any(|window| window == b"EVIL") {
            return Ok(Some(ThreatInfo {
                file_path: file_path.to_path_buf(),
                threat_type: ThreatType::Trojan,
                threat_name: "Generic.Trojan.EVIL".to_string(),
                severity: Severity::High,
                hash: hash.to_string(),
                size,
            }));
        }
        
        Ok(None)
    }
    
    /// Heuristic threat detection
    fn heuristic_detection(&self, file_path: &Path, size: u64) -> Result<Option<ThreatInfo>> {
        // Simple heuristic checks
        if file_path.extension().map_or(false, |ext| ext == "exe") {
            // Check for suspicious characteristics in executable
            if size < 1024 * 100 { // Very small executable
                return Ok(Some(ThreatInfo {
                    file_path: file_path.to_path_buf(),
                    threat_type: ThreatType::Trojan,
                    threat_name: "Heuristic.Small.Executable".to_string(),
                    severity: Severity::Medium,
                    hash: "".to_string(),
                    size,
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Get scan statistics (deprecated, use get_statistics instead)
    pub fn get_stats(&self) -> ScanStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Update scan progress
    fn update_progress(&self, files_scanned: u64, threats_found: u64, progress: f32) {
        if let Some(ref mut state) = *self.current_state.lock().unwrap() {
            state.files_scanned = files_scanned;
            state.threats_found = threats_found;
            state.progress = progress;
        }
        
        let mut stats = self.stats.lock().unwrap();
        stats.progress_percentage = progress;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_scanner_creation() {
        let config = Config::default();
        let scanner = Scanner::new(config);
        assert!(scanner.is_ok());
    }
    
    #[tokio::test]
    async fn test_scan_nonexistent_path() {
        let config = Config::default();
        let scanner = Scanner::new(config).unwrap();
        let result = scanner.scan_path("/nonexistent/path").await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_file_hash_calculation() {
        let config = Config::default();
        let scanner = Scanner::new(config).unwrap();
        
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test content").unwrap();
        
        let hash = scanner.calculate_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash, "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72");
    }
}