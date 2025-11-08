#!/bin/bash

# GhostAntivirus Benchmark Runner
# Runs all performance benchmarks and generates reports

set -e

echo "=========================================="
echo "GhostAntivirus Performance Benchmark Suite"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if criterion is available
if ! cargo bench --help &> /dev/null; then
    echo -e "${RED}Error: cargo bench not available${NC}"
    echo "Please ensure you have Rust nightly or criterion installed"
    exit 1
fi

echo -e "${CYAN}Starting benchmark suite...${NC}"
echo ""

cd core

# Run scanner benchmarks
echo -e "${YELLOW}[1/3] Running Scanner Benchmarks...${NC}"
echo "-----------------------------------"
if cargo bench --bench scanner_benchmarks; then
    echo -e "${GREEN}✓ Scanner benchmarks complete${NC}"
else
    echo -e "${RED}✗ Scanner benchmarks failed${NC}"
fi
echo ""

# Run quarantine benchmarks
echo -e "${YELLOW}[2/3] Running Quarantine Benchmarks...${NC}"
echo "--------------------------------------"
if cargo bench --bench quarantine_benchmarks; then
    echo -e "${GREEN}✓ Quarantine benchmarks complete${NC}"
else
    echo -e "${RED}✗ Quarantine benchmarks failed${NC}"
fi
echo ""

# Run system benchmarks
echo -e "${YELLOW}[3/3] Running System Benchmarks...${NC}"
echo "-----------------------------------"
if cargo bench --bench system_benchmarks; then
    echo -e "${GREEN}✓ System benchmarks complete${NC}"
else
    echo -e "${RED}✗ System benchmarks failed${NC}"
fi
echo ""

cd ..

echo ""
echo "=========================================="
echo "Benchmark Results Summary"
echo "=========================================="
echo ""
echo -e "${CYAN}Benchmark reports generated in:${NC}"
echo "  core/target/criterion/"
echo ""
echo -e "${CYAN}View detailed reports:${NC}"
echo "  - Scanner: core/target/criterion/scanner_*/report/index.html"
echo "  - Quarantine: core/target/criterion/quarantine_*/report/index.html"
echo "  - System: core/target/criterion/system_*/report/index.html"
echo ""
echo -e "${GREEN}Benchmark Categories:${NC}"
echo "  1. Scanner Performance:"
echo "     - Initialization time"
echo "     - Single file scan"
echo "     - Multiple file scans"
echo "     - Scan throughput (various file sizes)"
echo "     - Quick scan performance"
echo "     - Concurrent scans"
echo "     - Statistics retrieval"
echo "     - Scan lifecycle management"
echo "     - Memory usage"
echo ""
echo "  2. Quarantine Performance:"
echo "     - Initialization time"
echo "     - Add file operations"
echo "     - File size impact"
echo "     - List operations"
echo "     - File info retrieval"
echo "     - Restore operations"
echo "     - Delete operations"
echo "     - Concurrent operations"
echo "     - Encryption performance"
echo ""
echo "  3. System Monitoring Performance:"
echo "     - Initialization time"
echo "     - Metrics collection"
echo "     - CPU/Memory/Disk usage"
echo "     - Process management"
echo "     - Health checks"
echo "     - Network interfaces"
echo "     - I/O statistics"
echo "     - Snapshot operations"
echo "     - Monitoring overhead"
echo "     - Concurrent operations"
echo ""
echo -e "${CYAN}Performance Metrics Collected:${NC}"
echo "  - Execution time (mean, median, std dev)"
echo "  - Throughput (operations per second)"
echo "  - Memory usage"
echo "  - CPU utilization"
echo "  - Scalability (concurrent operations)"
echo ""
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}   All benchmarks complete! ✓${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Review benchmark reports in target/criterion/"
echo "  2. Identify performance bottlenecks"
echo "  3. Optimize critical paths"
echo "  4. Re-run benchmarks to verify improvements"
echo ""