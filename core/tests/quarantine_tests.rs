//! Unit tests for Quarantine module

use ghost_core::quarantine::QuarantineManager;
use std::path::Path;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_quarantine_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let manager = QuarantineManager::new(temp_dir.path());
    assert!(manager.is_ok());
}

#[test]
fn test_quarantine_file() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Create a test file
    let test_file = temp_dir.path().join("malicious.exe");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"Fake malicious content").unwrap();
    drop(file);
    
    // Quarantine the file
    let result = manager.quarantine_file(&test_file, "Test Threat");
    assert!(result.is_ok());
    
    let quarantine_id = result.unwrap();
    assert!(!quarantine_id.is_empty());
    
    // Original file should be deleted
    assert!(!test_file.exists());
    
    // File should be in quarantine
    let info = manager.get_file_info(&quarantine_id);
    assert!(info.is_some());
}

#[test]
fn test_quarantine_and_restore() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Create a test file
    let test_content = b"Test file content for restoration";
    let test_file = temp_dir.path().join("test.txt");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(test_content).unwrap();
    drop(file);
    
    let original_path = test_file.clone();
    
    // Quarantine the file
    let quarantine_id = manager.quarantine_file(&test_file, "Test").unwrap();
    
    // Restore the file
    let restored_path = manager.restore_file(&quarantine_id);
    assert!(restored_path.is_ok());
    
    let restored = restored_path.unwrap();
    assert!(restored.exists());
    
    // Verify content
    let restored_content = std::fs::read(&restored).unwrap();
    assert_eq!(restored_content, test_content);
    
    // File should no longer be in quarantine
    let info = manager.get_file_info(&quarantine_id);
    assert!(info.is_none());
}

#[test]
fn test_quarantine_and_delete() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Create a test file
    let test_file = temp_dir.path().join("delete_me.txt");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"To be deleted").unwrap();
    drop(file);
    
    // Quarantine the file
    let quarantine_id = manager.quarantine_file(&test_file, "Test").unwrap();
    
    // Delete from quarantine
    let result = manager.delete_file(&quarantine_id);
    assert!(result.is_ok());
    
    // File should no longer be in quarantine
    let info = manager.get_file_info(&quarantine_id);
    assert!(info.is_none());
}

#[test]
fn test_list_quarantined_files() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Quarantine multiple files
    for i in 0..3 {
        let test_file = temp_dir.path().join(format!("file{}.txt", i));
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Test content").unwrap();
        drop(file);
        
        manager.quarantine_file(&test_file, "Test Threat").unwrap();
    }
    
    // List files
    let files = manager.list_files();
    assert_eq!(files.len(), 3);
}

#[test]
fn test_quarantine_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Initially empty
    let stats = manager.get_statistics();
    assert_eq!(stats.total_files, 0);
    assert_eq!(stats.total_size, 0);
    
    // Add a file
    let test_file = temp_dir.path().join("stats_test.txt");
    let test_content = b"Test content for statistics";
    let mut file = File::create(&test_file).unwrap();
    file.write_all(test_content).unwrap();
    drop(file);
    
    manager.quarantine_file(&test_file, "Test").unwrap();
    
    // Check stats
    let stats = manager.get_statistics();
    assert_eq!(stats.total_files, 1);
    assert!(stats.total_size > 0);
}

#[test]
fn test_quarantine_count() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    assert_eq!(manager.get_count(), 0);
    
    // Add files
    for i in 0..5 {
        let test_file = temp_dir.path().join(format!("count{}.txt", i));
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Test").unwrap();
        drop(file);
        
        manager.quarantine_file(&test_file, "Test").unwrap();
    }
    
    assert_eq!(manager.get_count(), 5);
}

#[test]
fn test_quarantine_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    let nonexistent = temp_dir.path().join("nonexistent.txt");
    let result = manager.quarantine_file(&nonexistent, "Test");
    
    assert!(result.is_err());
}

#[test]
fn test_restore_nonexistent_quarantine() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    let result = manager.restore_file("nonexistent_id");
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent_quarantine() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    let result = manager.delete_file("nonexistent_id");
    assert!(result.is_err());
}

#[test]
fn test_clear_all_quarantine() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    // Add multiple files
    for i in 0..3 {
        let test_file = temp_dir.path().join(format!("clear{}.txt", i));
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Test").unwrap();
        drop(file);
        
        manager.quarantine_file(&test_file, "Test").unwrap();
    }
    
    assert_eq!(manager.get_count(), 3);
    
    // Clear all
    let result = manager.clear_all();
    assert!(result.is_ok());
    
    assert_eq!(manager.get_count(), 0);
}

#[test]
fn test_quarantine_file_info() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = QuarantineManager::new(temp_dir.path()).unwrap();
    
    let test_file = temp_dir.path().join("info_test.txt");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"Test content").unwrap();
    drop(file);
    
    let original_path = test_file.clone();
    let quarantine_id = manager.quarantine_file(&test_file, "Test Threat").unwrap();
    
    let info = manager.get_file_info(&quarantine_id);
    assert!(info.is_some());
    
    let entry = info.unwrap();
    assert_eq!(entry.id, quarantine_id);
    assert_eq!(entry.original_path, original_path);
    assert_eq!(entry.threat_type, "Test Threat");
    assert!(entry.file_size > 0);
    assert!(!entry.file_hash.is_empty());
}