//! Scanner Performance Benchmarks
//! 
//! Comprehensive benchmark suite for the scanner module.
//! Measures scan performance, throughput, and resource usage.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ghost_antivirus::scanner::Scanner;
use std::path::PathBuf;
use std::fs;
use tokio::runtime::Runtime;

/// Benchmark: Scanner initialization
fn bench_scanner_init(c: &mut Criterion) {
    c.bench_function("scanner_init", |b| {
        b.iter(|| {
            Scanner::new().unwrap()
        });
    });
}

/// Benchmark: Single file scan
fn bench_single_file_scan(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    // Create test file
    let test_file = PathBuf::from("/tmp/bench_test.txt");
    fs::write(&test_file, "Test content for benchmarking").unwrap();
    
    c.bench_function("single_file_scan", |b| {
        b.to_async(&rt).iter(|| async {
            scanner.scan_file(black_box(&test_file)).await.unwrap()
        });
    });
    
    // Cleanup
    let _ = fs::remove_file(&test_file);
}

/// Benchmark: Multiple file scans
fn bench_multiple_file_scans(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    // Create test directory with files
    let test_dir = PathBuf::from("/tmp/bench_multi");
    fs::create_dir_all(&test_dir).unwrap();
    
    for i in 0..100 {
        let file_path = test_dir.join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("Content {}", i)).unwrap();
    }
    
    c.bench_function("scan_100_files", |b| {
        b.to_async(&rt).iter(|| async {
            for i in 0..100 {
                let file_path = test_dir.join(format!("file_{}.txt", i));
                scanner.scan_file(black_box(&file_path)).await.unwrap();
            }
        });
    });
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

/// Benchmark: Scan throughput with different file sizes
fn bench_scan_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    let mut group = c.benchmark_group("scan_throughput");
    
    for size in [1024, 10240, 102400, 1024000].iter() {
        let test_file = PathBuf::from(format!("/tmp/bench_size_{}.txt", size));
        let content = "A".repeat(*size);
        fs::write(&test_file, &content).unwrap();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                scanner.scan_file(black_box(&test_file)).await.unwrap()
            });
        });
        
        let _ = fs::remove_file(&test_file);
    }
    
    group.finish();
}

/// Benchmark: Quick scan performance
fn bench_quick_scan(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    c.bench_function("quick_scan", |b| {
        b.to_async(&rt).iter(|| async {
            scanner.start_quick_scan().await.unwrap();
            while scanner.is_scanning() {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
    });
}

/// Benchmark: Concurrent file scans
fn bench_concurrent_scans(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = std::sync::Arc::new(Scanner::new().unwrap());
    
    // Create test files
    let test_dir = PathBuf::from("/tmp/bench_concurrent");
    fs::create_dir_all(&test_dir).unwrap();
    
    for i in 0..50 {
        let file_path = test_dir.join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("Content {}", i)).unwrap();
    }
    
    c.bench_function("concurrent_50_scans", |b| {
        b.to_async(&rt).iter(|| async {
            let mut handles = vec![];
            
            for i in 0..50 {
                let scanner_clone = std::sync::Arc::clone(&scanner);
                let file_path = test_dir.join(format!("file_{}.txt", i));
                
                let handle = tokio::spawn(async move {
                    scanner_clone.scan_file(&file_path).await
                });
                
                handles.push(handle);
            }
            
            for handle in handles {
                handle.await.unwrap().unwrap();
            }
        });
    });
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

/// Benchmark: Scanner statistics retrieval
fn bench_get_stats(c: &mut Criterion) {
    let scanner = Scanner::new().unwrap();
    
    c.bench_function("get_stats", |b| {
        b.iter(|| {
            scanner.get_stats().unwrap()
        });
    });
}

/// Benchmark: Scan state management
fn bench_scan_lifecycle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    c.bench_function("scan_lifecycle", |b| {
        b.to_async(&rt).iter(|| async {
            let scan_id = scanner.start_quick_scan().await.unwrap();
            scanner.pause_scan().unwrap();
            scanner.resume_scan().unwrap();
            scanner.stop_scan(scan_id).unwrap();
        });
    });
}

/// Benchmark: Memory usage during scan
fn bench_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let scanner = Scanner::new().unwrap();
    
    // Create large test directory
    let test_dir = PathBuf::from("/tmp/bench_memory");
    fs::create_dir_all(&test_dir).unwrap();
    
    for i in 0..1000 {
        let file_path = test_dir.join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("Content {}", i)).unwrap();
    }
    
    c.bench_function("scan_1000_files_memory", |b| {
        b.to_async(&rt).iter(|| async {
            for i in 0..1000 {
                let file_path = test_dir.join(format!("file_{}.txt", i));
                scanner.scan_file(&file_path).await.unwrap();
            }
        });
    });
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

criterion_group!(
    benches,
    bench_scanner_init,
    bench_single_file_scan,
    bench_multiple_file_scans,
    bench_scan_throughput,
    bench_quick_scan,
    bench_concurrent_scans,
    bench_get_stats,
    bench_scan_lifecycle,
    bench_memory_usage
);

criterion_main!(benches);