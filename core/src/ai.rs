//! AI Integration Module
//! 
//! Provides integration with the AI Engine for advanced threat detection
//! using machine learning models.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, bail};
use log::{info, warn, debug, error};
use reqwest;

use crate::config::Config;

/// AI analysis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub file_path: String,
    pub file_hash: String,
    pub file_size: u64,
    pub features: Vec<f32>,
}

/// AI analysis response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub is_malicious: bool,
    pub confidence: f32,
    pub threat_type: Option<String>,
    pub threat_family: Option<String>,
    pub risk_score: f32,
    pub analysis_time_ms: u64,
}

/// Threat report from AI Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatReport {
    pub hash: String,
    pub threat_name: String,
    pub threat_type: String,
    pub first_seen: String,
    pub last_seen: String,
    pub detection_count: u32,
    pub severity: String,
}

/// AI Integration Manager
pub struct AIIntegration {
    api_url: String,
    client: reqwest::blocking::Client,
    cache: HashMap<String, CachedResponse>,
    timeout: Duration,
    cache_ttl: Duration,
    enabled: bool,
}

/// Cached AI response
#[derive(Debug, Clone)]
struct CachedResponse {
    response: AIResponse,
    cached_at: SystemTime,
}

impl AIIntegration {
    /// Create new AI integration
    pub fn new(config: &Config) -> Result<Self> {
        info!("Initializing AI Integration");
        
        let api_url = config.ai.api_url.clone();
        let timeout = Duration::from_secs(config.ai.timeout_seconds);
        let cache_ttl = Duration::from_secs(config.ai.cache_ttl_seconds);
        let enabled = config.ai.enabled;
        
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            api_url,
            client,
            cache: HashMap::new(),
            timeout,
            cache_ttl,
            enabled,
        })
    }
    
    /// Initialize AI integration (async version for compatibility)
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.enabled {
            info!("AI integration is disabled");
            return Ok(());
        }
        
        info!("Initializing AI Engine connection");
        
        // Test connection to AI Engine
        match self.test_connection() {
            Ok(true) => {
                info!("AI Engine connection established successfully");
                Ok(())
            }
            Ok(false) => {
                warn!("AI Engine is not responding, continuing without AI support");
                self.enabled = false;
                Ok(())
            }
            Err(e) => {
                warn!("Failed to connect to AI Engine: {}, continuing without AI support", e);
                self.enabled = false;
                Ok(())
            }
        }
    }
    
    /// Analyze a file using AI Engine
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<AIResponse> {
        if !self.enabled {
            debug!("AI analysis disabled, returning default response");
            return Ok(Self::default_response());
        }
        
        info!("Analyzing file with AI: {:?}", file_path);
        
        // Calculate file hash for caching
        let file_hash = crate::utils::calculate_file_hash(file_path)?;
        
        // Check cache first
        if let Some(cached) = self.get_from_cache(&file_hash) {
            debug!("Using cached AI response for hash: {}", file_hash);
            return Ok(cached);
        }
        
        // Get file size
        let file_size = crate::utils::get_file_size(file_path)?;
        
        // Extract features (simplified - in production, extract real features)
        let features = self.extract_features(file_path)?;
        
        // Create request
        let request = AIRequest {
            file_path: file_path.to_string_lossy().to_string(),
            file_hash: file_hash.clone(),
            file_size,
            features,
        };
        
        // Send to AI Engine
        let response = self.send_request(&request)?;
        
        // Cache response
        self.cache_response(&file_hash, response.clone());
        
        info!("AI analysis complete: malicious={}, confidence={:.2}", 
              response.is_malicious, response.confidence);
        
        Ok(response)
    }
    
    /// Analyze multiple files in batch
    pub fn analyze_batch(&mut self, files: Vec<&Path>) -> Result<Vec<AIResponse>> {
        if !self.enabled {
            debug!("AI analysis disabled, returning default responses");
            return Ok(vec![Self::default_response(); files.len()]);
        }
        
        info!("Batch analyzing {} files with AI", files.len());
        
        let mut responses = Vec::new();
        
        for file in files {
            match self.analyze_file(file) {
                Ok(response) => responses.push(response),
                Err(e) => {
                    error!("Failed to analyze file {:?}: {}", file, e);
                    responses.push(Self::default_response());
                }
            }
        }
        
        Ok(responses)
    }
    
    /// Get threat report for a file hash
    pub fn get_threat_report(&self, hash: &str) -> Result<ThreatReport> {
        if !self.enabled {
            bail!("AI integration is disabled");
        }
        
        info!("Fetching threat report for hash: {}", hash);
        
        let url = format!("{}/api/v1/threat-report/{}", self.api_url, hash);
        
        let response = self.client.get(&url)
            .send()
            .context("Failed to fetch threat report")?;
        
        if !response.status().is_success() {
            bail!("Threat report request failed: {}", response.status());
        }
        
        let report: ThreatReport = response.json()
            .context("Failed to parse threat report")?;
        
        Ok(report)
    }
    
    /// Send analysis request to AI Engine
    fn send_request(&self, request: &AIRequest) -> Result<AIResponse> {
        let url = format!("{}/api/v1/analyze", self.api_url);
        
        debug!("Sending AI request to: {}", url);
        
        let response = self.client.post(&url)
            .json(request)
            .send()
            .context("Failed to send AI request")?;
        
        if !response.status().is_success() {
            bail!("AI request failed: {}", response.status());
        }
        
        let ai_response: AIResponse = response.json()
            .context("Failed to parse AI response")?;
        
        Ok(ai_response)
    }
    
    /// Extract features from file (simplified)
    fn extract_features(&self, _file_path: &Path) -> Result<Vec<f32>> {
        // In production, this would extract real features:
        // - File size, entropy, PE headers, imports, strings, etc.
        // For now, return dummy features
        Ok(vec![0.5; 30])
    }
    
    /// Get response from cache
    fn get_from_cache(&self, hash: &str) -> Option<AIResponse> {
        if let Some(cached) = self.cache.get(hash) {
            // Check if cache is still valid
            if let Ok(elapsed) = SystemTime::now().duration_since(cached.cached_at) {
                if elapsed < self.cache_ttl {
                    return Some(cached.response.clone());
                }
            }
        }
        None
    }
    
    /// Cache AI response
    fn cache_response(&mut self, hash: &str, response: AIResponse) {
        self.cache.insert(
            hash.to_string(),
            CachedResponse {
                response,
                cached_at: SystemTime::now(),
            },
        );
        
        // Limit cache size
        if self.cache.len() > 1000 {
            self.clear_old_cache_entries();
        }
    }
    
    /// Clear old cache entries
    fn clear_old_cache_entries(&mut self) {
        let now = SystemTime::now();
        self.cache.retain(|_, cached| {
            if let Ok(elapsed) = now.duration_since(cached.cached_at) {
                elapsed < self.cache_ttl
            } else {
                false
            }
        });
    }
    
    /// Clear all cache
    pub fn clear_cache(&mut self) {
        info!("Clearing AI response cache");
        self.cache.clear();
    }
    
    /// Get default response when AI is disabled
    fn default_response() -> AIResponse {
        AIResponse {
            is_malicious: false,
            confidence: 0.0,
            threat_type: None,
            threat_family: None,
            risk_score: 0.0,
            analysis_time_ms: 0,
        }
    }
    
    /// Enable AI integration
    pub fn enable(&mut self) {
        info!("Enabling AI integration");
        self.enabled = true;
    }
    
    /// Disable AI integration
    pub fn disable(&mut self) {
        info!("Disabling AI integration");
        self.enabled = false;
    }
    
    /// Check if AI integration is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> AIStats {
        AIStats {
            enabled: self.enabled,
            cache_size: self.cache.len(),
            api_url: self.api_url.clone(),
        }
    }
    
    /// Test connection to AI Engine
    pub fn test_connection(&self) -> Result<bool> {
        if !self.enabled {
            return Ok(false);
        }
        
        let url = format!("{}/api/v1/health", self.api_url);
        
        match self.client.get(&url).send() {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// AI integration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIStats {
    pub enabled: bool,
    pub cache_size: usize,
    pub api_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    use std::fs::File;
    
    fn create_test_config() -> Config {
        Config::default()
    }
    
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> std::path::PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }
    
    #[test]
    fn test_ai_integration_creation() {
        let config = create_test_config();
        let ai = AIIntegration::new(&config);
        assert!(ai.is_ok());
    }
    
    #[test]
    fn test_default_response() {
        let response = AIIntegration::default_response();
        assert!(!response.is_malicious);
        assert_eq!(response.confidence, 0.0);
    }
    
    #[test]
    fn test_cache_operations() {
        let config = create_test_config();
        let mut ai = AIIntegration::new(&config).unwrap();
        
        let response = AIResponse {
            is_malicious: true,
            confidence: 0.95,
            threat_type: Some("Trojan".to_string()),
            threat_family: Some("Generic".to_string()),
            risk_score: 0.9,
            analysis_time_ms: 100,
        };
        
        ai.cache_response("test_hash", response.clone());
        
        let cached = ai.get_from_cache("test_hash");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().confidence, 0.95);
    }
    
    #[test]
    fn test_clear_cache() {
        let config = create_test_config();
        let mut ai = AIIntegration::new(&config).unwrap();
        
        let response = AIIntegration::default_response();
        ai.cache_response("test1", response.clone());
        ai.cache_response("test2", response.clone());
        
        assert_eq!(ai.cache.len(), 2);
        
        ai.clear_cache();
        assert_eq!(ai.cache.len(), 0);
    }
    
    #[test]
    fn test_enable_disable() {
        let config = create_test_config();
        let mut ai = AIIntegration::new(&config).unwrap();
        
        assert!(ai.is_enabled());
        
        ai.disable();
        assert!(!ai.is_enabled());
        
        ai.enable();
        assert!(ai.is_enabled());
    }
    
    #[test]
    fn test_extract_features() {
        let config = create_test_config();
        let ai = AIIntegration::new(&config).unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_file = create_test_file(temp_dir.path(), "test.txt", b"test");
        
        let features = ai.extract_features(&test_file).unwrap();
        assert_eq!(features.len(), 30);
    }
}