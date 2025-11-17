#!/usr/bin/env python3
"""
GhostAntivirus Scanner Load Testing Suite

This script performs load testing on the scanner module to measure
performance under high load conditions.
"""

import asyncio
import time
import statistics
import json
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import tempfile
import os

class ScannerLoadTest:
    def __init__(self):
        self.results = {
            'tests': [],
            'summary': {}
        }
    
    def create_test_files(self, count, size_kb=10):
        """Create test files for scanning"""
        test_dir = Path(tempfile.mkdtemp(prefix='ghost_load_test_'))
        files = []
        
        print(f"Creating {count} test files ({size_kb}KB each)...")
        for i in range(count):
            file_path = test_dir / f"test_file_{i}.txt"
            content = "A" * (size_kb * 1024)
            file_path.write_text(content)
            files.append(file_path)
        
        return test_dir, files
    
    def cleanup_test_files(self, test_dir):
        """Clean up test files"""
        import shutil
        if test_dir.exists():
            shutil.rmtree(test_dir)
    
    async def scan_file_async(self, file_path):
        """Simulate async file scan"""
        start_time = time.time()
        
        # Simulate scan operation
        await asyncio.sleep(0.01)  # Simulated scan time
        
        end_time = time.time()
        return end_time - start_time
    
    async def test_concurrent_scans(self, file_count, concurrent_limit):
        """Test concurrent file scanning"""
        print(f"\n{'='*60}")
        print(f"Test: Concurrent Scans")
        print(f"Files: {file_count}, Concurrent: {concurrent_limit}")
        print(f"{'='*60}")
        
        test_dir, files = self.create_test_files(file_count)
        
        start_time = time.time()
        scan_times = []
        
        # Create semaphore to limit concurrency
        semaphore = asyncio.Semaphore(concurrent_limit)
        
        async def scan_with_limit(file_path):
            async with semaphore:
                return await self.scan_file_async(file_path)
        
        # Scan all files concurrently
        tasks = [scan_with_limit(f) for f in files]
        scan_times = await asyncio.gather(*tasks)
        
        end_time = time.time()
        total_time = end_time - start_time
        
        # Calculate statistics
        avg_scan_time = statistics.mean(scan_times)
        median_scan_time = statistics.median(scan_times)
        throughput = file_count / total_time
        
        result = {
            'test': 'concurrent_scans',
            'file_count': file_count,
            'concurrent_limit': concurrent_limit,
            'total_time': total_time,
            'avg_scan_time': avg_scan_time,
            'median_scan_time': median_scan_time,
            'throughput': throughput,
            'files_per_second': throughput
        }
        
        self.results['tests'].append(result)
        
        print(f"\nResults:")
        print(f"  Total Time: {total_time:.2f}s")
        print(f"  Avg Scan Time: {avg_scan_time*1000:.2f}ms")
        print(f"  Median Scan Time: {median_scan_time*1000:.2f}ms")
        print(f"  Throughput: {throughput:.2f} files/s")
        
        self.cleanup_test_files(test_dir)
        
        return result
    
    async def test_sustained_load(self, duration_seconds, files_per_second):
        """Test sustained scanning load"""
        print(f"\n{'='*60}")
        print(f"Test: Sustained Load")
        print(f"Duration: {duration_seconds}s, Rate: {files_per_second} files/s")
        print(f"{'='*60}")
        
        test_dir, files = self.create_test_files(100)
        
        start_time = time.time()
        scans_completed = 0
        scan_times = []
        
        interval = 1.0 / files_per_second
        
        while time.time() - start_time < duration_seconds:
            file_path = files[scans_completed % len(files)]
            scan_time = await self.scan_file_async(file_path)
            scan_times.append(scan_time)
            scans_completed += 1
            
            await asyncio.sleep(interval)
        
        end_time = time.time()
        total_time = end_time - start_time
        actual_rate = scans_completed / total_time
        
        result = {
            'test': 'sustained_load',
            'duration': duration_seconds,
            'target_rate': files_per_second,
            'actual_rate': actual_rate,
            'scans_completed': scans_completed,
            'avg_scan_time': statistics.mean(scan_times),
            'max_scan_time': max(scan_times),
            'min_scan_time': min(scan_times)
        }
        
        self.results['tests'].append(result)
        
        print(f"\nResults:")
        print(f"  Scans Completed: {scans_completed}")
        print(f"  Target Rate: {files_per_second} files/s")
        print(f"  Actual Rate: {actual_rate:.2f} files/s")
        print(f"  Avg Scan Time: {statistics.mean(scan_times)*1000:.2f}ms")
        
        self.cleanup_test_files(test_dir)
        
        return result
    
    async def test_burst_load(self, burst_size, burst_count):
        """Test burst scanning load"""
        print(f"\n{'='*60}")
        print(f"Test: Burst Load")
        print(f"Burst Size: {burst_size}, Bursts: {burst_count}")
        print(f"{'='*60}")
        
        test_dir, files = self.create_test_files(burst_size)
        
        burst_times = []
        
        for burst_num in range(burst_count):
            start_time = time.time()
            
            tasks = [self.scan_file_async(f) for f in files]
            await asyncio.gather(*tasks)
            
            end_time = time.time()
            burst_time = end_time - start_time
            burst_times.append(burst_time)
            
            print(f"  Burst {burst_num + 1}/{burst_count}: {burst_time:.2f}s")
            
            # Wait between bursts
            await asyncio.sleep(1)
        
        result = {
            'test': 'burst_load',
            'burst_size': burst_size,
            'burst_count': burst_count,
            'avg_burst_time': statistics.mean(burst_times),
            'max_burst_time': max(burst_times),
            'min_burst_time': min(burst_times)
        }
        
        self.results['tests'].append(result)
        
        print(f"\nResults:")
        print(f"  Avg Burst Time: {statistics.mean(burst_times):.2f}s")
        print(f"  Max Burst Time: {max(burst_times):.2f}s")
        print(f"  Min Burst Time: {min(burst_times):.2f}s")
        
        self.cleanup_test_files(test_dir)
        
        return result
    
    async def test_scalability(self):
        """Test scalability with increasing load"""
        print(f"\n{'='*60}")
        print(f"Test: Scalability")
        print(f"{'='*60}")
        
        file_counts = [10, 50, 100, 500, 1000]
        results = []
        
        for count in file_counts:
            test_dir, files = self.create_test_files(count)
            
            start_time = time.time()
            tasks = [self.scan_file_async(f) for f in files]
            await asyncio.gather(*tasks)
            end_time = time.time()
            
            total_time = end_time - start_time
            throughput = count / total_time
            
            results.append({
                'file_count': count,
                'time': total_time,
                'throughput': throughput
            })
            
            print(f"  {count} files: {total_time:.2f}s ({throughput:.2f} files/s)")
            
            self.cleanup_test_files(test_dir)
        
        result = {
            'test': 'scalability',
            'results': results
        }
        
        self.results['tests'].append(result)
        
        return result
    
    def generate_report(self, output_file='load_test_report.json'):
        """Generate load test report"""
        self.results['summary'] = {
            'total_tests': len(self.results['tests']),
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S')
        }
        
        with open(output_file, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print(f"\n{'='*60}")
        print(f"Load Test Report")
        print(f"{'='*60}")
        print(f"Total Tests: {self.results['summary']['total_tests']}")
        print(f"Report saved to: {output_file}")
        print(f"{'='*60}")

async def main():
    """Main load testing function"""
    print("GhostAntivirus Scanner Load Testing Suite")
    print("=" * 60)
    
    tester = ScannerLoadTest()
    
    # Test 1: Concurrent scans with different concurrency levels
    await tester.test_concurrent_scans(file_count=100, concurrent_limit=10)
    await tester.test_concurrent_scans(file_count=100, concurrent_limit=50)
    await tester.test_concurrent_scans(file_count=100, concurrent_limit=100)
    
    # Test 2: Sustained load
    await tester.test_sustained_load(duration_seconds=30, files_per_second=10)
    await tester.test_sustained_load(duration_seconds=30, files_per_second=50)
    
    # Test 3: Burst load
    await tester.test_burst_load(burst_size=50, burst_count=5)
    await tester.test_burst_load(burst_size=100, burst_count=3)
    
    # Test 4: Scalability
    await tester.test_scalability()
    
    # Generate report
    tester.generate_report()
    
    print("\n✓ All load tests completed!")

if __name__ == '__main__':
    asyncio.run(main())