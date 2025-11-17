#!/usr/bin/env python3
"""
Integration tests for GhostAntivirus API
Tests the Core Engine REST API endpoints
"""

import requests
import json
import time
from typing import Dict, Any

# Configuration
BASE_URL = "http://localhost:8080"
API_BASE = f"{BASE_URL}/api"

# Test credentials
TEST_USER = {
    "username": "admin",
    "password": "admin123"
}

class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    END = '\033[0m'

class APITester:
    def __init__(self):
        self.token = None
        self.passed = 0
        self.failed = 0
        self.total = 0

    def log_test(self, name: str, passed: bool, message: str = ""):
        self.total += 1
        if passed:
            self.passed += 1
            print(f"{Colors.GREEN}✓{Colors.END} {name}")
            if message:
                print(f"  {Colors.BLUE}→{Colors.END} {message}")
        else:
            self.failed += 1
            print(f"{Colors.RED}✗{Colors.END} {name}")
            if message:
                print(f"  {Colors.RED}→{Colors.END} {message}")

    def test_health_check(self):
        """Test health check endpoint"""
        try:
            response = requests.get(f"{BASE_URL}/health", timeout=5)
            passed = response.status_code == 200
            data = response.json() if passed else {}
            self.log_test(
                "Health Check",
                passed,
                f"Status: {data.get('data', {}).get('status', 'unknown')}"
            )
            return passed
        except Exception as e:
            self.log_test("Health Check", False, str(e))
            return False

    def test_login(self):
        """Test login endpoint"""
        try:
            response = requests.post(
                f"{BASE_URL}/auth/login",
                json=TEST_USER,
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json()
                self.token = data.get('data', {}).get('token')
                self.log_test(
                    "Login",
                    True,
                    f"Token received: {self.token[:20]}..."
                )
            else:
                self.log_test("Login", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Login", False, str(e))
            return False

    def get_headers(self) -> Dict[str, str]:
        """Get authorization headers"""
        return {
            "Authorization": f"Bearer {self.token}",
            "Content-Type": "application/json"
        }

    def test_scan_stats(self):
        """Test scan statistics endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/scan/stats",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Scan Stats",
                    True,
                    f"Scanned: {data.get('total_scanned', 0)}, Threats: {data.get('threats_found', 0)}"
                )
            else:
                self.log_test("Scan Stats", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Scan Stats", False, str(e))
            return False

    def test_start_scan(self):
        """Test start scan endpoint"""
        try:
            payload = {
                "path": "/tmp",
                "scan_type": "quick",
                "deep_scan": False
            }
            response = requests.post(
                f"{API_BASE}/scan/start",
                headers=self.get_headers(),
                json=payload,
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Start Scan",
                    True,
                    f"Scan ID: {data.get('scan_id', 'unknown')}"
                )
            else:
                self.log_test("Start Scan", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Start Scan", False, str(e))
            return False

    def test_threats_list(self):
        """Test threats list endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/threats",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Threats List",
                    True,
                    f"Total threats: {data.get('total', 0)}"
                )
            else:
                self.log_test("Threats List", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Threats List", False, str(e))
            return False

    def test_quarantine_list(self):
        """Test quarantine list endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/quarantine",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Quarantine List",
                    True,
                    f"Files in quarantine: {data.get('total', 0)}"
                )
            else:
                self.log_test("Quarantine List", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Quarantine List", False, str(e))
            return False

    def test_firewall_rules(self):
        """Test firewall rules endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/firewall/rules",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Firewall Rules",
                    True,
                    f"Total rules: {data.get('total', 0)}"
                )
            else:
                self.log_test("Firewall Rules", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Firewall Rules", False, str(e))
            return False

    def test_network_connections(self):
        """Test network connections endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/network/connections",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Network Connections",
                    True,
                    f"Active connections: {data.get('total', 0)}"
                )
            else:
                self.log_test("Network Connections", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Network Connections", False, str(e))
            return False

    def test_settings(self):
        """Test settings endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/settings",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "Settings",
                    True,
                    f"Real-time protection: {data.get('real_time_protection', False)}"
                )
            else:
                self.log_test("Settings", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("Settings", False, str(e))
            return False

    def test_system_info(self):
        """Test system info endpoint"""
        try:
            response = requests.get(
                f"{API_BASE}/system/info",
                headers=self.get_headers(),
                timeout=5
            )
            passed = response.status_code == 200
            if passed:
                data = response.json().get('data', {})
                self.log_test(
                    "System Info",
                    True,
                    f"Version: {data.get('version', 'unknown')}, OS: {data.get('os', 'unknown')}"
                )
            else:
                self.log_test("System Info", False, f"Status: {response.status_code}")
            return passed
        except Exception as e:
            self.log_test("System Info", False, str(e))
            return False

    def run_all_tests(self):
        """Run all integration tests"""
        print(f"\n{Colors.BLUE}{'='*60}{Colors.END}")
        print(f"{Colors.BLUE}GhostAntivirus API Integration Tests{Colors.END}")
        print(f"{Colors.BLUE}{'='*60}{Colors.END}\n")

        print(f"{Colors.YELLOW}Testing API Server: {BASE_URL}{Colors.END}\n")

        # Test health check first
        if not self.test_health_check():
            print(f"\n{Colors.RED}API server is not responding. Please start the server first.{Colors.END}")
            return

        print()

        # Test authentication
        if not self.test_login():
            print(f"\n{Colors.RED}Authentication failed. Cannot continue tests.{Colors.END}")
            return

        print()

        # Test all endpoints
        self.test_scan_stats()
        self.test_start_scan()
        self.test_threats_list()
        self.test_quarantine_list()
        self.test_firewall_rules()
        self.test_network_connections()
        self.test_settings()
        self.test_system_info()

        # Print summary
        print(f"\n{Colors.BLUE}{'='*60}{Colors.END}")
        print(f"{Colors.BLUE}Test Summary{Colors.END}")
        print(f"{Colors.BLUE}{'='*60}{Colors.END}")
        print(f"Total Tests: {self.total}")
        print(f"{Colors.GREEN}Passed: {self.passed}{Colors.END}")
        print(f"{Colors.RED}Failed: {self.failed}{Colors.END}")
        
        success_rate = (self.passed / self.total * 100) if self.total > 0 else 0
        print(f"Success Rate: {success_rate:.1f}%")
        
        if self.failed == 0:
            print(f"\n{Colors.GREEN}✓ All tests passed!{Colors.END}\n")
        else:
            print(f"\n{Colors.RED}✗ Some tests failed.{Colors.END}\n")

if __name__ == "__main__":
    tester = APITester()
    tester.run_all_tests()