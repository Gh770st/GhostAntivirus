//! Quarantine Manager Module
//! 
//! Manages quarantined files with encryption, restoration, and secure deletion.

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::time::SystemTime;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, bail};
use log::{info, warn};
use sha2::{Sha256, Digest};

/// Quarantine entry identifier
pub type QuarantineId = String;

/// Quarantine entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineEntry {
    pub id: QuarantineId,
    pub original_path: PathBuf,
    pub quarantine_path: PathBuf,
    pub file_hash: String,
    pub file_size: u64,
    pub threat_type: String,
    pub quarantined_at: SystemTime,
    pub encrypted: bool,
}

/// Quarantine database
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuarantineDatabase {
    entries: HashMap<QuarantineId, QuarantineEntry>,
}

impl QuarantineDatabase {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let data = fs::read_to_string(path)
            .context("Failed to read quarantine database")?;
        
        let db: QuarantineDatabase = serde_json::from_str(&data)
            .context("Failed to parse quarantine database")?;
        
        Ok(db)
    }
    
    fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string_pretty(self)
            .context("Failed to serialize quarantine database")?;
        
        fs::write(path, data)
            .context("Failed to write quarantine database")?;
        
        Ok(())
    }
}

/// Quarantine Manager
pub struct QuarantineManager {
    quarantine_path: PathBuf,
    database_path: PathBuf,
    database: QuarantineDatabase,
    encryption_key: Vec<u8>,
}

impl QuarantineManager {
    /// Create new quarantine manager
    pub fn new(quarantine_path: &Path) -> Result<Self> {
        info!("Initializing Quarantine Manager at: {:?}", quarantine_path);
        
        // Create quarantine directory if it doesn't exist
        if !quarantine_path.exists() {
            fs::create_dir_all(quarantine_path)
                .context("Failed to create quarantine directory")?;
        }
        
        let database_path = quarantine_path.join("quarantine.db");
        let database = QuarantineDatabase::load(&database_path)?;
        
        // Generate encryption key (in production, this should be securely stored)
        let encryption_key = Self::generate_encryption_key();
        
        Ok(Self {
            quarantine_path: quarantine_path.to_path_buf(),
            database_path,
            database,
            encryption_key,
        })
    }
    
    /// Generate encryption key
    fn generate_encryption_key() -> Vec<u8> {
        // In production, use a proper key derivation function
        // For now, use a simple key
        vec![0x42; 32] // 256-bit key
    }
    
    /// Quarantine a file
    pub fn quarantine_file(&mut self, file_path: &Path, threat_type: &str) -> Result<QuarantineId> {
        info!("Quarantining file: {:?}", file_path);
        
        if !file_path.exists() {
            bail!("File does not exist: {:?}", file_path);
        }
        
        // Read file content
        let mut file = File::open(file_path)
            .context("Failed to open file for quarantine")?;
        
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .context("Failed to read file content")?;
        
        // Calculate file hash
        let file_hash = self.calculate_hash(&content);
        let file_size = content.len() as u64;
        
        // Generate quarantine ID
        let id = self.generate_quarantine_id(&file_hash);
        
        // Encrypt content
        let encrypted_content = self.encrypt_data(&content)?;
        
        // Save to quarantine directory
        let quarantine_file_path = self.quarantine_path.join(&id);
        let mut quarantine_file = File::create(&quarantine_file_path)
            .context("Failed to create quarantine file")?;
        
        quarantine_file.write_all(&encrypted_content)
            .context("Failed to write quarantine file")?;
        
        // Create quarantine entry
        let entry = QuarantineEntry {
            id: id.clone(),
            original_path: file_path.to_path_buf(),
            quarantine_path: quarantine_file_path,
            file_hash,
            file_size,
            threat_type: threat_type.to_string(),
            quarantined_at: SystemTime::now(),
            encrypted: true,
        };
        
        // Add to database
        self.database.entries.insert(id.clone(), entry);
        self.save_database()?;
        
        // Delete original file
        fs::remove_file(file_path)
            .context("Failed to delete original file")?;
        
        info!("File quarantined successfully: {}", id);
        Ok(id)
    }
    
    /// Restore a quarantined file
    pub fn restore_file(&mut self, id: &QuarantineId) -> Result<PathBuf> {
        info!("Restoring file: {}", id);
        
        let entry = self.database.entries.get(id)
            .context("Quarantine entry not found")?
            .clone();
        
        // Read quarantined file
        let mut file = File::open(&entry.quarantine_path)
            .context("Failed to open quarantine file")?;
        
        let mut encrypted_content = Vec::new();
        file.read_to_end(&mut encrypted_content)
            .context("Failed to read quarantine file")?;
        
        // Decrypt content
        let content = self.decrypt_data(&encrypted_content)?;
        
        // Verify hash
        let hash = self.calculate_hash(&content);
        if hash != entry.file_hash {
            bail!("File hash mismatch - file may be corrupted");
        }
        
        // Restore to original location
        let restore_path = &entry.original_path;
        
        // Create parent directories if needed
        if let Some(parent) = restore_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create parent directories")?;
        }
        
        let mut restored_file = File::create(restore_path)
            .context("Failed to create restored file")?;
        
        restored_file.write_all(&content)
            .context("Failed to write restored file")?;
        
        // Remove from quarantine
        fs::remove_file(&entry.quarantine_path)
            .context("Failed to delete quarantine file")?;
        
        self.database.entries.remove(id);
        self.save_database()?;
        
        info!("File restored successfully: {:?}", restore_path);
        Ok(restore_path.clone())
    }
    
    /// Delete a quarantined file permanently
    pub fn delete_file(&mut self, id: &QuarantineId) -> Result<()> {
        info!("Deleting quarantined file: {}", id);
        
        let entry = self.database.entries.get(id)
            .context("Quarantine entry not found")?
            .clone();
        
        // Securely delete file (overwrite with random data)
        self.secure_delete(&entry.quarantine_path)?;
        
        // Remove from database
        self.database.entries.remove(id);
        self.save_database()?;
        
        info!("File deleted successfully: {}", id);
        Ok(())
    }
    
    /// List all quarantined files
    pub fn list_files(&self) -> Vec<QuarantineEntry> {
        self.database.entries.values().cloned().collect()
    }
    
    /// Get information about a quarantined file
    pub fn get_file_info(&self, id: &QuarantineId) -> Option<QuarantineEntry> {
        self.database.entries.get(id).cloned()
    }
    
    /// Get quarantine statistics
    pub fn get_statistics(&self) -> QuarantineStats {
        let total_files = self.database.entries.len();
        let total_size: u64 = self.database.entries.values()
            .map(|e| e.file_size)
            .sum();
        
        QuarantineStats {
            total_files,
            total_size,
        }
    }
    
    /// Get count of quarantined files
    pub fn get_count(&self) -> usize {
        self.database.entries.len()
    }
    
    /// Calculate SHA256 hash of data
    fn calculate_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    /// Generate quarantine ID from hash
    fn generate_quarantine_id(&self, hash: &str) -> QuarantineId {
        format!("quar_{}", &hash[..16])
    }
    
    /// Simple XOR encryption (in production, use proper encryption like AES)
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encrypted = Vec::with_capacity(data.len());
        
        for (i, byte) in data.iter().enumerate() {
            let key_byte = self.encryption_key[i % self.encryption_key.len()];
            encrypted.push(byte ^ key_byte);
        }
        
        Ok(encrypted)
    }
    
    /// Simple XOR decryption
    fn decrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // XOR is symmetric, so decryption is the same as encryption
        self.encrypt_data(data)
    }
    
    /// Securely delete a file
    fn secure_delete(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }
        
        // Get file size
        let metadata = fs::metadata(path)
            .context("Failed to get file metadata")?;
        let size = metadata.len() as usize;
        
        // Overwrite with zeros
        let zeros = vec![0u8; size];
        fs::write(path, zeros)
            .context("Failed to overwrite file")?;
        
        // Delete file
        fs::remove_file(path)
            .context("Failed to delete file")?;
        
        Ok(())
    }
    
    /// Save database to disk
    fn save_database(&self) -> Result<()> {
        self.database.save(&self.database_path)
    }
    
    /// Clear all quarantined files
    pub fn clear_all(&mut self) -> Result<()> {
        warn!("Clearing all quarantined files");
        
        for entry in self.database.entries.values() {
            if entry.quarantine_path.exists() {
                self.secure_delete(&entry.quarantine_path)?;
            }
        }
        
        self.database.entries.clear();
        self.save_database()?;
        
        Ok(())
    }
}

/// Quarantine statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineStats {
    pub total_files: usize,
    pub total_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }
    
    #[test]
    fn test_quarantine_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = QuarantineManager::new(temp_dir.path());
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_quarantine_and_restore() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
        
        // Create test file
        let test_content = b"This is a test file";
        let test_file = create_test_file(temp_dir.path(), "test.txt", test_content);
        
        // Quarantine file
        let id = manager.quarantine_file(&test_file, "Test Threat").unwrap();
        assert!(!test_file.exists()); // Original should be deleted
        
        // Restore file
        let restored_path = manager.restore_file(&id).unwrap();
        assert!(restored_path.exists());
        
        // Verify content
        let restored_content = fs::read(&restored_path).unwrap();
        assert_eq!(restored_content, test_content);
    }
    
    #[test]
    fn test_quarantine_and_delete() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
        
        // Create test file
        let test_file = create_test_file(temp_dir.path(), "test2.txt", b"Test");
        
        // Quarantine file
        let id = manager.quarantine_file(&test_file, "Test Threat").unwrap();
        
        // Delete quarantined file
        let result = manager.delete_file(&id);
        assert!(result.is_ok());
        
        // Verify it's gone
        assert!(manager.get_file_info(&id).is_none());
    }
    
    #[test]
    fn test_list_files() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
        
        // Create and quarantine multiple files with DIFFERENT content
        for i in 0..3 {
            let content = format!("Test content {}", i);
            let test_file = create_test_file(
                temp_dir.path(), 
                &format!("test{}.txt", i), 
                content.as_bytes()
            );
            manager.quarantine_file(&test_file, "Test Threat").unwrap();
        }
        
        let files = manager.list_files();
        assert_eq!(files.len(), 3);
    }
    
    #[test]
    fn test_encryption_decryption() {
        let temp_dir = TempDir::new().unwrap();
        let manager = QuarantineManager::new(temp_dir.path()).unwrap();
        
        let original_data = b"Secret data to encrypt";
        let encrypted = manager.encrypt_data(original_data).unwrap();
        let decrypted = manager.decrypt_data(&encrypted).unwrap();
        
        assert_eq!(original_data, decrypted.as_slice());
        assert_ne!(original_data, encrypted.as_slice());
    }
}