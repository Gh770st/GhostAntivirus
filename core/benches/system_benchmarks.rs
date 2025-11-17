//! System Monitoring Performance Benchmarks
//! 
//! Benchmark suite for the system monitoring module.
//! Measures metrics collection, resource monitoring, and performance overhead.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ghost_antivirus::system::SystemMonitor;

/// Benchmark: System monitor initialization
fn bench_system_init(c: &mut Criterion) {
    c.bench_function("system_monitor_init", |b| {
        b.iter(|| {
            SystemMonitor::new().unwrap()
        });
    });
}

/// Benchmark: Get system metrics
fn bench_get_metrics(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_system_metrics", |b| {
        b.iter(|| {
            monitor.get_metrics().unwrap()
        });
    });
}

/// Benchmark: Get CPU usage
fn bench_get_cpu_usage(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_cpu_usage", |b| {
        b.iter(|| {
            monitor.get_cpu_usage().unwrap()
        });
    });
}

/// Benchmark: Get memory usage
fn bench_get_memory_usage(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_memory_usage", |b| {
        b.iter(|| {
            monitor.get_memory_usage().unwrap()
        });
    });
}

/// Benchmark: Get disk usage
fn bench_get_disk_usage(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_disk_usage", |b| {
        b.iter(|| {
            monitor.get_disk_usage().unwrap()
        });
    });
}

/// Benchmark: Get all processes
fn bench_get_processes(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_all_processes", |b| {
        b.iter(|| {
            monitor.get_processes().unwrap()
        });
    });
}

/// Benchmark: Get process by PID
fn bench_get_process_by_pid(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    let current_pid = std::process::id();
    
    c.bench_function("get_process_by_pid", |b| {
        b.iter(|| {
            monitor.get_process(black_box(current_pid)).unwrap()
        });
    });
}

/// Benchmark: Get top CPU processes
fn bench_get_top_cpu(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_top_cpu_processes", |b| {
        b.iter(|| {
            monitor.get_top_cpu_processes(10).unwrap()
        });
    });
}

/// Benchmark: Get top memory processes
fn bench_get_top_memory(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_top_memory_processes", |b| {
        b.iter(|| {
            monitor.get_top_memory_processes(10).unwrap()
        });
    });
}

/// Benchmark: System health check
fn bench_health_check(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("system_health_check", |b| {
        b.iter(|| {
            monitor.check_health().unwrap()
        });
    });
}

/// Benchmark: Get network interfaces
fn bench_get_network_interfaces(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_network_interfaces", |b| {
        b.iter(|| {
            monitor.get_network_interfaces().unwrap()
        });
    });
}

/// Benchmark: Get system info
fn bench_get_system_info(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_system_info", |b| {
        b.iter(|| {
            monitor.get_system_info().unwrap()
        });
    });
}

/// Benchmark: Get load average
fn bench_get_load_average(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_load_average", |b| {
        b.iter(|| {
            monitor.get_load_average().unwrap()
        });
    });
}

/// Benchmark: Get disk I/O
fn bench_get_disk_io(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_disk_io", |b| {
        b.iter(|| {
            monitor.get_disk_io().unwrap()
        });
    });
}

/// Benchmark: Get network I/O
fn bench_get_network_io(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("get_network_io", |b| {
        b.iter(|| {
            monitor.get_network_io().unwrap()
        });
    });
}

/// Benchmark: Take metrics snapshot
fn bench_take_snapshot(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("take_metrics_snapshot", |b| {
        b.iter(|| {
            monitor.take_snapshot().unwrap()
        });
    });
}

/// Benchmark: Compare snapshots
fn bench_compare_snapshots(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    let snapshot1 = monitor.take_snapshot().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let snapshot2 = monitor.take_snapshot().unwrap();
    
    c.bench_function("compare_snapshots", |b| {
        b.iter(|| {
            monitor.compare_snapshots(black_box(&snapshot1), black_box(&snapshot2)).unwrap()
        });
    });
}

/// Benchmark: Monitoring overhead
fn bench_monitoring_overhead(c: &mut Criterion) {
    let monitor = SystemMonitor::new().unwrap();
    
    c.bench_function("monitoring_overhead", |b| {
        b.iter(|| {
            monitor.start_monitoring().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
            monitor.stop_monitoring().unwrap();
        });
    });
}

/// Benchmark: Concurrent metric collection
fn bench_concurrent_metrics(c: &mut Criterion) {
    let monitor = std::sync::Arc::new(SystemMonitor::new().unwrap());
    
    c.bench_function("concurrent_metric_collection", |b| {
        b.iter(|| {
            let mut handles = vec![];
            
            for _ in 0..10 {
                let monitor_clone = std::sync::Arc::clone(&monitor);
                let handle = std::thread::spawn(move || {
                    let _ = monitor_clone.get_metrics();
                    let _ = monitor_clone.get_cpu_usage();
                    let _ = monitor_clone.get_memory_usage();
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_system_init,
    bench_get_metrics,
    bench_get_cpu_usage,
    bench_get_memory_usage,
    bench_get_disk_usage,
    bench_get_processes,
    bench_get_process_by_pid,
    bench_get_top_cpu,
    bench_get_top_memory,
    bench_health_check,
    bench_get_network_interfaces,
    bench_get_system_info,
    bench_get_load_average,
    bench_get_disk_io,
    bench_get_network_io,
    bench_take_snapshot,
    bench_compare_snapshots,
    bench_monitoring_overhead,
    bench_concurrent_metrics
);

criterion_main!(benches);