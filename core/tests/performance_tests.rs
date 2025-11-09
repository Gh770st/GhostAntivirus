//! Performance Tests
//! 
//! Comprehensive performance testing for GhostAntivirus core components

use ghost_core::{scanner::Scanner, config::Config, utils};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::fs;

/// Performance test configuration
struct PerfConfig {
    test_file_count: usize,
    test_file_size: usize,
    scan_iterations: usize,
}

impl Default for PerfConfig {
    fn default() -> Self {
        Self {
            test_file_count: 100,
            test_file_size: 1024, // 1KB
            scan_iterations: 10,
        }
    }
}

/// Create test files for performance testing
fn create_test_files(config: &PerfConfig) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let test_dir = PathBuf::from("./perf_test_files");
    fs::create_dir_all(&test_dir)?;
    
    let mut files = Vec::new();
    
    for i in 0..config.test_file_count {
        let file_path = test_dir.join(format!("test_file_{}.txt", i));
        let content = format!("Test file {} content\n", i).repeat(config.test_file_size / 20);
        fs::write(&file_path, content)?;
        files.push(file_path);
    }
    
    Ok(files)
}

/// Clean up test files
fn cleanup_test_files() {
    let _ = fs::remove_dir_all("./perf_test_files");
    let _ = fs::remove_dir_all("./test_quarantine_perf");
}

#[tokio::test]
async fn test_scanner_performance() {
    println!("🚀 Starting Scanner Performance Test...");
    
    let config = PerfConfig::default();
    let scanner = Scanner::new(Config::default()).unwrap();
    
    // Create test files
    let test_files = create_test_files(&config).expect("Failed to create test files");
    println!("Created {} test files", test_files.len());
    
    // Measure quick scan performance
    let start_time = Instant::now();
    let result = scanner.quick_scan().await.unwrap();
    let scan_duration = start_time.elapsed();
    
    println!("📊 Quick Scan Results:");
    println!("  - Files scanned: {}", result.files_scanned);
    println!("  - Threats found: {:?}", result.threats_found);
    println!("  - Scan duration: {:?}", scan_duration);
    println!("  - Files per second: {:.2}", result.files_scanned as f64 / scan_duration.as_secs_f64());
    
    // Performance assertions
    assert!(scan_duration < Duration::from_secs(30), "Scan should complete within 30 seconds");
    assert!(result.files_scanned > 0, "Should scan at least some files");
    
    // Cleanup
    cleanup_test_files();
    
    println!("✅ Scanner Performance Test Completed");
}

#[tokio::test]
async fn test_memory_usage() {
    println!("🧠 Starting Memory Usage Test...");
    
    let initial_memory = get_memory_usage();
    println!("Initial memory: {} KB", initial_memory);
    
    // Create multiple scanners
    let mut scanners = Vec::new();
    for _ in 0..10 {
        let scanner = Scanner::new(Config::default()).unwrap();
        scanners.push(scanner);
    }
    
    let peak_memory = get_memory_usage();
    println!("Peak memory: {} KB", peak_memory);
    println!("Memory increase: {} KB", peak_memory - initial_memory);
    
    // Memory usage should be reasonable (< 100MB increase)
    assert!(peak_memory - initial_memory < 100_000, "Memory usage should be reasonable");
    
    // Cleanup
    drop(scanners);
    let final_memory = get_memory_usage();
    println!("Final memory: {} KB", final_memory);
    println!("Memory reclaimed: {} KB", peak_memory - final_memory);
    
    println!("✅ Memory Usage Test Completed");
}

#[tokio::test]
async fn test_concurrent_scans() {
    println!("⚡ Starting Concurrent Scans Test...");
    
    let config = Config::default();
    let test_files = create_test_files(&PerfConfig::default()).expect("Failed to create test files");
    
    // Create multiple concurrent scans
    let mut handles = Vec::new();
    let start_time = Instant::now();
    
    for i in 0..5 {
        let config_clone = config.clone();
        let handle = tokio::spawn(async move {
            let scanner = Scanner::new(config_clone).unwrap();
            let result = scanner.quick_scan().await.unwrap();
            println!("Concurrent scan {} completed: {} files", i + 1, result.files_scanned);
            result
        });
        handles.push(handle);
    }
    
    // Wait for all scans to complete
    let results = futures::future::join_all(handles).await;
    let total_duration = start_time.elapsed();
    
    println!("📊 Concurrent Scan Results:");
    println!("  - Total scans: {}", results.len());
    println!("  - Total duration: {:?}", total_duration);
    println!("  - Average duration per scan: {:?}", total_duration / results.len() as u32);
    
    // Verify all scans completed successfully
    for result in results {
        assert!(result.is_ok(), "All concurrent scans should succeed");
    }
    
    cleanup_test_files();
    println!("✅ Concurrent Scans Test Completed");
}

#[test]
fn test_hash_calculation_performance() {
    println!("🔢 Starting Hash Calculation Performance Test...");
    
    let test_file = PathBuf::from("./perf_hash_test.txt");
    let content = "Performance test content\n".repeat(10000); // ~260KB file
    fs::write(&test_file, content).unwrap();
    
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        let _hash = utils::calculate_file_hash(&test_file).unwrap();
    }
    
    let duration = start_time.elapsed();
    let hashes_per_second = iterations as f64 / duration.as_secs_f64();
    
    println!("📊 Hash Calculation Results:");
    println!("  - Iterations: {}", iterations);
    println!("  - Total duration: {:?}", duration);
    println!("  - Hashes per second: {:.2}", hashes_per_second);
    println!("  - Time per hash: {:.2} ms", duration.as_millis() as f64 / iterations as f64);
    
    // Performance assertions
    assert!(hashes_per_second > 100.0, "Should calculate at least 100 hashes per second");
    
    // Cleanup
    let _ = fs::remove_file(&test_file);
    
    println!("✅ Hash Calculation Performance Test Completed");
}

#[tokio::test]
async fn test_large_file_handling() {
    println!("📁 Starting Large File Handling Test...");
    
    let large_file = PathBuf::from("./perf_large_test.txt");
    let large_content = "Large file test content\n".repeat(1_000_000); // ~20MB file
    fs::write(&large_file, large_content).unwrap();
    
    let scanner = Scanner::new(Config::default()).unwrap();
    let start_time = Instant::now();
    
    // Test scanning a single large file
    let result = scanner.scan_path("./perf_large_test.txt").await.unwrap();
    let duration = start_time.elapsed();
    
    println!("📊 Large File Scan Results:");
    println!("  - File size: ~20MB");
    println!("  - Scan duration: {:?}", duration);
    println!("  - Files scanned: {}", result.files_scanned);
    println!("  - Throughput: {:.2} MB/s", 20.0 / duration.as_secs_f64());
    
    // Performance assertions
    assert!(duration < Duration::from_secs(10), "Large file scan should complete within 10 seconds");
    
    // Cleanup
    let _ = fs::remove_file(&large_file);
    
    println!("✅ Large File Handling Test Completed");
}

#[test]
fn test_startup_time() {
    println!("🚀 Starting Startup Time Test...");
    
    let iterations = 50;
    let mut total_time = Duration::ZERO;
    
    for _ in 0..iterations {
        let start_time = Instant::now();
        let scanner = Scanner::new(Config::default()).unwrap();
        let _ = scanner.get_stats(); // Initialize
        let duration = start_time.elapsed();
        total_time += duration;
    }
    
    let avg_time = total_time / iterations as u32;
    
    println!("📊 Startup Time Results:");
    println!("  - Iterations: {}", iterations);
    println!("  - Average startup time: {:?}", avg_time);
    println!("  - Total time: {:?}", total_time);
    
    // Performance assertions
    assert!(avg_time < Duration::from_millis(100), "Startup should be fast (< 100ms)");
    
    println!("✅ Startup Time Test Completed");
}

/// Helper function to get current memory usage in KB
fn get_memory_usage() -> u64 {
    // This is a simplified memory usage check
    // In a real implementation, you'd use platform-specific APIs
    use std::fs;
    
    match fs::read_to_string("/proc/self/status") {
        Ok(content) => {
            for line in content.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].parse::<u64>().unwrap_or(0);
                    }
                }
            }
        }
        Err(_) => {}
    }
    
    0 // Fallback
}

#[tokio::test]
async fn test_performance_regression() {
    println!("📈 Starting Performance Regression Test...");
    
    let config = PerfConfig {
        test_file_count: 50,
        test_file_size: 2048,
        scan_iterations: 5,
    };
    
    let test_files = create_test_files(&config).expect("Failed to create test files");
    
    // Run multiple iterations to establish baseline
    let mut scan_times = Vec::new();
    
    for _ in 0..config.scan_iterations {
        let scanner = Scanner::new(Config::default()).unwrap();
        let start_time = Instant::now();
        let result = scanner.quick_scan().await.unwrap();
        let duration = start_time.elapsed();
        
        scan_times.push(duration);
        assert!(result.files_scanned > 0, "Should scan files");
    }
    
    let avg_scan_time = scan_times.iter().sum::<Duration>() / scan_times.len() as u32;
    let max_scan_time = scan_times.iter().max().unwrap();
    let min_scan_time = scan_times.iter().min().unwrap();
    
    println!("📊 Performance Regression Results:");
    println!("  - Files created: {}", test_files.len());
    println!("  - Scan iterations: {}", config.scan_iterations);
    println!("  - Average scan time: {:?}", avg_scan_time);
    println!("  - Min scan time: {:?}", min_scan_time);
    println!("  - Max scan time: {:?}", max_scan_time);
    let variance = max_scan_time.saturating_sub(*min_scan_time);
    println!("  - Variance: {:?}", variance);
    assert!(variance < avg_scan_time / 2, "Scan times should be consistent");
    
    cleanup_test_files();
    println!("✅ Performance Regression Test Completed");
}