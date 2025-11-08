//! Quarantine Performance Benchmarks
//! 
//! Benchmark suite for the quarantine module.
//! Measures quarantine operations, encryption, and file management performance.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ghost_antivirus::quarantine::QuarantineManager;
use std::path::PathBuf;
use std::fs;

/// Benchmark: Quarantine manager initialization
fn bench_quarantine_init(c: &amp;mut Criterion) {
    c.bench_function("quarantine_init", |b| {
        b.iter(|| {
            QuarantineManager::new().unwrap()
        });
    });
}

/// Benchmark: Add file to quarantine
fn bench_add_file(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    // Create test file
    let test_file = PathBuf::from("/tmp/quarantine_bench.txt");
    fs::write(&amp;test_file, "Test content for quarantine benchmarking").unwrap();
    
    c.bench_function("add_file_to_quarantine", |b| {
        b.iter(|| {
            let qid = quarantine.add_file(black_box(&amp;test_file), "Test threat").unwrap();
            // Restore to allow re-quarantine
            quarantine.restore_file(qid, &amp;test_file).unwrap();
        });
    });
    
    // Cleanup
    let _ = fs::remove_file(&amp;test_file);
}

/// Benchmark: Add files of different sizes
fn bench_add_file_sizes(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    let mut group = c.benchmark_group("quarantine_file_sizes");
    
    for size in [1024, 10240, 102400, 1024000].iter() {
        let test_file = PathBuf::from(format!("/tmp/quarantine_size_{}.txt", size));
        let content = "A".repeat(*size);
        fs::write(&amp;test_file, &amp;content).unwrap();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let qid = quarantine.add_file(black_box(&amp;test_file), "Test threat").unwrap();
                quarantine.restore_file(qid, &amp;test_file).unwrap();
            });
        });
        
        let _ = fs::remove_file(&amp;test_file);
    }
    
    group.finish();
}

/// Benchmark: List quarantined files
fn bench_list_files(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    // Add some files to quarantine
    for i in 0..10 {
        let test_file = PathBuf::from(format!("/tmp/quarantine_list_{}.txt", i));
        fs::write(&amp;test_file, format!("Content {}", i)).unwrap();
        quarantine.add_file(&amp;test_file, "Test threat").unwrap();
    }
    
    c.bench_function("list_quarantined_files", |b| {
        b.iter(|| {
            quarantine.list_files().unwrap()
        });
    });
}

/// Benchmark: Get file info
fn bench_get_file_info(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    // Add test file
    let test_file = PathBuf::from("/tmp/quarantine_info.txt");
    fs::write(&amp;test_file, "Test content").unwrap();
    let qid = quarantine.add_file(&amp;test_file, "Test threat").unwrap();
    
    c.bench_function("get_file_info", |b| {
        b.iter(|| {
            quarantine.get_file_info(black_box(qid)).unwrap()
        });
    });
}

/// Benchmark: Restore file from quarantine
fn bench_restore_file(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    let test_file = PathBuf::from("/tmp/quarantine_restore.txt");
    let restore_path = PathBuf::from("/tmp/quarantine_restored.txt");
    
    c.bench_function("restore_file", |b| {
        b.iter(|| {
            fs::write(&amp;test_file, "Test content").unwrap();
            let qid = quarantine.add_file(&amp;test_file, "Test threat").unwrap();
            quarantine.restore_file(black_box(qid), &amp;restore_path).unwrap();
            let _ = fs::remove_file(&amp;restore_path);
        });
    });
}

/// Benchmark: Delete quarantined file
fn bench_delete_file(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    c.bench_function("delete_quarantined_file", |b| {
        b.iter(|| {
            let test_file = PathBuf::from("/tmp/quarantine_delete.txt");
            fs::write(&amp;test_file, "Test content").unwrap();
            let qid = quarantine.add_file(&amp;test_file, "Test threat").unwrap();
            quarantine.delete_file(black_box(qid)).unwrap();
        });
    });
}

/// Benchmark: Get quarantine count
fn bench_get_count(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    c.bench_function("get_quarantine_count", |b| {
        b.iter(|| {
            quarantine.get_count()
        });
    });
}

/// Benchmark: Concurrent quarantine operations
fn bench_concurrent_operations(c: &amp;mut Criterion) {
    let quarantine = std::sync::Arc::new(QuarantineManager::new().unwrap());
    
    c.bench_function("concurrent_quarantine_ops", |b| {
        b.iter(|| {
            let mut handles = vec![];
            
            for i in 0..10 {
                let quarantine_clone = std::sync::Arc::clone(&amp;quarantine);
                let handle = std::thread::spawn(move || {
                    let test_file = PathBuf::from(format!("/tmp/quarantine_concurrent_{}.txt", i));
                    fs::write(&amp;test_file, format!("Content {}", i)).unwrap();
                    let qid = quarantine_clone.add_file(&amp;test_file, "Test threat").unwrap();
                    quarantine_clone.delete_file(qid).unwrap();
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark: Encryption performance
fn bench_encryption(c: &amp;mut Criterion) {
    let quarantine = QuarantineManager::new().unwrap();
    
    let mut group = c.benchmark_group("encryption_performance");
    
    for size in [1024, 10240, 102400].iter() {
        let test_file = PathBuf::from(format!("/tmp/quarantine_encrypt_{}.txt", size));
        let content = "A".repeat(*size);
        fs::write(&amp;test_file, &amp;content).unwrap();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let qid = quarantine.add_file(black_box(&amp;test_file), "Test threat").unwrap();
                quarantine.restore_file(qid, &amp;test_file).unwrap();
            });
        });
        
        let _ = fs::remove_file(&amp;test_file);
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_quarantine_init,
    bench_add_file,
    bench_add_file_sizes,
    bench_list_files,
    bench_get_file_info,
    bench_restore_file,
    bench_delete_file,
    bench_get_count,
    bench_concurrent_operations,
    bench_encryption
);

criterion_main!(benches);