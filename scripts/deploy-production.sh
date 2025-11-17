#!/bin/bash

# Production Deployment Script for GhostAntivirus
# This script handles the complete production deployment process

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="$PROJECT_DIR/backups"
LOG_FILE="$PROJECT_DIR/deployment.log"

# Functions
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
    exit 1
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

# Header
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}GhostAntivirus Production Deployment${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Step 1: Pre-deployment checks
log "Step 1: Running pre-deployment checks..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    error "Docker is not installed. Please install Docker first."
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    error "Docker Compose is not installed. Please install Docker Compose first."
fi

# Check if .env file exists
if [ ! -f "$PROJECT_DIR/.env" ]; then
    warning ".env file not found. Creating from example..."
    cp "$PROJECT_DIR/.env.production.example" "$PROJECT_DIR/.env"
    warning "Please edit .env file with your production values before continuing."
    read -p "Press Enter to continue after editing .env file..."
fi

# Check if required environment variables are set
source "$PROJECT_DIR/.env"
if [ "$JWT_SECRET" == "your-super-secret-jwt-key-change-this-in-production" ]; then
    error "Please update JWT_SECRET in .env file with a secure value."
fi

success "Pre-deployment checks passed"

# Step 2: Create backup
log "Step 2: Creating backup of current deployment..."

mkdir -p "$BACKUP_DIR"
BACKUP_NAME="backup-$(date +'%Y%m%d-%H%M%S').tar.gz"

if docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" ps | grep -q "Up"; then
    log "Creating backup of running services..."
    docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" exec -T postgres pg_dump -U ghost ghostantivirus > "$BACKUP_DIR/db-backup-$(date +'%Y%m%d-%H%M%S').sql" || warning "Database backup failed"
fi

success "Backup created: $BACKUP_NAME"

# Step 3: Pull latest code
log "Step 3: Pulling latest code..."

cd "$PROJECT_DIR"
if [ -d ".git" ]; then
    git pull origin main || warning "Git pull failed"
    success "Code updated"
else
    warning "Not a git repository, skipping code update"
fi

# Step 4: Build Docker images
log "Step 4: Building Docker images..."

docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" build --no-cache || error "Docker build failed"

success "Docker images built successfully"

# Step 5: Stop existing services
log "Step 5: Stopping existing services..."

if docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" ps | grep -q "Up"; then
    docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" down || error "Failed to stop services"
    success "Services stopped"
else
    log "No running services found"
fi

# Step 6: Start services
log "Step 6: Starting services..."

docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" up -d || error "Failed to start services"

success "Services started"

# Step 7: Wait for services to be healthy
log "Step 7: Waiting for services to be healthy..."

sleep 10

# Check Core Engine
log "Checking Core Engine..."
for i in {1..30}; do
    if curl -s http://localhost:8080/health > /dev/null 2>&1; then
        success "Core Engine is healthy"
        break
    fi
    if [ $i -eq 30 ]; then
        error "Core Engine failed to start"
    fi
    sleep 2
done

# Check AI Engine
log "Checking AI Engine..."
for i in {1..30}; do
    if curl -s http://localhost:8000/health > /dev/null 2>&1; then
        success "AI Engine is healthy"
        break
    fi
    if [ $i -eq 30 ]; then
        error "AI Engine failed to start"
    fi
    sleep 2
done

# Check Network Guard
log "Checking Network Guard..."
for i in {1..30}; do
    if curl -s http://localhost:9000/health > /dev/null 2>&1; then
        success "Network Guard is healthy"
        break
    fi
    if [ $i -eq 30 ]; then
        error "Network Guard failed to start"
    fi
    sleep 2
done

# Check Web Dashboard
log "Checking Web Dashboard..."
for i in {1..30}; do
    if curl -s http://localhost:80 > /dev/null 2>&1; then
        success "Web Dashboard is healthy"
        break
    fi
    if [ $i -eq 30 ]; then
        error "Web Dashboard failed to start"
    fi
    sleep 2
done

# Step 8: Run smoke tests
log "Step 8: Running smoke tests..."

# Test authentication
log "Testing authentication..."
AUTH_RESPONSE=$(curl -s -X POST http://localhost:8080/auth/login \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin123"}')

if echo "$AUTH_RESPONSE" | grep -q "token"; then
    success "Authentication test passed"
else
    error "Authentication test failed"
fi

# Step 9: Display service status
log "Step 9: Displaying service status..."

echo ""
docker-compose -f "$PROJECT_DIR/docker-compose.production.yml" ps

# Step 10: Display access information
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Deployment Complete!${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${GREEN}Services are now running:${NC}"
echo -e "  Core Engine API:  http://localhost:8080"
echo -e "  AI Engine API:    http://localhost:8000"
echo -e "  Network Guard:    http://localhost:9000"
echo -e "  Web Dashboard:    http://localhost:80"
echo -e "  Prometheus:       http://localhost:9090"
echo -e "  Grafana:          http://localhost:3001"
echo ""

echo -e "${YELLOW}Important Notes:${NC}"
echo -e "  1. Update DNS records to point to this server"
echo -e "  2. Configure SSL certificates for HTTPS"
echo -e "  3. Set up firewall rules"
echo -e "  4. Configure backup schedule"
echo -e "  5. Set up monitoring alerts"
echo ""

echo -e "${YELLOW}Useful Commands:${NC}"
echo -e "  View logs:        docker-compose -f docker-compose.production.yml logs -f"
echo -e "  Stop services:    docker-compose -f docker-compose.production.yml down"
echo -e "  Restart services: docker-compose -f docker-compose.production.yml restart"
echo -e "  Update services:  ./scripts/deploy-production.sh"
echo ""

log "Deployment completed successfully!"