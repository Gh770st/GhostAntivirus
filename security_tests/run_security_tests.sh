#!/bin/bash

# GhostAntivirus Security Testing Runner
# Runs all security tests and generates comprehensive reports

set -e

echo "=========================================="
echo "GhostAntivirus Security Testing Suite"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check Python availability
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: Python 3 not found${NC}"
    exit 1
fi

echo -e "${CYAN}Starting security test suite...${NC}"
echo ""

# Create results directory
mkdir -p security_test_results

# Run automated security tests
echo -e "${YELLOW}[1/4] Running Automated Security Tests...${NC}"
echo "----------------------------------------"
if python3 security_test_suite.py; then
    echo -e "${GREEN}✓ Automated security tests complete${NC}"
    mv security_test_report.json security_test_results/ 2>/dev/null || true
else
    echo -e "${RED}✗ Some security tests failed${NC}"
fi
echo ""

# Check for dependency vulnerabilities (Rust)
echo -e "${YELLOW}[2/4] Checking Rust Dependencies...${NC}"
echo "------------------------------------"
cd ../core
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        echo -e "${GREEN}✓ No vulnerabilities found in Rust dependencies${NC}"
    else
        echo -e "${RED}✗ Vulnerabilities found in Rust dependencies${NC}"
    fi
else
    echo -e "${YELLOW}⚠ cargo-audit not installed. Install with: cargo install cargo-audit${NC}"
fi
cd ../security_tests
echo ""

# Check for dependency vulnerabilities (Python)
echo -e "${YELLOW}[3/4] Checking Python Dependencies...${NC}"
echo "--------------------------------------"
if [ -f "../ai-engine/requirements.txt" ]; then
    if command -v pip-audit &> /dev/null; then
        if pip-audit -r ../ai-engine/requirements.txt; then
            echo -e "${GREEN}✓ No vulnerabilities found in Python dependencies${NC}"
        else
            echo -e "${RED}✗ Vulnerabilities found in Python dependencies${NC}"
        fi
    else
        echo -e "${YELLOW}⚠ pip-audit not installed. Install with: pip install pip-audit${NC}"
    fi
else
    echo -e "${YELLOW}⚠ requirements.txt not found${NC}"
fi
echo ""

# Static code analysis
echo -e "${YELLOW}[4/4] Running Static Code Analysis...${NC}"
echo "--------------------------------------"
cd ../core
if command -v cargo-clippy &> /dev/null; then
    if cargo clippy -- -D warnings; then
        echo -e "${GREEN}✓ No clippy warnings${NC}"
    else
        echo -e "${YELLOW}⚠ Clippy warnings found${NC}"
    fi
else
    echo -e "${YELLOW}⚠ cargo-clippy not installed${NC}"
fi
cd ../security_tests
echo ""

echo ""
echo "=========================================="
echo "Security Test Results Summary"
echo "=========================================="
echo ""
echo -e "${CYAN}Test Reports Generated:${NC}"
echo "  - Automated Tests: security_test_results/security_test_report.json"
echo ""
echo -e "${CYAN}Security Test Categories:${NC}"
echo ""
echo "  1. File Permissions:"
echo "     - World-writable files check"
echo "     - Script executability"
echo ""
echo "  2. Secrets & Credentials:"
echo "     - Hardcoded passwords"
echo "     - API keys"
echo "     - Private keys"
echo ""
echo "  3. Dependencies:"
echo "     - Known vulnerabilities (Rust)"
echo "     - Known vulnerabilities (Python)"
echo ""
echo "  4. Input Validation:"
echo "     - SQL injection prevention"
echo "     - Path traversal prevention"
echo ""
echo "  5. Cryptography:"
echo "     - Weak algorithms check"
echo "     - Strong algorithms usage"
echo ""
echo "  6. Error Handling:"
echo "     - Unwrap usage"
echo "     - Result/Option handling"
echo ""
echo "  7. Logging:"
echo "     - Sensitive data in logs"
echo ""
echo "  8. Authentication:"
echo "     - Session management"
echo "     - Rate limiting"
echo ""
echo "  9. Network Security:"
echo "     - HTTPS usage"
echo "     - Certificate validation"
echo ""
echo "  10. Configuration:"
echo "      - Debug mode"
echo "      - Environment variables"
echo ""
echo -e "${CYAN}Security Best Practices Checked:${NC}"
echo "  - No hardcoded secrets"
echo "  - Strong encryption algorithms"
echo "  - Proper error handling"
echo "  - Secure logging practices"
echo "  - Input validation"
echo "  - Dependency security"
echo "  - Network security"
echo "  - Configuration security"
echo ""
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}   Security tests complete! ✓${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Review security test report"
echo "  2. Address any failed tests"
echo "  3. Fix security warnings"
echo "  4. Update dependencies with vulnerabilities"
echo "  5. Re-run tests to verify fixes"
echo ""
echo -e "${CYAN}Additional Security Tools:${NC}"
echo "  - cargo-audit: cargo install cargo-audit"
echo "  - pip-audit: pip install pip-audit"
echo "  - cargo-clippy: rustup component add clippy"
echo ""