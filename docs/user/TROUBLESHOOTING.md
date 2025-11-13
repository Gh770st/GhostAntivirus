# 🔧 GhostAntivirus Troubleshooting Guide

## Common Issues

### Service Won't Start

**Problem**: Services fail to start
**Solution**: 
```bash
# Check ports
netstat -tulpn | grep :8080

# Check logs
docker-compose logs

# Reset services
docker-compose down && docker-compose up -d
```

### High Memory Usage

**Problem**: Memory usage is excessive
**Solution**:
```bash
# Check memory usage
docker stats

# Reduce concurrent scans
# Edit config.yaml:
scanner:
  max_concurrent_scans: 2
```

### Slow Scanning

**Problem**: Scans are running slowly
**Solution**:
```bash
# Check disk I/O
iostat -x 1

# Exclude large directories
# Add to config.yaml:
scanner:
  exclude_paths:
    - "/node_modules"
    - "/.git"
```

## Getting Help

- Check logs: `docker-compose logs -f`
- Review configuration: `config/config.yaml`
- Open issue: [GitHub Issues](https://github.com/Gh770st/GhostAntivirus/issues)
