#!/usr/bin/env python3
"""
GhostAntivirus API Load Testing Suite

This script performs load testing on the REST API to measure
performance under high load conditions.
"""

import asyncio
import aiohttp
import time
import statistics
import json
from typing import List, Dict

class APILoadTest:
    def __init__(self, base_url='http://localhost:8080'):
        self.base_url = base_url
        self.results = {
            'tests': [],
            'summary': {}
        }
    
    async def make_request(self, session, method, endpoint, data=None):
        """Make HTTP request and measure response time"""
        url = f"{self.base_url}{endpoint}"
        start_time = time.time()
        
        try:
            if method == 'GET':
                async with session.get(url) as response:
                    await response.text()
                    status = response.status
            elif method == 'POST':
                async with session.post(url, json=data) as response:
                    await response.text()
                    status = response.status
            
            end_time = time.time()
            return {
                'success': True,
                'status': status,
                'time': end_time - start_time
            }
        except Exception as e:
            end_time = time.time()
            return {
                'success': False,
                'error': str(e),
                'time': end_time - start_time
            }
    
    async def test_concurrent_requests(self, endpoint, method='GET', concurrent=10, total=100):
        """Test concurrent API requests"""
        print(f"\n{'='*60}")
        print(f"Test: Concurrent API Requests")
        print(f"Endpoint: {endpoint}, Method: {method}")
        print(f"Concurrent: {concurrent}, Total: {total}")
        print(f"{'='*60}")
        
        async with aiohttp.ClientSession() as session:
            semaphore = asyncio.Semaphore(concurrent)
            
            async def request_with_limit():
                async with semaphore:
                    return await self.make_request(session, method, endpoint)
            
            start_time = time.time()
            results = await asyncio.gather(*[request_with_limit() for _ in range(total)])
            end_time = time.time()
            
            total_time = end_time - start_time
            successful = sum(1 for r in results if r['success'])
            failed = total - successful
            
            response_times = [r['time'] for r in results if r['success']]
            
            if response_times:
                avg_time = statistics.mean(response_times)
                median_time = statistics.median(response_times)
                min_time = min(response_times)
                max_time = max(response_times)
                throughput = successful / total_time
            else:
                avg_time = median_time = min_time = max_time = throughput = 0
            
            result = {
                'test': 'concurrent_requests',
                'endpoint': endpoint,
                'method': method,
                'concurrent': concurrent,
                'total_requests': total,
                'successful': successful,
                'failed': failed,
                'total_time': total_time,
                'avg_response_time': avg_time,
                'median_response_time': median_time,
                'min_response_time': min_time,
                'max_response_time': max_time,
                'throughput': throughput
            }
            
            self.results['tests'].append(result)
            
            print(f"\nResults:")
            print(f"  Total Time: {total_time:.2f}s")
            print(f"  Successful: {successful}/{total}")
            print(f"  Failed: {failed}/{total}")
            print(f"  Avg Response Time: {avg_time*1000:.2f}ms")
            print(f"  Median Response Time: {median_time*1000:.2f}ms")
            print(f"  Throughput: {throughput:.2f} req/s")
            
            return result
    
    async def test_sustained_load(self, endpoint, duration_seconds=30, requests_per_second=10):
        """Test sustained API load"""
        print(f"\n{'='*60}")
        print(f"Test: Sustained API Load")
        print(f"Endpoint: {endpoint}")
        print(f"Duration: {duration_seconds}s, Rate: {requests_per_second} req/s")
        print(f"{'='*60}")
        
        async with aiohttp.ClientSession() as session:
            start_time = time.time()
            requests_completed = 0
            response_times = []
            errors = 0
            
            interval = 1.0 / requests_per_second
            
            while time.time() - start_time < duration_seconds:
                result = await self.make_request(session, 'GET', endpoint)
                
                if result['success']:
                    response_times.append(result['time'])
                else:
                    errors += 1
                
                requests_completed += 1
                await asyncio.sleep(interval)
            
            end_time = time.time()
            total_time = end_time - start_time
            actual_rate = requests_completed / total_time
            
            result = {
                'test': 'sustained_load',
                'endpoint': endpoint,
                'duration': duration_seconds,
                'target_rate': requests_per_second,
                'actual_rate': actual_rate,
                'requests_completed': requests_completed,
                'errors': errors,
                'avg_response_time': statistics.mean(response_times) if response_times else 0,
                'max_response_time': max(response_times) if response_times else 0,
                'min_response_time': min(response_times) if response_times else 0
            }
            
            self.results['tests'].append(result)
            
            print(f"\nResults:")
            print(f"  Requests Completed: {requests_completed}")
            print(f"  Errors: {errors}")
            print(f"  Target Rate: {requests_per_second} req/s")
            print(f"  Actual Rate: {actual_rate:.2f} req/s")
            print(f"  Avg Response Time: {result['avg_response_time']*1000:.2f}ms")
            
            return result
    
    async def test_endpoint_stress(self, endpoints: List[str], concurrent=50, duration=60):
        """Stress test multiple endpoints"""
        print(f"\n{'='*60}")
        print(f"Test: Endpoint Stress Test")
        print(f"Endpoints: {len(endpoints)}, Concurrent: {concurrent}")
        print(f"Duration: {duration}s")
        print(f"{'='*60}")
        
        async with aiohttp.ClientSession() as session:
            start_time = time.time()
            requests_completed = 0
            endpoint_stats = {ep: {'success': 0, 'failed': 0, 'times': []} for ep in endpoints}
            
            async def stress_worker():
                nonlocal requests_completed
                while time.time() - start_time < duration:
                    endpoint = endpoints[requests_completed % len(endpoints)]
                    result = await self.make_request(session, 'GET', endpoint)
                    
                    if result['success']:
                        endpoint_stats[endpoint]['success'] += 1
                        endpoint_stats[endpoint]['times'].append(result['time'])
                    else:
                        endpoint_stats[endpoint]['failed'] += 1
                    
                    requests_completed += 1
            
            # Create worker tasks
            workers = [stress_worker() for _ in range(concurrent)]
            await asyncio.gather(*workers)
            
            end_time = time.time()
            total_time = end_time - start_time
            
            result = {
                'test': 'endpoint_stress',
                'endpoints': endpoints,
                'concurrent': concurrent,
                'duration': duration,
                'total_time': total_time,
                'requests_completed': requests_completed,
                'throughput': requests_completed / total_time,
                'endpoint_stats': {}
            }
            
            for endpoint, stats in endpoint_stats.items():
                if stats['times']:
                    result['endpoint_stats'][endpoint] = {
                        'success': stats['success'],
                        'failed': stats['failed'],
                        'avg_time': statistics.mean(stats['times'])
                    }
            
            self.results['tests'].append(result)
            
            print(f"\nResults:")
            print(f"  Total Requests: {requests_completed}")
            print(f"  Throughput: {result['throughput']:.2f} req/s")
            for endpoint, stats in result['endpoint_stats'].items():
                print(f"  {endpoint}:")
                print(f"    Success: {stats['success']}, Failed: {stats['failed']}")
                print(f"    Avg Time: {stats['avg_time']*1000:.2f}ms")
            
            return result
    
    async def test_rate_limiting(self, endpoint, requests_per_window=100, window_seconds=60):
        """Test API rate limiting"""
        print(f"\n{'='*60}")
        print(f"Test: Rate Limiting")
        print(f"Endpoint: {endpoint}")
        print(f"Limit: {requests_per_window} req/{window_seconds}s")
        print(f"{'='*60}")
        
        async with aiohttp.ClientSession() as session:
            start_time = time.time()
            requests_sent = 0
            rate_limited = 0
            successful = 0
            
            # Send requests rapidly
            for _ in range(requests_per_window + 50):
                result = await self.make_request(session, 'GET', endpoint)
                requests_sent += 1
                
                if result['success']:
                    if result['status'] == 429:  # Too Many Requests
                        rate_limited += 1
                    else:
                        successful += 1
            
            end_time = time.time()
            total_time = end_time - start_time
            
            result = {
                'test': 'rate_limiting',
                'endpoint': endpoint,
                'requests_sent': requests_sent,
                'successful': successful,
                'rate_limited': rate_limited,
                'total_time': total_time
            }
            
            self.results['tests'].append(result)
            
            print(f"\nResults:")
            print(f"  Requests Sent: {requests_sent}")
            print(f"  Successful: {successful}")
            print(f"  Rate Limited: {rate_limited}")
            print(f"  Time: {total_time:.2f}s")
            
            return result
    
    def generate_report(self, output_file='api_load_test_report.json'):
        """Generate load test report"""
        self.results['summary'] = {
            'total_tests': len(self.results['tests']),
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S')
        }
        
        with open(output_file, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print(f"\n{'='*60}")
        print(f"API Load Test Report")
        print(f"{'='*60}")
        print(f"Total Tests: {self.results['summary']['total_tests']}")
        print(f"Report saved to: {output_file}")
        print(f"{'='*60}")

async def main():
    """Main load testing function"""
    print("GhostAntivirus API Load Testing Suite")
    print("=" * 60)
    
    tester = APILoadTest()
    
    # Common endpoints
    endpoints = [
        '/api/scanner/stats',
        '/api/scanner/status',
        '/api/threats',
        '/api/quarantine',
        '/api/system/metrics'
    ]
    
    # Test 1: Concurrent requests to single endpoint
    await tester.test_concurrent_requests('/api/scanner/stats', concurrent=10, total=100)
    await tester.test_concurrent_requests('/api/scanner/stats', concurrent=50, total=500)
    
    # Test 2: Sustained load
    await tester.test_sustained_load('/api/scanner/stats', duration_seconds=30, requests_per_second=10)
    await tester.test_sustained_load('/api/scanner/stats', duration_seconds=30, requests_per_second=50)
    
    # Test 3: Endpoint stress test
    await tester.test_endpoint_stress(endpoints, concurrent=20, duration=30)
    
    # Test 4: Rate limiting
    await tester.test_rate_limiting('/api/scanner/stats', requests_per_window=100, window_seconds=60)
    
    # Generate report
    tester.generate_report()
    
    print("\n✓ All API load tests completed!")

if __name__ == '__main__':
    asyncio.run(main())