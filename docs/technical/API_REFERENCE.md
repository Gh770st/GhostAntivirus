# 📚 GhostAntivirus API Reference

## Core Engine API

### Scanner Endpoints

#### Start Scan
```
POST /api/v1/scan/start
Content-Type: application/json

{
  "path": "/path/to/scan",
  "scan_type": "quick|full|custom"
}
```

#### Get Scan Status
```
GET /api/v1/scan/{scan_id}/status
```

#### Get Scan Results
```
GET /api/v1/scan/{scan_id}/results
```

## AI Engine API

### Analysis Endpoints

#### Analyze File
```
POST /api/v1/ai/analyze
Content-Type: application/json

{
  "file_path": "/path/to/file"
}
```

#### Get Threat Info
```
GET /api/v1/ai/threat/{threat_id}
```

## Network Guard API

### Network Endpoints

#### Block IP
```
POST /api/v1/network/block
Content-Type: application/json

{
  "ip_address": "192.168.1.100",
  "reason": "malicious activity"
}
```

## WebSocket Events

### Real-time Updates
- `scan.progress` - Scan progress updates
- `threat.detected` - New threat detected
- `system.status` - System status changes
