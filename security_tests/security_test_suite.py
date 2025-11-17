#!/usr/bin/env python3
"""
GhostAntivirus Security Testing Suite

Automated security tests for common vulnerabilities and security best practices.
"""

import os
import sys
import json
import subprocess
import re
from pathlib import Path
from typing import List, Dict, Tuple

class SecurityTestSuite:
    def __init__(self):
        self.results = {
            'tests': [],
            'summary': {
                'total': 0,
                'passed': 0,
                'failed': 0,
                'warnings': 0
            }
        }
        self.project_root = Path(__file__).parent.parent
    
    def log_test(self, category: str, test_name: str, status: str, details: str = ""):
        """Log test result"""
        result = {
            'category': category,
            'test': test_name,
            'status': status,
            'details': details
        }
        self.results['tests'].append(result)
        self.results['summary']['total'] += 1
        
        if status == 'PASS':
            self.results['summary']['passed'] += 1
            print(f"  ✓ {test_name}: PASS")
        elif status == 'FAIL':
            self.results['summary']['failed'] += 1
            print(f"  ✗ {test_name}: FAIL - {details}")
        elif status == 'WARN':
            self.results['summary']['warnings'] += 1
            print(f"  ⚠ {test_name}: WARNING - {details}")
    
    def test_file_permissions(self):
        """Test file and directory permissions"""
        print("\n[1/10] Testing File Permissions...")
        
        # Check for world-writable files
        try:
            result = subprocess.run(
                ['find', str(self.project_root), '-type', 'f', '-perm', '-002'],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            world_writable = result.stdout.strip().split('\n') if result.stdout.strip() else []
            
            if world_writable and world_writable[0]:
                self.log_test('File Security', 'World-writable files', 'FAIL', 
                            f"Found {len(world_writable)} world-writable files")
            else:
                self.log_test('File Security', 'World-writable files', 'PASS')
        except Exception as e:
            self.log_test('File Security', 'World-writable files', 'WARN', str(e))
        
        # Check for executable scripts
        scripts = list(self.project_root.glob('**/*.sh'))
        for script in scripts:
            if os.access(script, os.X_OK):
                self.log_test('File Security', f'Script executable: {script.name}', 'PASS')
            else:
                self.log_test('File Security', f'Script executable: {script.name}', 'WARN', 
                            'Script not executable')
    
    def test_hardcoded_secrets(self):
        """Test for hardcoded secrets and credentials"""
        print("\n[2/10] Testing for Hardcoded Secrets...")
        
        patterns = [
            (r'password\s*=\s*["\'][^"\']+["\']', 'Hardcoded password'),
            (r'api[_-]?key\s*=\s*["\'][^"\']+["\']', 'Hardcoded API key'),
            (r'secret\s*=\s*["\'][^"\']+["\']', 'Hardcoded secret'),
            (r'token\s*=\s*["\'][^"\']+["\']', 'Hardcoded token'),
            (r'-----BEGIN\s+(?:RSA\s+)?PRIVATE\s+KEY-----', 'Private key'),
        ]
        
        code_files = []
        for ext in ['*.rs', '*.py', '*.js', '*.go', '*.ts']:
            code_files.extend(self.project_root.glob(f'**/{ext}'))
        
        findings = []
        for file_path in code_files:
            try:
                content = file_path.read_text()
                for pattern, desc in patterns:
                    matches = re.finditer(pattern, content, re.IGNORECASE)
                    for match in matches:
                        findings.append(f"{file_path.name}: {desc}")
            except Exception:
                pass
        
        if findings:
            self.log_test('Secrets', 'Hardcoded secrets check', 'FAIL', 
                        f"Found {len(findings)} potential secrets")
        else:
            self.log_test('Secrets', 'Hardcoded secrets check', 'PASS')
    
    def test_dependency_vulnerabilities(self):
        """Test for known dependency vulnerabilities"""
        print("\n[3/10] Testing Dependency Vulnerabilities...")
        
        # Check Rust dependencies
        cargo_toml = self.project_root / 'core' / 'Cargo.toml'
        if cargo_toml.exists():
            try:
                result = subprocess.run(
                    ['cargo', 'audit'],
                    cwd=cargo_toml.parent,
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                if 'vulnerabilities found' in result.stdout.lower():
                    self.log_test('Dependencies', 'Rust dependencies', 'FAIL', 
                                'Vulnerabilities found in Cargo dependencies')
                else:
                    self.log_test('Dependencies', 'Rust dependencies', 'PASS')
            except FileNotFoundError:
                self.log_test('Dependencies', 'Rust dependencies', 'WARN', 
                            'cargo-audit not installed')
            except Exception as e:
                self.log_test('Dependencies', 'Rust dependencies', 'WARN', str(e))
        
        # Check Python dependencies
        requirements = self.project_root / 'ai-engine' / 'requirements.txt'
        if requirements.exists():
            try:
                result = subprocess.run(
                    ['pip-audit', '-r', str(requirements)],
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                if 'vulnerabilities found' in result.stdout.lower():
                    self.log_test('Dependencies', 'Python dependencies', 'FAIL', 
                                'Vulnerabilities found in Python dependencies')
                else:
                    self.log_test('Dependencies', 'Python dependencies', 'PASS')
            except FileNotFoundError:
                self.log_test('Dependencies', 'Python dependencies', 'WARN', 
                            'pip-audit not installed')
            except Exception as e:
                self.log_test('Dependencies', 'Python dependencies', 'WARN', str(e))
    
    def test_input_validation(self):
        """Test for input validation patterns"""
        print("\n[4/10] Testing Input Validation...")
        
        # Check for SQL injection prevention
        rust_files = list(self.project_root.glob('**/*.rs'))
        
        sql_safe = True
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                # Check for string concatenation in SQL queries
                if re.search(r'format!\s*\(\s*["\'].*SELECT.*FROM', content):
                    sql_safe = False
                    break
            except Exception:
                pass
        
        if sql_safe:
            self.log_test('Input Validation', 'SQL injection prevention', 'PASS')
        else:
            self.log_test('Input Validation', 'SQL injection prevention', 'WARN', 
                        'Potential SQL injection vulnerability')
        
        # Check for path traversal prevention
        path_safe = True
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                # Check for unsafe path operations
                if re.search(r'Path::new\([^)]*\+', content):
                    path_safe = False
                    break
            except Exception:
                pass
        
        if path_safe:
            self.log_test('Input Validation', 'Path traversal prevention', 'PASS')
        else:
            self.log_test('Input Validation', 'Path traversal prevention', 'WARN', 
                        'Potential path traversal vulnerability')
    
    def test_encryption_usage(self):
        """Test for proper encryption usage"""
        print("\n[5/10] Testing Encryption Usage...")
        
        rust_files = list(self.project_root.glob('**/*.rs'))
        
        # Check for weak encryption algorithms
        weak_algos = ['md5', 'sha1', 'des', 'rc4']
        weak_found = []
        
        for file_path in rust_files:
            try:
                content = file_path.read_text().lower()
                for algo in weak_algos:
                    if algo in content:
                        weak_found.append(f"{file_path.name}: {algo}")
            except Exception:
                pass
        
        if weak_found:
            self.log_test('Cryptography', 'Weak algorithms', 'FAIL', 
                        f"Found weak algorithms: {', '.join(weak_found)}")
        else:
            self.log_test('Cryptography', 'Weak algorithms', 'PASS')
        
        # Check for strong encryption
        strong_algos = ['aes', 'sha256', 'sha512']
        strong_found = False
        
        for file_path in rust_files:
            try:
                content = file_path.read_text().lower()
                if any(algo in content for algo in strong_algos):
                    strong_found = True
                    break
            except Exception:
                pass
        
        if strong_found:
            self.log_test('Cryptography', 'Strong algorithms', 'PASS')
        else:
            self.log_test('Cryptography', 'Strong algorithms', 'WARN', 
                        'No strong encryption algorithms found')
    
    def test_error_handling(self):
        """Test for proper error handling"""
        print("\n[6/10] Testing Error Handling...")
        
        rust_files = list(self.project_root.glob('**/*.rs'))
        
        # Check for unwrap() usage (can cause panics)
        unwrap_count = 0
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                unwrap_count += len(re.findall(r'\.unwrap\(\)', content))
            except Exception:
                pass
        
        if unwrap_count > 50:  # Allow some unwraps in tests
            self.log_test('Error Handling', 'Unwrap usage', 'WARN', 
                        f"Found {unwrap_count} unwrap() calls")
        else:
            self.log_test('Error Handling', 'Unwrap usage', 'PASS')
        
        # Check for proper Result/Option handling
        result_handling = 0
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                result_handling += len(re.findall(r'\.map_err\(|\.ok\(\)|\.err\(\)', content))
            except Exception:
                pass
        
        if result_handling > 0:
            self.log_test('Error Handling', 'Result handling', 'PASS')
        else:
            self.log_test('Error Handling', 'Result handling', 'WARN', 
                        'Limited Result/Option handling found')
    
    def test_logging_security(self):
        """Test for secure logging practices"""
        print("\n[7/10] Testing Logging Security...")
        
        code_files = []
        for ext in ['*.rs', '*.py', '*.js']:
            code_files.extend(self.project_root.glob(f'**/{ext}'))
        
        # Check for sensitive data in logs
        sensitive_patterns = [
            r'log.*password',
            r'log.*secret',
            r'log.*token',
            r'log.*api[_-]?key',
        ]
        
        sensitive_logging = []
        for file_path in code_files:
            try:
                content = file_path.read_text().lower()
                for pattern in sensitive_patterns:
                    if re.search(pattern, content):
                        sensitive_logging.append(file_path.name)
                        break
            except Exception:
                pass
        
        if sensitive_logging:
            self.log_test('Logging', 'Sensitive data in logs', 'WARN', 
                        f"Potential sensitive logging in {len(sensitive_logging)} files")
        else:
            self.log_test('Logging', 'Sensitive data in logs', 'PASS')
    
    def test_authentication_security(self):
        """Test authentication security"""
        print("\n[8/10] Testing Authentication Security...")
        
        # Check for session management
        rust_files = list(self.project_root.glob('**/*.rs'))
        
        session_mgmt = False
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                if 'session' in content.lower() or 'auth' in content.lower():
                    session_mgmt = True
                    break
            except Exception:
                pass
        
        if session_mgmt:
            self.log_test('Authentication', 'Session management', 'PASS')
        else:
            self.log_test('Authentication', 'Session management', 'WARN', 
                        'No session management found')
        
        # Check for rate limiting
        rate_limiting = False
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                if 'rate' in content.lower() and 'limit' in content.lower():
                    rate_limiting = True
                    break
            except Exception:
                pass
        
        if rate_limiting:
            self.log_test('Authentication', 'Rate limiting', 'PASS')
        else:
            self.log_test('Authentication', 'Rate limiting', 'WARN', 
                        'No rate limiting found')
    
    def test_network_security(self):
        """Test network security"""
        print("\n[9/10] Testing Network Security...")
        
        # Check for HTTPS usage
        code_files = []
        for ext in ['*.rs', '*.py', '*.js']:
            code_files.extend(self.project_root.glob(f'**/{ext}'))
        
        http_usage = []
        for file_path in code_files:
            try:
                content = file_path.read_text()
                if re.search(r'http://(?!localhost|127\.0\.0\.1)', content):
                    http_usage.append(file_path.name)
            except Exception:
                pass
        
        if http_usage:
            self.log_test('Network', 'HTTPS usage', 'WARN', 
                        f"HTTP usage found in {len(http_usage)} files")
        else:
            self.log_test('Network', 'HTTPS usage', 'PASS')
        
        # Check for certificate validation
        cert_validation = False
        for file_path in code_files:
            try:
                content = file_path.read_text()
                if 'verify' in content.lower() and 'cert' in content.lower():
                    cert_validation = True
                    break
            except Exception:
                pass
        
        if cert_validation:
            self.log_test('Network', 'Certificate validation', 'PASS')
        else:
            self.log_test('Network', 'Certificate validation', 'WARN', 
                        'No certificate validation found')
    
    def test_configuration_security(self):
        """Test configuration security"""
        print("\n[10/10] Testing Configuration Security...")
        
        # Check for debug mode
        config_files = list(self.project_root.glob('**/*.toml'))
        config_files.extend(self.project_root.glob('**/*.yaml'))
        config_files.extend(self.project_root.glob('**/*.json'))
        
        debug_mode = []
        for file_path in config_files:
            try:
                content = file_path.read_text()
                if re.search(r'debug\s*=\s*true', content, re.IGNORECASE):
                    debug_mode.append(file_path.name)
            except Exception:
                pass
        
        if debug_mode:
            self.log_test('Configuration', 'Debug mode', 'WARN', 
                        f"Debug mode enabled in {len(debug_mode)} files")
        else:
            self.log_test('Configuration', 'Debug mode', 'PASS')
        
        # Check for environment variables
        env_vars = False
        for file_path in config_files:
            try:
                content = file_path.read_text()
                if 'env' in content.lower() or 'environment' in content.lower():
                    env_vars = True
                    break
            except Exception:
                pass
        
        if env_vars:
            self.log_test('Configuration', 'Environment variables', 'PASS')
        else:
            self.log_test('Configuration', 'Environment variables', 'WARN', 
                        'No environment variable usage found')
    
    def generate_report(self, output_file='security_test_report.json'):
        """Generate security test report"""
        with open(output_file, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print("\n" + "="*60)
        print("Security Test Report")
        print("="*60)
        print(f"Total Tests: {self.results['summary']['total']}")
        print(f"Passed: {self.results['summary']['passed']}")
        print(f"Failed: {self.results['summary']['failed']}")
        print(f"Warnings: {self.results['summary']['warnings']}")
        print(f"\nReport saved to: {output_file}")
        print("="*60)
        
        # Calculate security score
        total = self.results['summary']['total']
        passed = self.results['summary']['passed']
        warnings = self.results['summary']['warnings']
        
        score = ((passed + warnings * 0.5) / total * 100) if total > 0 else 0
        
        print(f"\nSecurity Score: {score:.1f}/100")
        
        if score >= 90:
            print("Status: ✓ Excellent")
        elif score >= 75:
            print("Status: ✓ Good")
        elif score >= 60:
            print("Status: ⚠ Fair")
        else:
            print("Status: ✗ Needs Improvement")

def main():
    """Main security testing function"""
    print("GhostAntivirus Security Testing Suite")
    print("=" * 60)
    
    tester = SecurityTestSuite()
    
    # Run all security tests
    tester.test_file_permissions()
    tester.test_hardcoded_secrets()
    tester.test_dependency_vulnerabilities()
    tester.test_input_validation()
    tester.test_encryption_usage()
    tester.test_error_handling()
    tester.test_logging_security()
    tester.test_authentication_security()
    tester.test_network_security()
    tester.test_configuration_security()
    
    # Generate report
    tester.generate_report()
    
    print("\n✓ All security tests completed!")
    
    # Exit with error code if there are failures
    if tester.results['summary']['failed'] > 0:
        sys.exit(1)

if __name__ == '__main__':
    main()