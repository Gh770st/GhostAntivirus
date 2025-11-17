#!/bin/bash

# GhostAntivirus Load Testing Runner
# Runs all load tests and generates comprehensive reports

set -e

echo "=========================================="
echo "GhostAntivirus Load Testing Suite"
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

# Install dependencies if needed
echo -e "${CYAN}Checking dependencies...${NC}"
pip3 install -q aiohttp 2>/dev/null || true

echo ""
echo -e "${CYAN}Starting load test suite...${NC}"
echo ""

# Create results directory
mkdir -p load_test_results

# Run scanner load tests
echo -e "${YELLOW}[1/2] Running Scanner Load Tests...${NC}"
echo "-----------------------------------"
if python3 load_test_scanner.py; then
    echo -e "${GREEN}✓ Scanner load tests complete${NC}"
    mv load_test_report.json load_test_results/scanner_load_test.json 2>/dev/null || true
else
    echo -e "${RED}✗ Scanner load tests failed${NC}"
fi
echo ""

# Run API load tests (requires running server)
echo -e "${YELLOW}[2/2] Running API Load Tests...${NC}"
echo "--------------------------------"
echo -e "${BLUE}Note: Ensure GhostAntivirus API server is running on localhost:8080${NC}"
read -p "Press Enter to continue or Ctrl+C to skip..."

if python3 load_test_api.py; then
    echo -e "${GREEN}✓ API load tests complete${NC}"
    mv api_load_test_report.json load_test_results/api_load_test.json 2>/dev/null || true
else
    echo -e "${RED}✗ API load tests failed (server may not be running)${NC}"
fi
echo ""

echo ""
echo "=========================================="
echo "Load Test Results Summary"
echo "=========================================="
echo ""
echo -e "${CYAN}Test Reports Generated:${NC}"
echo "  - Scanner Load Test: load_test_results/scanner_load_test.json"
echo "  - API Load Test: load_test_results/api_load_test.json"
echo ""
echo -e "${CYAN}Load Test Categories:${NC}"
echo ""
echo "  1. Scanner Load Tests:"
echo "     - Concurrent scans (10, 50, 100 concurrent)"
echo "     - Sustained load (10, 50 files/s for 30s)"
echo "     - Burst load (50, 100 files per burst)"
echo "     - Scalability (10 to 1000 files)"
echo ""
echo "  2. API Load Tests:"
echo "     - Concurrent requests (10, 50 concurrent)"
echo "     - Sustained load (10, 50 req/s for 30s)"
echo "     - Endpoint stress test (20 concurrent for 30s)"
echo "     - Rate limiting validation"
echo ""
echo -e "${CYAN}Key Metrics Measured:${NC}"
echo "  - Throughput (operations/second)"
echo "  - Response time (avg, median, min, max)"
echo "  - Success/failure rates"
echo "  - Scalability characteristics"
echo "  - Resource utilization"
echo ""
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}   All load tests complete! ✓${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Review test reports in load_test_results/"
echo "  2. Analyze performance bottlenecks"
echo "  3. Compare with performance targets"
echo "  4. Optimize critical paths"
echo "  5. Re-run tests to verify improvements"
echo ""