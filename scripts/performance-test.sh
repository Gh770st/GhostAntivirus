#!/bin/bash

# Performance Testing Script for GhostAntivirus
# Tests API performance, load handling, and response times

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
BASE_URL="http://localhost:8080"
CONCURRENT_USERS=10
REQUESTS_PER_USER=100
TOTAL_REQUESTS=$((CONCURRENT_USERS * REQUESTS_PER_USER))

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}GhostAntivirus Performance Testing${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Check if Apache Bench is installed
if ! command -v ab &> /dev/null; then
    echo -e "${YELLOW}Apache Bench (ab) not found. Installing...${NC}"
    sudo apt-get update && sudo apt-get install -y apache2-utils
fi

# Function to run performance test
run_test() {
    local endpoint=$1
    local name=$2
    
    echo -e "\n${BLUE}Testing: $name${NC}"
    echo -e "${YELLOW}Endpoint: $endpoint${NC}"
    echo -e "${YELLOW}Concurrent Users: $CONCURRENT_USERS${NC}"
    echo -e "${YELLOW}Requests per User: $REQUESTS_PER_USER${NC}\n"
    
    ab -n $TOTAL_REQUESTS -c $CONCURRENT_USERS "$BASE_URL$endpoint" 2>&1 | \
        grep -E "Requests per second|Time per request|Transfer rate|Failed requests"
}

# Login first to get token
echo -e "${BLUE}Authenticating...${NC}"
TOKEN=$(curl -s -X POST "$BASE_URL/auth/login" \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin123"}' | \
    jq -r '.data.token')

if [ -z "$TOKEN" ] || [ "$TOKEN" == "null" ]; then
    echo -e "${RED}Authentication failed. Please ensure the API is running.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Authentication successful${NC}"

# Test 1: Health Check Endpoint
run_test "/health" "Health Check"

# Test 2: Scan Stats (requires auth)
echo -e "\n${BLUE}Testing: Scan Stats (Authenticated)${NC}"
echo -e "${YELLOW}Note: This test requires authentication${NC}\n"

# Create temporary file with auth header
cat > /tmp/ab-headers.txt << EOF
Authorization: Bearer $TOKEN
EOF

ab -n $TOTAL_REQUESTS -c $CONCURRENT_USERS \
    -H "Authorization: Bearer $TOKEN" \
    "$BASE_URL/api/scan/stats" 2>&1 | \
    grep -E "Requests per second|Time per request|Transfer rate|Failed requests"

# Test 3: Concurrent Load Test
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}Concurrent Load Test${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Testing with increasing concurrent users...${NC}\n"

for users in 1 5 10 20 50; do
    echo -e "${BLUE}Testing with $users concurrent users...${NC}"
    ab -n $((users * 10)) -c $users "$BASE_URL/health" 2>&1 | \
        grep "Requests per second" | \
        awk -v users=$users '{print "  " users " users: " $4 " req/sec"}'
done

# Test 4: Response Time Analysis
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}Response Time Analysis${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Measuring response times for different endpoints...${NC}\n"

endpoints=(
    "/health:Health Check"
    "/api/scan/stats:Scan Stats"
    "/api/threats:Threats List"
    "/api/quarantine:Quarantine List"
)

for endpoint_info in "${endpoints[@]}"; do
    IFS=':' read -r endpoint name <<< "$endpoint_info"
    echo -e "${BLUE}$name${NC}"
    
    if [[ "$endpoint" == "/health" ]]; then
        curl -w "\n  Time: %{time_total}s\n  Size: %{size_download} bytes\n" \
            -o /dev/null -s "$BASE_URL$endpoint"
    else
        curl -w "\n  Time: %{time_total}s\n  Size: %{size_download} bytes\n" \
            -H "Authorization: Bearer $TOKEN" \
            -o /dev/null -s "$BASE_URL$endpoint"
    fi
done

# Test 5: Memory and CPU Usage
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}Resource Usage${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Docker Container Resource Usage:${NC}\n"
docker stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}" | \
    grep ghost

# Summary
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}Performance Test Complete${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${GREEN}✓ All performance tests completed${NC}"
echo -e "\n${YELLOW}Recommendations:${NC}"
echo -e "  1. Monitor response times under load"
echo -e "  2. Optimize slow endpoints"
echo -e "  3. Consider caching for frequently accessed data"
echo -e "  4. Scale horizontally if needed"
echo -e "  5. Set up monitoring alerts for performance degradation"
echo ""