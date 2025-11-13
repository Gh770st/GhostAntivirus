# 🔗 GhostAntivirus Integration Guide

## API Integration

### Authentication
```bash
# Get API token
curl -X POST https://your-domain.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}'
```

### Python Integration
```python
import requests

# Start scan
response = requests.post(
    "https://your-domain.com/api/v1/scan/start",
    headers={"Authorization": "Bearer YOUR_TOKEN"},
    json={"path": "/path/to/scan", "scan_type": "quick"}
)

scan_id = response.json()["scan_id"]

# Get results
results = requests.get(
    f"https://your-domain.com/api/v1/scan/{scan_id}/results",
    headers={"Authorization": "Bearer YOUR_TOKEN"}
)
```

### JavaScript Integration
```javascript
// Start scan
const response = await fetch('/api/v1/scan/start', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    path: '/path/to/scan',
    scan_type: 'quick'
  })
});

const { scan_id } = await response.json();
```

## WebSocket Integration

```javascript
// Connect to WebSocket
const ws = new WebSocket('wss://your-domain.com/ws');

// Listen for events
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data.type, data.payload);
};

// Events:
// scan.progress - Scan progress updates
// threat.detected - New threat detected
// system.status - System status changes
```

## Third-party Integration

### SIEM Integration
```bash
# Forward events to SIEM
curl -X POST https://your-siem.com/api/events \
  -H "Authorization: Bearer SIEM_TOKEN" \
  -H "Content-Type: application/json" \
  -d @ghost_events.json
```

### SOAR Integration
```python
# SOAR playbook integration
def handle_threat(threat_data):
    if threat_data['severity'] == 'critical':
        # Auto-quarantine
        requests.post('/api/v1/quarantine/add', {
            'file_path': threat_data['file_path']
        })
        
        # Notify SOAR platform
        requests.post('https://soar-platform.com/api/incidents', {
            'threat_id': threat_data['id'],
            'severity': threat_data['severity']
        })
```

## SDKs

### Official SDKs
- Python: `pip install ghostantivirus-sdk`
- JavaScript: `npm install ghostantivirus-sdk`
- Go: `go get github.com/ghostantivirus/sdk-go`

### Community Integrations
- Splunk App
- Elastic Stack Integration
- Graylog Plugin
