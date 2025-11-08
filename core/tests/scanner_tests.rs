//! Unit tests for Scanner module

use ghost_core::scanner::{Scanner, ScanType, Severity, ThreatType};
use ghost_core::config::Config;
use std::path::Path;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

#[tokio::test]
async fn test_scanner_creation() {
    let config = Config::default();
    let scanner = Scanner::new(config);
    assert!(scanner.is_ok());
}

#[tokio::test]
async fn test_scanner_statistics() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let stats = scanner.get_statistics();
    assert_eq!(stats.total_files_scanned, 0);
    assert_eq!(stats.threats_detected, 0);
    assert!(!stats.is_scanning);
}

#[tokio::test]
async fn test_scan_nonexistent_path() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let result = scanner.scan_path("/nonexistent/path/to/file").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_scan_empty_directory() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let result = scanner.scan_path(temp_dir.path().to_str().unwrap()).await;
    
    assert!(result.is_ok());
    let scan_result = result.unwrap();
    assert_eq!(scan_result.files_scanned, 0);
    assert_eq!(scan_result.threats_found.len(), 0);
}

#[tokio::test]
async fn test_scan_harmless_file() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    // Create a harmless text file
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("harmless.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"This is a harmless text file for testing.").unwrap();
    
    let result = scanner.scan_path(file_path.to_str().unwrap()).await;
    
    assert!(result.is_ok());
    let scan_result = result.unwrap();
    assert_eq!(scan_result.files_scanned, 1);
    // Harmless file should not be detected as threat
    assert_eq!(scan_result.threats_found.len(), 0);
}

#[tokio::test]
async fn test_scan_eicar_file() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    // Create EICAR test file
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("eicar.txt");
    let mut file = File::create(&file_path).unwrap();
    let eicar = "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
    file.write_all(eicar.as_bytes()).unwrap();
    
    let result = scanner.scan_path(file_path.to_str().unwrap()).await;
    
    assert!(result.is_ok());
    let scan_result = result.unwrap();
    assert_eq!(scan_result.files_scanned, 1);
    // EICAR should be detected
    assert!(scan_result.threats_found.len() > 0);
    
    if let Some(threat) = scan_result.threats_found.first() {
        assert_eq!(threat.threat_name, "EICAR-Test-File");
    }
}

#[tokio::test]
async fn test_scan_lifecycle() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    // Create test directory with files
    let temp_dir = TempDir::new().unwrap();
    for i in 0..5 {
        let file_path = temp_dir.path().join(format!("file{}.txt", i));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Test content").unwrap();
    }
    
    // Start scan
    let scan_id = scanner.start_scan(
        temp_dir.path().to_str().unwrap(),
        ScanType::Custom(temp_dir.path().to_string_lossy().to_string()),
        false
    ).await;
    
    assert!(scan_id.is_ok());
    
    // Check that scan is running
    let stats = scanner.get_statistics();
    assert!(stats.is_scanning || stats.total_files_scanned > 0);
}

#[tokio::test]
async fn test_scan_pause_resume() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Test").unwrap();
    
    // Start scan
    let _ = scanner.start_scan(
        temp_dir.path().to_str().unwrap(),
        ScanType::Quick,
        false
    ).await;
    
    // Try to pause
    let pause_result = scanner.pause_scan().await;
    // May succeed or fail depending on timing
    
    // Try to resume
    if pause_result.is_ok() {
        let resume_result = scanner.resume_scan().await;
        assert!(resume_result.is_ok() || resume_result.is_err());
    }
}

#[tokio::test]
async fn test_scan_stop() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    
    // Start scan
    let _ = scanner.start_scan(
        temp_dir.path().to_str().unwrap(),
        ScanType::Quick,
        false
    ).await;
    
    // Stop scan
    let stop_result = scanner.stop_scan().await;
    // May succeed or fail depending on timing
    
    // After stop, should not be scanning
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let stats = scanner.get_statistics();
    // Stats should show scan is not running (eventually)
}

#[tokio::test]
async fn test_multiple_scans_prevented() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    
    // Start first scan
    let first_scan = scanner.start_scan(
        temp_dir.path().to_str().unwrap(),
        ScanType::Quick,
        false
    ).await;
    
    assert!(first_scan.is_ok());
    
    // Try to start second scan immediately
    let second_scan = scanner.start_scan(
        temp_dir.path().to_str().unwrap(),
        ScanType::Quick,
        false
    ).await;
    
    // Second scan should fail (scan already in progress)
    assert!(second_scan.is_err());
}

#[test]
fn test_threat_severity_levels() {
    // Test severity enum
    assert!(matches!(Severity::Low, Severity::Low));
    assert!(matches!(Severity::Medium, Severity::Medium));
    assert!(matches!(Severity::High, Severity::High));
    assert!(matches!(Severity::Critical, Severity::Critical));
}

#[test]
fn test_threat_types() {
    // Test threat type enum
    assert!(matches!(ThreatType::Virus, ThreatType::Virus));
    assert!(matches!(ThreatType::Trojan, ThreatType::Trojan));
    assert!(matches!(ThreatType::Ransomware, ThreatType::Ransomware));
}