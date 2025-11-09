//! Cryptography Module
//! 
//! Provides encryption, decryption, hashing, and key management utilities.

use std::path::Path;
use std::fs::{self, File};
use std::io::Read;
use anyhow::{Result, Context, bail};
use log::{info, debug};
use sha2::{Sha256, Digest};

/// Encryption algorithm
#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    AES256,
    ChaCha20,
    XOR,
}

/// Encryption key
pub struct EncryptionKey {
    key: Vec<u8>,
    algorithm: Algorithm,
}

impl EncryptionKey {
    /// Create new encryption key
    pub fn new(algorithm: Algorithm) -> Self {
        let key = Self::generate_key(&algorithm);
        Self { key, algorithm }
    }
    
    /// Generate key for algorithm
    fn generate_key(algorithm: &Algorithm) -> Vec<u8> {
        match algorithm {
            Algorithm::AES256 => vec![0x42; 32], // 256-bit key
            Algorithm::ChaCha20 => vec![0x42; 32], // 256-bit key
            Algorithm::XOR => vec![0x42; 32],
        }
    }
    
    /// Get key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
    
    /// Get algorithm
    pub fn algorithm(&self) -> &Algorithm {
        &self.algorithm
    }
}

/// Encrypt a file
pub fn encrypt_file(path: &Path, key: &EncryptionKey) -> Result<Vec<u8>> {
    info!("Encrypting file: {:?}", path);
    
    if !path.exists() {
        bail!("File does not exist: {:?}", path);
    }
    
    // Read file content
    let mut file = File::open(path)
        .context("Failed to open file for encryption")?;
    
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .context("Failed to read file content")?;
    
    // Encrypt based on algorithm
    let encrypted = match key.algorithm() {
        Algorithm::AES256 => encrypt_aes256(&content, key.as_bytes())?,
        Algorithm::ChaCha20 => encrypt_chacha20(&content, key.as_bytes())?,
        Algorithm::XOR => encrypt_xor(&content, key.as_bytes()),
    };
    
    debug!("File encrypted successfully: {} bytes", encrypted.len());
    Ok(encrypted)
}

/// Decrypt data
pub fn decrypt_file(data: &[u8], key: &EncryptionKey) -> Result<Vec<u8>> {
    debug!("Decrypting data: {} bytes", data.len());
    
    // Decrypt based on algorithm
    let decrypted = match key.algorithm() {
        Algorithm::AES256 => decrypt_aes256(data, key.as_bytes())?,
        Algorithm::ChaCha20 => decrypt_chacha20(data, key.as_bytes())?,
        Algorithm::XOR => encrypt_xor(data, key.as_bytes()), // XOR is symmetric
    };
    
    debug!("Data decrypted successfully: {} bytes", decrypted.len());
    Ok(decrypted)
}

/// AES-256 encryption (simplified - in production use proper AES library)
fn encrypt_aes256(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // In production, use a proper AES implementation like `aes-gcm` crate
    // For now, use XOR as placeholder
    Ok(encrypt_xor(data, key))
}

/// AES-256 decryption
fn decrypt_aes256(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // In production, use a proper AES implementation
    Ok(encrypt_xor(data, key))
}

/// ChaCha20 encryption (simplified)
fn encrypt_chacha20(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // In production, use `chacha20poly1305` crate
    Ok(encrypt_xor(data, key))
}

/// ChaCha20 decryption
fn decrypt_chacha20(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    Ok(encrypt_xor(data, key))
}

/// Simple XOR encryption (for demonstration)
fn encrypt_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, byte)| byte ^ key[i % key.len()])
        .collect()
}

/// Generate random key
pub fn generate_key(size: usize) -> Vec<u8> {
    // In production, use a cryptographically secure random generator
    // For now, use a simple pattern
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Calculate SHA256 hash
pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Calculate file hash
pub fn calculate_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)
        .context("Failed to read file for hashing")?;
    
    Ok(calculate_sha256(&content))
}

/// Hash password using SHA256 (in production, use bcrypt or argon2)
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    let password_hash = hash_password(password);
    password_hash == hash
}

/// Secure wipe of data in memory
pub fn secure_wipe(data: &mut [u8]) {
    // Overwrite with zeros
    for byte in data.iter_mut() {
        *byte = 0;
    }
}

/// Generate random nonce
pub fn generate_nonce(size: usize) -> Vec<u8> {
    // In production, use a cryptographically secure random generator
    (0..size).map(|i| ((i * 7) % 256) as u8).collect()
}

/// Derive key from password (simplified PBKDF2)
pub fn derive_key_from_password(password: &str, salt: &[u8], iterations: u32) -> Vec<u8> {
    // In production, use proper PBKDF2 or Argon2
    let mut key = password.as_bytes().to_vec();
    
    for _ in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(&key);
        hasher.update(salt);
        key = hasher.finalize().to_vec();
    }
    
    key
}

/// Constant-time comparison (prevents timing attacks)
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> std::path::PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }
    
    #[test]
    fn test_encryption_key_creation() {
        let key = EncryptionKey::new(Algorithm::AES256);
        assert_eq!(key.as_bytes().len(), 32);
        assert_eq!(key.algorithm(), &Algorithm::AES256);
    }
    
    #[test]
    fn test_xor_encryption_decryption() {
        let data = b"Hello, World!";
        let key = b"secret_key_12345678901234567890";
        
        let encrypted = encrypt_xor(data, key);
        assert_ne!(encrypted, data);
        
        let decrypted = encrypt_xor(&encrypted, key);
        assert_eq!(decrypted, data);
    }
    
    #[test]
    fn test_file_encryption_decryption() {
        let temp_dir = TempDir::new().unwrap();
        let test_content = b"Secret data to encrypt";
        let test_file = create_test_file(temp_dir.path(), "test.txt", test_content);
        
        let key = EncryptionKey::new(Algorithm::XOR);
        
        let encrypted = encrypt_file(&test_file, &key).unwrap();
        assert_ne!(encrypted, test_content);
        
        let decrypted = decrypt_file(&encrypted, &key).unwrap();
        assert_eq!(decrypted, test_content);
    }
    
    #[test]
    fn test_calculate_sha256() {
        let data = b"test data";
        let hash = calculate_sha256(data);
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex characters
    }
    
    #[test]
    fn test_password_hashing() {
        let password = "my_secure_password";
        let hash = hash_password(password);
        
        assert_eq!(hash.len(), 64);
        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong_password", &hash));
    }
    
    #[test]
    fn test_generate_key() {
        let key = generate_key(32);
        assert_eq!(key.len(), 32);
    }
    
    #[test]
    fn test_generate_nonce() {
        let nonce = generate_nonce(12);
        assert_eq!(nonce.len(), 12);
    }
    
    #[test]
    fn test_derive_key_from_password() {
        let password = "my_password";
        let salt = b"random_salt_1234";
        
        let key1 = derive_key_from_password(password, salt, 1000);
        let key2 = derive_key_from_password(password, salt, 1000);
        
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }
    
    #[test]
    fn test_constant_time_compare() {
        let a = b"secret_data";
        let b = b"secret_data";
        let c = b"other_data!";
        
        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
    }
    
    #[test]
    fn test_secure_wipe() {
        let mut data = vec![1u8, 2, 3, 4, 5];
        secure_wipe(&mut data);
        
        assert!(data.iter().all(|&b| b == 0));
    }
}