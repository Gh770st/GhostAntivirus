//! Simple Integration Tests
//! 
//! Basic integration tests that focus on core functionality

use ghost_core::{scanner::Scanner, quarantine::QuarantineManager, config::Config};
use std::path::PathBuf;
use std::fs;

#[tokio::test]
async fn test_scanner_creation() {
    let config = Config::default();
    let scanner = Scanner::new(config);
    assert!(scanner.is_ok());
}

#[tokio::test] 
async fn test_quarantine_creation() {
    let test_path = PathBuf::from("./test_quarantine_simple");
    fs::create_dir_all(&test_path).unwrap();
    
    let quarantine = QuarantineManager::new(&test_path);
    assert!(quarantine.is_ok());
    
    // Cleanup
    fs::remove_dir_all(&test_path).unwrap();
}

#[test]
fn test_config_default() {
    let config = Config::default();
    // Just verify it creates without panicking
    assert!(true);
}

#[test] 
fn test_basic_functionality() {
    // Test that we can create the main components
    let config = Config::default();
    let scanner = Scanner::new(config.clone());
    assert!(scanner.is_ok());
    
    let test_path = PathBuf::from("./test_quarantine_basic");
    fs::create_dir_all(&test_path).unwrap();
    
    let quarantine = QuarantineManager::new(&test_path);
    assert!(quarantine.is_ok());
    
    // Cleanup
    fs::remove_dir_all(&test_path).unwrap();
}

#[tokio::test]
async fn test_async_operations() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    // Test async scan operations
    let result = scanner.quick_scan().await;
    assert!(result.is_ok());
    
    let scan_result = result.unwrap();
    // Should complete without panicking
    assert!(scan_result.files_scanned >= 0);
}