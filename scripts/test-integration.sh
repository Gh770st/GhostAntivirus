#!/bin/bash

# Integration Testing Script for GhostAntivirus
# This script tests the full integration of all components

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}GhostAntivirus Integration Testing${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}✗ Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Docker is running${NC}\n"

# Step 1: Build all containers
echo -e "${YELLOW}Step 1: Building Docker containers...${NC}"
docker-compose -f docker-compose.integration.yml build
echo -e "${GREEN}✓ Containers built successfully${NC}\n"

# Step 2: Start all services
echo -e "${YELLOW}Step 2: Starting all services...${NC}"
docker-compose -f docker-compose.integration.yml up -d
echo -e "${GREEN}✓ Services started${NC}\n"

# Step 3: Wait for services to be ready
echo -e "${YELLOW}Step 3: Waiting for services to be ready...${NC}"
sleep 10

# Check Core Engine
echo -n "Checking Core Engine... "
if curl -s http://localhost:8080/health > /dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Check AI Engine
echo -n "Checking AI Engine... "
if curl -s http://localhost:8000/health > /dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Check Network Guard
echo -n "Checking Network Guard... "
if curl -s http://localhost:9000/health > /dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

# Check Web Dashboard
echo -n "Checking Web Dashboard... "
if curl -s http://localhost:3000 > /dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
fi

echo ""

# Step 4: Run API integration tests
echo -e "${YELLOW}Step 4: Running API integration tests...${NC}"
if [ -f "integration-tests/test_api.py" ]; then
    python3 integration-tests/test_api.py
else
    echo -e "${RED}✗ Test script not found${NC}"
fi

echo ""

# Step 5: Show service logs
echo -e "${YELLOW}Step 5: Service Status${NC}"
docker-compose -f docker-compose.integration.yml ps

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Integration Testing Complete${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${GREEN}Services are running at:${NC}"
echo -e "  Core Engine API:  http://localhost:8080"
echo -e "  AI Engine API:    http://localhost:8000"
echo -e "  Network Guard:    http://localhost:9000"
echo -e "  Web Dashboard:    http://localhost:3000"
echo ""

echo -e "${YELLOW}To stop services:${NC}"
echo -e "  docker-compose -f docker-compose.integration.yml down"
echo ""

echo -e "${YELLOW}To view logs:${NC}"
echo -e "  docker-compose -f docker-compose.integration.yml logs -f [service-name]"
echo ""