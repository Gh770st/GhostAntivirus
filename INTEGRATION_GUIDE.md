# 🔗 GhostAntivirus Integration Guide

## Overview
This guide explains how to integrate and test all GhostAntivirus components together.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Web Dashboard                          │
│                   (React + TypeScript)                      │
│                    Port: 3000 / 80                          │
└────────────┬────────────────────────────────────────────────┘
             │
             │ HTTP/WebSocket
             │
┌────────────┴────────────────────────────────────────────────┐
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Core Engine  │  │  AI Engine   │  │Network Guard │     │
│  │   (Rust)     │  │  (Python)    │  │    (Go)      │     │
│  │  Port: 8080  │  │  Port: 8000  │  │  Port: 9000  │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Core Engine (Rust)
- **Port:** 8080
- **Endpoints:** 40+ REST API endpoints
- **WebSocket:** Real-time updates on `/ws`
- **Features:**
  - File scanning
  - Threat detection
  - Quarantine management
  - Firewall integration
  - System monitoring

### 2. AI Engine (Python)
- **Port:** 8000
- **Features:**
  - Machine learning threat detection
  - Feature extraction
  - Model training
  - Threat classification

### 3. Network Guard (Go)
- **Port:** 9000
- **Features:**
  - Network monitoring
  - Firewall rules
  - VPN management
  - Traffic analysis

### 4. Web Dashboard (React)
- **Port:** 3000 (dev) / 80 (prod)
- **Features:**
  - User interface
  - Real-time updates
  - Settings management
  - Visualization

---

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Python 3.8+ (for testing)
- Node.js 18+ (for development)
- Rust 1.70+ (for development)
- Go 1.20+ (for development)

### Option 1: Docker Compose (Recommended)

```bash
# Start all services
docker-compose -f docker-compose.integration.yml up -d

# Check status
docker-compose -f docker-compose.integration.yml ps

# View logs
docker-compose -f docker-compose.integration.yml logs -f

# Stop services
docker-compose -f docker-compose.integration.yml down
```

### Option 2: Run Integration Tests

```bash
# Run automated integration tests
./scripts/test-integration.sh
```

### Option 3: Manual Testing

```bash
# Terminal 1: Core Engine
cd core
cargo run --release

# Terminal 2: AI Engine
cd ai-engine
python src/main.py

# Terminal 3: Network Guard
cd network-guard
go run main.go

# Terminal 4: Web Dashboard
cd web-ui
npm run dev
```

---

## API Endpoints

### Core Engine (http://localhost:8080)

#### Authentication
- `POST /auth/login` - Login
- `POST /auth/logout` - Logout
- `POST /auth/refresh` - Refresh token

#### Scanner
- `GET /api/scan/stats` - Get scan statistics
- `POST /api/scan/start` - Start scan
- `POST /api/scan/stop` - Stop scan
- `POST /api/scan/pause` - Pause scan
- `POST /api/scan/resume` - Resume scan
- `GET /api/scan/results` - Get scan results

#### Threats
- `GET /api/threats` - List threats
- `GET /api/threats/:id` - Get threat details
- `POST /api/threats/:id/quarantine` - Quarantine threat
- `DELETE /api/threats/:id/remove` - Remove threat
- `POST /api/threats/:id/restore` - Restore threat

#### Quarantine
- `GET /api/quarantine` - List quarantined files
- `GET /api/quarantine/:id` - Get file details
- `POST /api/quarantine/:id/restore` - Restore file
- `DELETE /api/quarantine/:id/delete` - Delete file
- `GET /api/quarantine/stats` - Get statistics

#### Firewall
- `GET /api/firewall/rules` - List rules
- `POST /api/firewall/rules` - Add rule
- `PUT /api/firewall/rules/:id` - Update rule
- `DELETE /api/firewall/rules/:id` - Delete rule
- `GET /api/firewall/stats` - Get statistics

#### Network
- `GET /api/network/connections` - List connections
- `GET /api/network/stats` - Get statistics
- `POST /api/network/scan` - Scan network

#### Settings
- `GET /api/settings` - Get settings
- `PUT /api/settings` - Update settings

#### System
- `GET /api/system/info` - Get system info
- `GET /api/system/stats` - Get system stats

#### Updates
- `GET /api/updates/check` - Check for updates
- `POST /api/updates/apply` - Apply update

#### WebSocket
- `GET /ws` - WebSocket connection for real-time updates

#### Health
- `GET /health` - Health check

---

## Testing

### 1. Health Check
```bash
curl http://localhost:8080/health
```

### 2. Login
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

### 3. Get Scan Stats (with token)
```bash
curl http://localhost:8080/api/scan/stats \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### 4. Start Scan
```bash
curl -X POST http://localhost:8080/api/scan/start \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"path":"/tmp","scan_type":"quick","deep_scan":false}'
```

### 5. Run Python Integration Tests
```bash
cd integration-tests
python3 test_api.py
```

---

## Environment Variables

### Core Engine
```bash
RUST_LOG=info
API_HOST=0.0.0.0
API_PORT=8080
JWT_SECRET=your-secret-key
```

### AI Engine
```bash
PYTHONUNBUFFERED=1
API_HOST=0.0.0.0
API_PORT=8000
```

### Network Guard
```bash
API_HOST=0.0.0.0
API_PORT=9000
```

### Web Dashboard
```bash
VITE_CORE_ENGINE_URL=http://localhost:8080
VITE_AI_ENGINE_URL=http://localhost:8000
VITE_NETWORK_GUARD_URL=http://localhost:9000
VITE_WS_URL=ws://localhost:8080/ws
```

---

## WebSocket Integration

### Connect to WebSocket
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  console.log('Connected to WebSocket');
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = () => {
  console.log('WebSocket closed');
};
```

### WebSocket Events
- `scan_progress` - Scan progress updates
- `threat_detected` - New threat detected
- `threat_removed` - Threat removed
- `network_alert` - Network alert
- `system_update` - System update

---

## Troubleshooting

### Services Not Starting
```bash
# Check Docker logs
docker-compose -f docker-compose.integration.yml logs

# Check individual service
docker-compose -f docker-compose.integration.yml logs core-engine
```

### Port Already in Use
```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>
```

### Authentication Issues
- Default credentials: `admin` / `admin123`
- Token expires in 1 hour
- Use refresh token to get new access token

### CORS Issues
- CORS is enabled by default
- All origins are allowed in development
- Configure specific origins in production

---

## Production Deployment

### 1. Build Production Images
```bash
docker-compose -f docker-compose.yml build
```

### 2. Configure Environment
```bash
# Copy and edit environment files
cp .env.example .env
nano .env
```

### 3. Deploy
```bash
docker-compose up -d
```

### 4. Monitor
```bash
# View logs
docker-compose logs -f

# Check health
curl http://your-domain.com/health
```

---

## Security Considerations

### 1. Change Default Credentials
```bash
# Update in production
JWT_SECRET=your-secure-random-secret
```

### 2. Enable HTTPS
```bash
# Use reverse proxy (nginx/traefik)
# Configure SSL certificates
```

### 3. Firewall Rules
```bash
# Only expose necessary ports
# Use internal network for service communication
```

### 4. Rate Limiting
```bash
# Configure rate limiting in middleware
# Protect against DDoS attacks
```

---

## Performance Optimization

### 1. Database
- Use PostgreSQL for production
- Configure connection pooling
- Enable query caching

### 2. Caching
- Redis for session storage
- Cache API responses
- Use CDN for static assets

### 3. Load Balancing
- Multiple Core Engine instances
- Load balancer (nginx/HAProxy)
- Health checks enabled

---

## Monitoring

### 1. Logs
```bash
# Centralized logging
docker-compose logs -f > logs/app.log
```

### 2. Metrics
- Prometheus for metrics collection
- Grafana for visualization
- Alert manager for notifications

### 3. Health Checks
- Regular health check endpoints
- Service status monitoring
- Automatic restart on failure

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/witerdev/GhostAntivirus/issues
- Documentation: https://docs.ghostantivirus.com
- Email: support@ghostantivirus.com

---

## License

MIT License - See LICENSE file for details