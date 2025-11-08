# GhostAntivirus Deployment Guide

## 🚀 Quick Start

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+
- 4GB+ RAM
- 20GB+ disk space
- Linux/macOS/Windows with WSL2

### One-Command Deployment
```bash
# Clone the repository
git clone https://github.com/your-org/GhostAntivirus.git
cd GhostAntivirus

# Deploy everything
docker-compose -f deploy/docker-compose.yml up -d

# Check status
docker-compose ps
```

## 📋 Detailed Deployment

### 1. System Preparation

#### System Requirements
```
Minimum:
- CPU: 2 cores
- RAM: 4GB
- Disk: 20GB
- Network: 100Mbps

Recommended:
- CPU: 4+ cores
- RAM: 8GB+
- Disk: 50GB+
- Network: 1Gbps
```

#### Install Docker (Ubuntu/Debian)
```bash
# Update packages
sudo apt-get update

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo apt-get install docker-compose-plugin

# Add user to docker group
sudo usermod -aG docker $USER

# Reboot or log out/in
newgrp docker
```

### 2. Configuration

#### Environment Variables
```bash
# Copy environment template
cp deploy/.env.example deploy/.env

# Edit configuration
nano deploy/.env
```

#### Key Configuration Options
```bash
# Database Configuration
POSTGRES_PASSWORD=your_secure_password
POSTGRES_USER=ghost
POSTGRES_DB=ghost_ai

# Redis Configuration
REDIS_PASSWORD=your_redis_password

# API Keys
AI_API_KEY=your_api_key_here
JWT_SECRET=your_jwt_secret_here

# SSL Configuration (optional)
SSL_CERT_PATH=/path/to/cert.pem
SSL_KEY_PATH=/path/to/key.pem

# Monitoring
GRAFANA_ADMIN_PASSWORD=your_admin_password
```

#### Advanced Configuration
```yaml
# deploy/docker-compose.override.yml
version: '3.8'

services:
  ghost-core:
    environment:
      - RUST_LOG=debug
      - GHOST_CORE_MAX_THREADS=8
    volumes:
      - ./custom-config:/etc/ghost-antivirus:ro
    deploy:
      resources:
        limits:
          cpus: '2.0'
          memory: 2G
        reservations:
          cpus: '1.0'
          memory: 1G

  ghost-ai:
    environment:
      - AI_LOG_LEVEL=DEBUG
      - AI_BATCH_SIZE=200
    volumes:
      - ./custom-models:/app/models:ro
    deploy:
      resources:
        limits:
          cpus: '4.0'
          memory: 4G
        reservations:
          cpus: '2.0'
          memory: 2G
```

### 3. SSL/TLS Setup

#### Generate Self-Signed Certificate
```bash
# Create SSL directory
mkdir -p deploy/ssl
cd deploy/ssl

# Generate private key
openssl genrsa -out key.pem 2048

# Generate certificate
openssl req -new -x509 -key key.pem -out cert.pem -days 365 \
  -subj "/C=US/ST=State/L=City/O=Organization/CN=ghostantivirus.com"

# Set permissions
chmod 600 key.pem
chmod 644 cert.pem
```

#### Use Let's Encrypt (Production)
```bash
# Install Certbot
sudo apt-get install certbot

# Generate certificate
sudo certbot certonly --standalone -d ghostantivirus.com

# Copy certificates
sudo cp /etc/letsencrypt/live/ghostantivirus.com/fullchain.pem deploy/ssl/cert.pem
sudo cp /etc/letsencrypt/live/ghostantivirus.com/privkey.pem deploy/ssl/key.pem
```

### 4. Deployment Steps

#### Step 1: Initialize Environment
```bash
# Create necessary directories
mkdir -p deploy/data/{postgres,redis,logs}
mkdir -p deploy/models

# Set permissions
chmod 755 deploy/data
chmod 700 deploy/data/postgres
chmod 700 deploy/data/redis
```

#### Step 2: Start Services
```bash
# Start core services first (database, cache)
docker-compose -f deploy/docker-compose.yml up -d postgres redis

# Wait for services to be ready
docker-compose logs -f postgres &
docker-compose logs -f redis &

# Start application services
docker-compose -f deploy/docker-compose.yml up -d ghost-ai ghost-core

# Start supporting services
docker-compose -f deploy/docker-compose.yml up -d nginx ghost-web

# Start monitoring (optional)
docker-compose -f deploy/docker-compose.yml up -d prometheus grafana
```

#### Step 3: Verify Deployment
```bash
# Check all services status
docker-compose ps

# Check service logs
docker-compose logs -f ghost-ai
docker-compose logs -f ghost-core

# Health checks
curl http://localhost:8000/health  # AI Engine
curl http://localhost:9000/health  # Core Engine
curl https://localhost/health     # Frontend (if SSL)
```

### 5. Service Configuration

#### GhostAntivirus AI Engine
```yaml
# Custom AI Engine config
environment:
  - AI_MODEL_PATH=/app/models/threat_classifier.h5
  - AI_CONFIDENCE_THRESHOLD=0.7
  - AI_MAX_FILE_SIZE=100MB
  - AI_RATE_LIMIT=100
```

#### GhostAntivirus Core Engine
```yaml
# Custom Core Engine config
environment:
  - RUST_LOG=info
  - GHOST_CORE_SCAN_THREADS=4
  - GHOST_CORE_QUARANTINE_PATH=/var/lib/ghost-antivirus/quarantine
  - GHOST_CORE_MAX_MEMORY=1GB
```

### 6. Monitoring Setup

#### Access Monitoring Interfaces
```bash
# Grafana Dashboard
# URL: http://localhost:3001
# Username: admin
# Password: from environment variable

# Prometheus Metrics
# URL: http://localhost:9090

# Application Logs
docker-compose logs -f ghost-ai ghost-core
```

#### Configure Alerting
```yaml
# deploy/grafana/alerting.yml
apiVersion: 1

alerting:
  contact_points:
    - orgId: 1
      name: 'email_alerts'
      receivers:
        - uid: 'email_receiver'
          type: email
          settings:
            addresses: ['admin@ghostantivirus.com']
```

## 🔧 Maintenance & Operations

### Updates and Upgrades

#### Update GhostAntivirus
```bash
# Pull latest images
docker-compose -f deploy/docker-compose.yml pull

# Restart with new images
docker-compose -f deploy/docker-compose.yml up -d

# Check status
docker-compose ps
```

#### Database Maintenance
```bash
# Backup database
docker-compose exec postgres pg_dump -U ghost ghost_ai > backup_$(date +%Y%m%d).sql

# Restore database
docker-compose exec -T postgres psql -U ghost ghost_ai < backup_20231201.sql

# Clean up old data
docker-compose exec postgres psql -U ghost -c "DELETE FROM scan_results WHERE created_at < NOW() - INTERVAL '30 days';"
```

### Performance Tuning

#### Optimize Docker Resources
```yaml
# deploy/docker-compose.prod.yml
version: '3.8'

services:
  ghost-ai:
    deploy:
      resources:
        limits:
          cpus: '4.0'
          memory: 4G
        reservations:
          cpus: '2.0'
          memory: 2G
    environment:
      - AI_WORKERS=4
      - AI_BATCH_SIZE=500
```

#### Database Optimization
```sql
-- PostgreSQL performance tuning
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
SELECT pg_reload_conf();
```

### Troubleshooting

#### Common Issues

**Service won't start**
```bash
# Check logs
docker-compose logs service_name

# Check resource usage
docker stats

# Restart service
docker-compose restart service_name
```

**High memory usage**
```bash
# Check memory usage
docker stats --no-stream

# Clean up unused images
docker image prune -a

# Restart services
docker-compose restart
```

**Database connection issues**
```bash
# Check database status
docker-compose exec postgres pg_isready

# Test connection
docker-compose exec ghost-ai python -c "import psycopg2; conn = psycopg2.connect('postgresql://ghost:password@postgres:5432/ghost_ai'); print('Connected')"
```

#### Log Analysis
```bash
# View real-time logs
docker-compose logs -f

# Filter logs
docker-compose logs ghost-ai | grep ERROR

# Export logs
docker-compose logs --no-color > ghost-antivirus.log
```

## 🔒 Security Configuration

### Production Security Checklist

#### Network Security
- [ ] Firewall configured
- [ ] Only necessary ports exposed
- [ ] SSL/TLS enabled
- [ ] Rate limiting configured
- [ ] Access control implemented

#### Application Security
- [ ] API keys configured
- [ ] JWT secrets set
- [ ] Database credentials strong
- [ ] File upload limits set
- [ ] Input validation enabled

#### System Security
- [ ] Non-root users in containers
- [ ] Read-only filesystems where possible
- [ ] Resource limits configured
- [ ] Logging enabled
- [ ] Monitoring configured

### Security Hardening

#### Docker Security
```yaml
# deploy/docker-compose.security.yml
services:
  ghost-core:
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    user: "1000:1000"
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE

  ghost-ai:
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
      - /app/logs
    user: "1000:1000"
    cap_drop:
      - ALL
```

#### System Hardening
```bash
# Configure firewall
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable

# Configure fail2ban
sudo apt-get install fail2ban
sudo systemctl enable fail2ban
```

## 📊 Scaling

### Horizontal Scaling

#### Load Balancer Configuration
```nginx
# deploy/nginx-lb.conf
upstream ghost_ai_cluster {
    server ghost-ai-1:8000;
    server ghost-ai-2:8000;
    server ghost-ai-3:8000;
}

server {
    listen 443 ssl;
    location /api/ai/ {
        proxy_pass http://ghost_ai_cluster;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

#### Multi-Node Deployment
```bash
# Scale AI Engine
docker-compose up -d --scale ghost-ai=3

# Scale Core Engine
docker-compose up -d --scale ghost-core=2
```

### Performance Optimization

#### Caching Strategy
```yaml
# Redis caching
redis:
  image: redis:7-alpine
  command: redis-server --maxmemory 1gb --maxmemory-policy allkeys-lru
```

#### Database Optimization
```sql
-- Indexes for performance
CREATE INDEX idx_scan_results_created_at ON scan_results(created_at);
CREATE INDEX idx_threats_severity ON threats(severity);
CREATE INDEX idx_files_hash ON files(sha256_hash);
```

## 🚀 Production Deployment

### Cloud Deployment (AWS)

#### ECS Deployment
```bash
# Create ECS cluster
aws ecs create-cluster --cluster-name ghost-antivirus

# Deploy stack
aws cloudformation deploy \
  --template-file deploy/cloudformation.yaml \
  --stack-name ghost-antivirus \
  --capabilities CAPABILITY_IAM
```

#### Kubernetes Deployment
```yaml
# deploy/kubernetes/ghost-antivirus.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ghost-ai
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ghost-ai
  template:
    metadata:
      labels:
        app: ghost-ai
    spec:
      containers:
      - name: ghost-ai
        image: ghcr.io/your-org/ghost-antivirus-ai:latest
        ports:
        - containerPort: 8000
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
```

## 📞 Support

### Getting Help
- 📖 Documentation: `https://docs.ghostantivirus.com`
- 🐛 Issues: `https://github.com/your-org/GhostAntivirus/issues`
- 💬 Discord: `https://discord.gg/ghostantivirus`
- 📧 Email: `support@ghostantivirus.com`

### Monitoring Alerts
Set up alerts for:
- Service downtime
- High error rates
- Resource exhaustion
- Security incidents
- Performance degradation

### Backup Strategy
- Daily database backups
- Weekly configuration backups
- Monthly full system backups
- Off-site backup storage
- Backup restoration testing

---

**🎉 Congratulations! Your GhostAntivirus system is now deployed and ready to protect your infrastructure!**