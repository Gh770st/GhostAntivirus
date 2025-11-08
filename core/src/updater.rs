//! Update Manager Module
//! 
//! Manages automatic updates for virus definitions, engine, and configuration.

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, bail};
use log::{info, warn, debug, error};
use sha2::{Sha256, Digest};
use reqwest;

use crate::config::Config;

/// Update type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateType {
    Definitions,  // Virus definitions
    Engine,       // Core engine
    Signatures,   // Threat signatures
    Config,       // Configuration
}

/// Version information
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
    
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            bail!("Invalid version format: {}", s);
        }
        
        Ok(Self {
            major: parts[0].parse()?,
            minor: parts[1].parse()?,
            patch: parts[2].parse()?,
        })
    }
    
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Update information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub update_type: UpdateType,
    pub version: Version,
    pub download_url: String,
    pub file_size: u64,
    pub checksum: String,
    pub release_date: String,
    pub description: String,
    pub release_notes: String,
    pub critical: bool,
}

/// Update schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub enabled: bool,
    pub interval: Duration,
    pub last_check: Option<SystemTime>,
}

impl Schedule {
    pub fn new(interval: Duration) -> Self {
        Self {
            enabled: true,
            interval,
            last_check: None,
        }
    }
    
    pub fn should_check(&self) -> bool {
        if !self.enabled {
            return false;
        }
        
        match self.last_check {
            None => true,
            Some(last) => {
                if let Ok(elapsed) = SystemTime::now().duration_since(last) {
                    elapsed >= self.interval
                } else {
                    true
                }
            }
        }
    }
}

/// Update Manager
pub struct UpdateManager {
    update_url: String,
    current_version: Version,
    update_schedule: Schedule,
    update_path: PathBuf,
    client: reqwest::blocking::Client,
}

impl UpdateManager {
    /// Create new update manager
    pub fn new(config: &Config) -> Result<Self> {
        info!("Initializing Update Manager");
        
        let current_version = Version::from_string(env!("CARGO_PKG_VERSION"))?;
        let update_url = config.updater.update_url.clone();
        let update_path = PathBuf::from(&config.updater.update_path);
        
        // Create update directory
        if !update_path.exists() {
            fs::create_dir_all(&update_path)
                .context("Failed to create update directory")?;
        }
        
        let update_schedule = Schedule::new(Duration::from_secs(config.updater.check_interval));
        
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            update_url,
            current_version,
            update_schedule,
            update_path,
            client,
        })
    }
    
    /// Check for available updates
    pub fn check_updates(&mut self) -> Result<Vec<Update>> {
        info!("Checking for updates...");
        
        if !self.update_schedule.should_check() {
            debug!("Update check skipped - not time yet");
            return Ok(Vec::new());
        }
        
        // Update last check time
        self.update_schedule.last_check = Some(SystemTime::now());
        
        // Fetch update manifest
        let manifest_url = format!("{}/manifest.json", self.update_url);
        let response = self.client.get(&manifest_url)
            .send()
            .context("Failed to fetch update manifest")?;
        
        if !response.status().is_success() {
            bail!("Update server returned error: {}", response.status());
        }
        
        let updates: Vec<Update> = response.json()
            .context("Failed to parse update manifest")?;
        
        // Filter updates newer than current version
        let available_updates: Vec<Update> = updates.into_iter()
            .filter(|u| u.version > self.current_version)
            .collect();
        
        if available_updates.is_empty() {
            info!("No updates available");
        } else {
            info!("Found {} update(s)", available_updates.len());
        }
        
        Ok(available_updates)
    }
    
    /// Download an update
    pub fn download_update(&self, update: &Update) -> Result<PathBuf> {
        info!("Downloading update: {} v{}", 
              format!("{:?}", update.update_type), 
              update.version.to_string());
        
        // Create filename
        let filename = format!("{}_{}.update", 
                              format!("{:?}", update.update_type).to_lowercase(),
                              update.version.to_string());
        let download_path = self.update_path.join(&filename);
        
        // Download file
        let mut response = self.client.get(&update.download_url)
            .send()
            .context("Failed to download update")?;
        
        if !response.status().is_success() {
            bail!("Download failed: {}", response.status());
        }
        
        // Save to file
        let mut file = File::create(&download_path)
            .context("Failed to create update file")?;
        
        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 8192];
        
        loop {
            let bytes_read = response.copy_to(&mut buffer[..])
                .context("Failed to read update data")? as usize;
            
            if bytes_read == 0 {
                break;
            }
            
            file.write_all(&buffer[..bytes_read])
                .context("Failed to write update data")?;
            
            downloaded += bytes_read as u64;
            
            // Log progress
            if downloaded % (1024 * 1024) == 0 {
                debug!("Downloaded {} MB / {} MB", 
                      downloaded / (1024 * 1024),
                      update.file_size / (1024 * 1024));
            }
        }
        
        info!("Update downloaded successfully: {:?}", download_path);
        Ok(download_path)
    }
    
    /// Verify update integrity
    pub fn verify_update(&self, path: &Path, expected_checksum: &str) -> Result<bool> {
        info!("Verifying update: {:?}", path);
        
        if !path.exists() {
            bail!("Update file does not exist");
        }
        
        // Calculate checksum
        let content = fs::read(path)
            .context("Failed to read update file")?;
        
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let checksum = format!("{:x}", hasher.finalize());
        
        let valid = checksum == expected_checksum;
        
        if valid {
            info!("Update verification successful");
        } else {
            error!("Update verification failed - checksum mismatch");
            error!("Expected: {}", expected_checksum);
            error!("Got: {}", checksum);
        }
        
        Ok(valid)
    }
    
    /// Apply an update
    pub fn apply_update(&mut self, path: &Path, update_type: UpdateType) -> Result<()> {
        info!("Applying update: {:?}", path);
        
        if !path.exists() {
            bail!("Update file does not exist");
        }
        
        match update_type {
            UpdateType::Definitions => self.apply_definitions_update(path)?,
            UpdateType::Engine => self.apply_engine_update(path)?,
            UpdateType::Signatures => self.apply_signatures_update(path)?,
            UpdateType::Config => self.apply_config_update(path)?,
        }
        
        info!("Update applied successfully");
        Ok(())
    }
    
    /// Apply virus definitions update
    fn apply_definitions_update(&self, path: &Path) -> Result<()> {
        debug!("Applying definitions update");
        
        let definitions_path = self.update_path.parent()
            .context("Invalid update path")?
            .join("definitions");
        
        if !definitions_path.exists() {
            fs::create_dir_all(&definitions_path)?;
        }
        
        // Copy update file to definitions directory
        let target_path = definitions_path.join("definitions.dat");
        fs::copy(path, target_path)
            .context("Failed to copy definitions file")?;
        
        Ok(())
    }
    
    /// Apply engine update
    fn apply_engine_update(&self, path: &Path) -> Result<()> {
        debug!("Applying engine update");
        
        // In production, this would:
        // 1. Stop the current engine
        // 2. Backup current binary
        // 3. Replace with new binary
        // 4. Restart engine
        
        warn!("Engine update requires restart");
        Ok(())
    }
    
    /// Apply signatures update
    fn apply_signatures_update(&self, path: &Path) -> Result<()> {
        debug!("Applying signatures update");
        
        let signatures_path = self.update_path.parent()
            .context("Invalid update path")?
            .join("signatures");
        
        if !signatures_path.exists() {
            fs::create_dir_all(&signatures_path)?;
        }
        
        let target_path = signatures_path.join("signatures.dat");
        fs::copy(path, target_path)
            .context("Failed to copy signatures file")?;
        
        Ok(())
    }
    
    /// Apply configuration update
    fn apply_config_update(&self, path: &Path) -> Result<()> {
        debug!("Applying config update");
        
        let config_path = self.update_path.parent()
            .context("Invalid update path")?
            .join("config.toml");
        
        // Backup current config
        if config_path.exists() {
            let backup_path = config_path.with_extension("toml.backup");
            fs::copy(&config_path, backup_path)?;
        }
        
        fs::copy(path, config_path)
            .context("Failed to copy config file")?;
        
        Ok(())
    }
    
    /// Schedule automatic update checks
    pub fn schedule_check(&mut self, interval: Duration) {
        info!("Scheduling update checks every {:?}", interval);
        self.update_schedule.interval = interval;
        self.update_schedule.enabled = true;
    }
    
    /// Disable automatic updates
    pub fn disable_auto_update(&mut self) {
        info!("Disabling automatic updates");
        self.update_schedule.enabled = false;
    }
    
    /// Get current version
    pub fn get_current_version(&self) -> &Version {
        &self.current_version
    }
    
    /// Get update statistics
    pub fn get_statistics(&self) -> UpdateStats {
        UpdateStats {
            current_version: self.current_version.clone(),
            last_check: self.update_schedule.last_check,
            auto_update_enabled: self.update_schedule.enabled,
            check_interval: self.update_schedule.interval,
        }
    }
    
    /// Check for updates (async version for API compatibility)
    pub async fn check_for_updates(&self) -> Result<Option<Update>> {
        info!("Checking for updates asynchronously");
        
        // In a real implementation, this would:
        // 1. Query update server
        // 2. Compare versions
        // 3. Return update info if available
        
        // For now, return None (no updates available)
        Ok(None)
    }
    
    /// Apply update (async version for API compatibility)
    pub async fn apply_update(&mut self, version: &str, auto_restart: bool) -> Result<()> {
        info!("Applying update to version: {}", version);
        
        // In a real implementation, this would:
        // 1. Download update package
        // 2. Verify checksum
        // 3. Apply update
        // 4. Optionally restart service
        
        // For now, just log the action
        if auto_restart {
            info!("Auto-restart requested after update");
        }
        
        Ok(())
    }
}

/// Update statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStats {
    pub current_version: Version,
    pub last_check: Option<SystemTime>,
    pub auto_update_enabled: bool,
    pub check_interval: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_parsing() {
        let version = Version::from_string("3.0.1").unwrap();
        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 1);
    }
    
    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(3, 0, 0);
        let v2 = Version::new(3, 0, 1);
        let v3 = Version::new(3, 1, 0);
        
        assert!(v2 > v1);
        assert!(v3 > v2);
        assert!(v3 > v1);
    }
    
    #[test]
    fn test_version_to_string() {
        let version = Version::new(3, 0, 1);
        assert_eq!(version.to_string(), "3.0.1");
    }
    
    #[test]
    fn test_schedule_should_check() {
        let mut schedule = Schedule::new(Duration::from_secs(60));
        
        // Should check on first run
        assert!(schedule.should_check());
        
        // Update last check
        schedule.last_check = Some(SystemTime::now());
        
        // Should not check immediately
        assert!(!schedule.should_check());
    }
    
    #[test]
    fn test_schedule_disabled() {
        let mut schedule = Schedule::new(Duration::from_secs(60));
        schedule.enabled = false;
        
        assert!(!schedule.should_check());
    }
}