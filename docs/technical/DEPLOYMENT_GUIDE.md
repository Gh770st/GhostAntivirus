# 🚀 GhostAntivirus Deployment Guide

## Production Deployment

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+
- 4GB+ RAM
- SSL certificate

### Quick Deploy

```bash
# Clone repository
git clone https://github.com/Gh770st/GhostAntivirus.git
cd GhostAntivirus

# Configure environment
cp .env.production.example .env.production
# Edit .env.production with your settings

# Deploy
docker-compose -f docker-compose.production.yml up -d
```

### Environment Configuration

```bash
# .env.production
DOMAIN=your-domain.com
EMAIL=admin@your-domain.com
DB_PASSWORD=your-secure-password
API_SECRET=your-api-secret-key
```

### SSL Configuration

```bash
# Generate SSL certificate
certbot certonly --standalone -d your-domain.com

# Configure nginx
# nginx.conf already configured for SSL
```

## Monitoring

### Health Checks
```bash
# Check service status
curl https://your-domain.com/health

# Check metrics
curl https://your-domain.com/metrics
```

### Logging
```bash
# View logs
docker-compose -f docker-compose.production.yml logs -f

# Log rotation configured automatically
```

## Scaling

### Horizontal Scaling
```bash
# Scale scanner service
docker-compose -f docker-compose.production.yml up -d --scale scanner=3
```

### Load Balancing
- nginx configured for load balancing
- Redis for session management
- PostgreSQL for clustering support

## Security

### Firewall Rules
```bash
# Allow only necessary ports
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw enable
```

### Security Headers
nginx configured with security headers:
- HSTS
- X-Frame-Options
- X-Content-Type-Options
