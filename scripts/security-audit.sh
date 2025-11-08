#!/bin/bash

# Security Audit Script for GhostAntivirus
# Performs security checks and vulnerability scanning

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}GhostAntivirus Security Audit${NC}"
echo -e "${BLUE}========================================${NC}\n"

PASSED=0
FAILED=0
WARNINGS=0

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED++))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED++))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

# Section 1: Environment Variables
echo -e "${BLUE}1. Environment Variables Security${NC}\n"

if [ -f ".env" ]; then
    # Check JWT Secret
    if grep -q "JWT_SECRET=your-super-secret-jwt-key-change-this-in-production" .env; then
        check_fail "JWT_SECRET is using default value - CHANGE THIS!"
    else
        check_pass "JWT_SECRET has been customized"
    fi
    
    # Check Database Password
    if grep -q "POSTGRES_PASSWORD=your-secure-database-password" .env; then
        check_fail "POSTGRES_PASSWORD is using default value - CHANGE THIS!"
    else
        check_pass "POSTGRES_PASSWORD has been customized"
    fi
    
    # Check Redis Password
    if grep -q "REDIS_PASSWORD=your-secure-redis-password" .env; then
        check_fail "REDIS_PASSWORD is using default value - CHANGE THIS!"
    else
        check_pass "REDIS_PASSWORD has been customized"
    fi
    
    # Check if .env is in .gitignore
    if grep -q "^\.env$" .gitignore 2>/dev/null; then
        check_pass ".env is in .gitignore"
    else
        check_fail ".env is NOT in .gitignore - sensitive data may be exposed!"
    fi
else
    check_warn ".env file not found"
fi

# Section 2: File Permissions
echo -e "\n${BLUE}2. File Permissions${NC}\n"

# Check if sensitive files have proper permissions
if [ -f ".env" ]; then
    PERMS=$(stat -c "%a" .env)
    if [ "$PERMS" == "600" ] || [ "$PERMS" == "400" ]; then
        check_pass ".env has secure permissions ($PERMS)"
    else
        check_warn ".env permissions are $PERMS (recommended: 600)"
    fi
fi

# Check script permissions
for script in scripts/*.sh; do
    if [ -x "$script" ]; then
        check_pass "$(basename $script) is executable"
    else
        check_warn "$(basename $script) is not executable"
    fi
done

# Section 3: Docker Security
echo -e "\n${BLUE}3. Docker Security${NC}\n"

# Check if containers are running as non-root
if docker-compose -f docker-compose.production.yml ps | grep -q "Up"; then
    check_pass "Docker containers are running"
    
    # Check for non-root users
    for container in $(docker-compose -f docker-compose.production.yml ps -q); do
        USER=$(docker inspect -f '{{.Config.User}}' $container)
        NAME=$(docker inspect -f '{{.Name}}' $container | sed 's/\///')
        if [ -z "$USER" ] || [ "$USER" == "root" ]; then
            check_warn "$NAME is running as root"
        else
            check_pass "$NAME is running as non-root user ($USER)"
        fi
    done
else
    check_warn "Docker containers are not running"
fi

# Section 4: Network Security
echo -e "\n${BLUE}4. Network Security${NC}\n"

# Check if firewall is enabled
if command -v ufw &> /dev/null; then
    if sudo ufw status | grep -q "Status: active"; then
        check_pass "UFW firewall is active"
    else
        check_warn "UFW firewall is not active"
    fi
else
    check_warn "UFW firewall not installed"
fi

# Check exposed ports
echo -e "\n${YELLOW}Exposed Ports:${NC}"
if docker-compose -f docker-compose.production.yml ps | grep -q "Up"; then
    docker-compose -f docker-compose.production.yml ps | grep "Up" | awk '{print "  " $1 ": " $NF}'
fi

# Section 5: SSL/TLS Configuration
echo -e "\n${BLUE}5. SSL/TLS Configuration${NC}\n"

if [ -f ".env" ]; then
    if grep -q "SSL_ENABLED=true" .env; then
        check_pass "SSL is enabled"
        
        # Check if certificates exist
        SSL_CERT=$(grep "SSL_CERT_PATH" .env | cut -d'=' -f2)
        SSL_KEY=$(grep "SSL_KEY_PATH" .env | cut -d'=' -f2)
        
        if [ -f "$SSL_CERT" ]; then
            check_pass "SSL certificate found"
        else
            check_warn "SSL certificate not found at $SSL_CERT"
        fi
        
        if [ -f "$SSL_KEY" ]; then
            check_pass "SSL key found"
        else
            check_warn "SSL key not found at $SSL_KEY"
        fi
    else
        check_warn "SSL is not enabled"
    fi
fi

# Section 6: Authentication Security
echo -e "\n${BLUE}6. Authentication Security${NC}\n"

# Test default credentials
echo -e "${YELLOW}Testing default credentials...${NC}"
RESPONSE=$(curl -s -X POST http://localhost:8080/auth/login \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin123"}' 2>/dev/null || echo "")

if echo "$RESPONSE" | grep -q "token"; then
    check_fail "Default credentials (admin/admin123) are still active - CHANGE THIS!"
else
    check_pass "Default credentials have been changed or API is not accessible"
fi

# Section 7: Dependency Security
echo -e "\n${BLUE}7. Dependency Security${NC}\n"

# Check for outdated npm packages
if [ -f "web-ui/package.json" ]; then
    echo -e "${YELLOW}Checking npm dependencies...${NC}"
    cd web-ui
    if command -v npm &> /dev/null; then
        OUTDATED=$(npm outdated 2>/dev/null | wc -l)
        if [ "$OUTDATED" -gt 1 ]; then
            check_warn "Found $((OUTDATED-1)) outdated npm packages"
        else
            check_pass "All npm packages are up to date"
        fi
    fi
    cd ..
fi

# Check for Rust security advisories
if [ -f "core/Cargo.toml" ]; then
    echo -e "${YELLOW}Checking Rust dependencies...${NC}"
    if command -v cargo &> /dev/null; then
        cd core
        if cargo audit --version &> /dev/null; then
            VULNS=$(cargo audit 2>/dev/null | grep "Crate:" | wc -l)
            if [ "$VULNS" -gt 0 ]; then
                check_warn "Found $VULNS potential vulnerabilities in Rust dependencies"
            else
                check_pass "No known vulnerabilities in Rust dependencies"
            fi
        else
            check_warn "cargo-audit not installed (run: cargo install cargo-audit)"
        fi
        cd ..
    fi
fi

# Section 8: API Security
echo -e "\n${BLUE}8. API Security${NC}\n"

# Check CORS configuration
if [ -f ".env" ]; then
    CORS_ORIGINS=$(grep "CORS_ALLOWED_ORIGINS" .env | cut -d'=' -f2)
    if [ "$CORS_ORIGINS" == "*" ]; then
        check_warn "CORS allows all origins - consider restricting in production"
    else
        check_pass "CORS is configured with specific origins"
    fi
fi

# Check rate limiting
if [ -f ".env" ]; then
    if grep -q "RATE_LIMIT_ENABLED=true" .env; then
        check_pass "Rate limiting is enabled"
    else
        check_warn "Rate limiting is not enabled"
    fi
fi

# Section 9: Logging and Monitoring
echo -e "\n${BLUE}9. Logging and Monitoring${NC}\n"

# Check if log directories exist
if [ -d "logs" ]; then
    check_pass "Log directory exists"
else
    check_warn "Log directory not found"
fi

# Check if monitoring is enabled
if [ -f ".env" ]; then
    if grep -q "ENABLE_MONITORING=true" .env; then
        check_pass "Monitoring is enabled"
    else
        check_warn "Monitoring is not enabled"
    fi
fi

# Section 10: Backup Configuration
echo -e "\n${BLUE}10. Backup Configuration${NC}\n"

if [ -f ".env" ]; then
    if grep -q "BACKUP_ENABLED=true" .env; then
        check_pass "Backups are enabled"
        
        BACKUP_DIR=$(grep "BACKUP_DIR" .env | cut -d'=' -f2 || echo "backups")
        if [ -d "$BACKUP_DIR" ]; then
            check_pass "Backup directory exists"
        else
            check_warn "Backup directory not found"
        fi
    else
        check_warn "Backups are not enabled"
    fi
fi

# Summary
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}Security Audit Summary${NC}"
echo -e "${BLUE}========================================${NC}\n"

TOTAL=$((PASSED + FAILED + WARNINGS))
echo -e "Total Checks: $TOTAL"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo -e "${YELLOW}Warnings: $WARNINGS${NC}"

if [ $FAILED -gt 0 ]; then
    echo -e "\n${RED}⚠ CRITICAL: $FAILED security issues found!${NC}"
    echo -e "${RED}Please address these issues before deploying to production.${NC}"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "\n${YELLOW}⚠ $WARNINGS warnings found.${NC}"
    echo -e "${YELLOW}Consider addressing these for better security.${NC}"
    exit 0
else
    echo -e "\n${GREEN}✓ All security checks passed!${NC}"
    exit 0
fi