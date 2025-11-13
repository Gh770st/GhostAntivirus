# 📦 GhostAntivirus Installation Guide

## System Requirements

- OS: Linux, macOS, Windows
- RAM: 4GB minimum
- Storage: 20GB available space
- Docker: Latest version (recommended)

## Quick Installation

### Option 1: Docker (Recommended)
```bash
git clone https://github.com/Gh770st/GhostAntivirus.git
cd GhostAntivirus
docker-compose up -d
```

### Option 2: Native Installation
```bash
# Install dependencies
./scripts/install-dependencies.sh

# Build components
./scripts/build-all.sh

# Start services
./scripts/start-all.sh
```

## Configuration

Edit `config/config.yaml` to customize settings:

```yaml
server:
  host: "0.0.0.0"
  port: 8080

database:
  type: "sqlite"
  path: "./data/ghost.db"

scanner:
  max_concurrent_scans: 5
  default_scan_type: "quick"
```

## Verification

Access the dashboard at http://localhost:3000 to verify installation.

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues.
