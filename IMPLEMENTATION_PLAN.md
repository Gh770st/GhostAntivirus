# 📋 Szczegółowy Plan Implementacji - GhostAntivirus

## 🎯 Cel: Doprowadzenie projektu do prawdziwej gotowości produkcyjnej

**Czas realizacji:** 3 tygodnie (56-68 godzin)
**Status początkowy:** 75-80% (z mock data)
**Status docelowy:** 100% (w pełni funkcjonalny)

---

## 📅 TYDZIEŃ 1: Krytyczne Implementacje (40 godzin)

### 🔴 DZIEŃ 1: Core Engine - Rzeczywiste Skanowanie (8h)

#### Godzina 1-2: Przygotowanie środowiska
```bash
# Zadanie 1.1: Setup Rust environment
cd GhostAntivirus/core
cargo check
cargo build

# Zadanie 1.2: Analiza istniejącego kodu
# - Przejrzyj scanner.rs
# - Przejrzyj api/handlers/scanner.rs
# - Zidentyfikuj wszystkie TODO
```

#### Godzina 3-5: Implementacja rzeczywistego skanowania
```rust
// Plik: core/src/api/handlers/scanner.rs

// Zadanie 1.3: Implementacja get_stats
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // ZMIANA: Pobierz rzeczywiste statystyki
    let scanner_stats = engine.scanner.get_statistics();
    
    let stats = ScanStatsResponse {
        total_scanned: scanner_stats.total_files_scanned,
        threats_found: scanner_stats.threats_detected,
        files_quarantined: scanner_stats.files_quarantined,
        last_scan: scanner_stats.last_scan_time,
        scan_in_progress: scanner_stats.is_scanning,
        scan_progress: scanner_stats.progress_percentage,
    };

    success_response(stats)
}

// Zadanie 1.4: Implementacja start_scan
pub async fn start_scan(
    State(state): State<AppState>,
    Json(payload): Json<StartScanRequest>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // ZMIANA: Rzeczywiste uruchomienie skanowania
    let scan_result = match payload.scan_type {
        ScanType::Quick => {
            engine.scanner.start_quick_scan(&payload.path, payload.deep_scan).await
        }
        ScanType::Full => {
            engine.scanner.start_full_scan(&payload.path, payload.deep_scan).await
        }
        ScanType::Custom => {
            engine.scanner.start_custom_scan(&payload.path, payload.deep_scan).await
        }
    };

    match scan_result {
        Ok(scan_id) => success_response(serde_json::json!({
            "scan_id": scan_id,
            "message": "Scan started successfully",
            "path": payload.path,
        })),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
```

#### Godzina 6-7: Implementacja kontroli skanowania
```rust
// Zadanie 1.5: Stop scan
pub async fn stop_scan(State(state): State<AppState>) -> Response {
    let mut engine = state.engine.write().await;
    
    match engine.scanner.stop_scan().await {
        Ok(_) => success_response(serde_json::json!({
            "message": "Scan stopped successfully"
        })),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

// Zadanie 1.6: Pause/Resume scan
pub async fn pause_scan(State(state): State<AppState>) -> Response {
    let mut engine = state.engine.write().await;
    
    match engine.scanner.pause_scan().await {
        Ok(_) => success_response(serde_json::json!({
            "message": "Scan paused successfully"
        })),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
```

#### Godzina 8: Testy i weryfikacja
```bash
# Zadanie 1.7: Uruchom testy
cargo test scanner

# Zadanie 1.8: Weryfikacja API
curl -X POST http://localhost:8080/api/scan/start \
  -H "Content-Type: application/json" \
  -d '{"path":"/tmp","scan_type":"quick","deep_scan":false}'
```

**Deliverables Dzień 1:**
- ✅ Rzeczywiste skanowanie plików
- ✅ Prawdziwe statystyki
- ✅ Kontrola skanowania (start/stop/pause/resume)
- ✅ Testy jednostkowe

---

### 🔴 DZIEŃ 2: Core Engine - Zagrożenia i Kwarantanna (8h)

#### Godzina 1-3: Operacje na zagrożeniach
```rust
// Plik: core/src/api/handlers/threats.rs

// Zadanie 2.1: Lista zagrożeń
pub async fn list_threats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // ZMIANA: Pobierz rzeczywiste zagrożenia
    let threats = engine.analyzer.get_detected_threats();
    
    let threat_list: Vec<ThreatInfo> = threats.iter().map(|t| {
        ThreatInfo {
            id: t.id.clone(),
            name: t.name.clone(),
            path: t.file_path.clone(),
            threat_type: t.threat_type.clone(),
            severity: map_severity(&t.severity),
            detected_at: t.detected_at,
            status: map_status(&t.status),
            hash: t.file_hash.clone(),
            size: t.file_size,
        }
    }).collect();

    let response = ThreatListResponse {
        threats: threat_list,
        total: threats.len(),
    };

    success_response(response)
}

// Zadanie 2.2: Kwarantanna zagrożenia
pub async fn quarantine_threat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    match engine.quarantine.quarantine_file_by_threat_id(&id).await {
        Ok(_) => success_response(serde_json::json!({
            "message": "Threat quarantined successfully",
            "threat_id": id,
        })),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
```

#### Godzina 4-6: Operacje na kwarantannie
```rust
// Plik: core/src/api/handlers/quarantine.rs

// Zadanie 2.3: Lista plików w kwarantannie
pub async fn list_files(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    let quarantine_files = engine.quarantine.list_files();
    
    let files: Vec<QuarantineFile> = quarantine_files.iter().map(|f| {
        QuarantineFile {
            id: f.id.clone(),
            original_path: f.original_path.clone(),
            quarantine_path: f.quarantine_path.clone(),
            quarantined_at: f.quarantined_at,
            threat_type: f.threat_type.clone(),
            size: f.size,
            hash: f.hash.clone(),
        }
    }).collect();

    let response = QuarantineListResponse {
        files,
        total: quarantine_files.len(),
    };

    success_response(response)
}

// Zadanie 2.4: Przywracanie pliku
pub async fn restore_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    match engine.quarantine.restore_file(&id).await {
        Ok(restored_path) => success_response(serde_json::json!({
            "message": "File restored successfully",
            "file_id": id,
            "restored_path": restored_path,
        })),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
```

#### Godzina 7-8: Testy i integracja
```bash
# Zadanie 2.5: Testy
cargo test threats
cargo test quarantine

# Zadanie 2.6: Integracja end-to-end
# Test flow: Scan -> Detect -> Quarantine -> Restore
```

**Deliverables Dzień 2:**
- ✅ Rzeczywiste operacje na zagrożeniach
- ✅ Rzeczywiste operacje na kwarantannie
- ✅ Integracja z analyzer i quarantine manager
- ✅ Testy end-to-end

---

### 🔴 DZIEŃ 3: AI Engine Integration (8h)

#### Godzina 1-3: AI Engine API
```python
# Plik: ai-engine/src/api.py

# Zadanie 3.1: Implementacja autentykacji
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials

security = HTTPBearer()

async def verify_token(credentials: HTTPAuthorizationCredentials = Depends(security)):
    token = credentials.credentials
    # Weryfikacja JWT token
    if not verify_jwt_token(token):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid authentication credentials"
        )
    return token

# Zadanie 3.2: Rzeczywista logika skanowania
@app.post("/api/analyze")
async def analyze_file(
    file_path: str,
    token: str = Depends(verify_token)
):
    try:
        # Rzeczywista analiza pliku
        result = await ai_engine.analyze_file(Path(file_path))
        
        # Tracking
        metrics.files_processed += 1
        if result.is_threat:
            metrics.threats_detected += 1
        
        return {
            "success": True,
            "result": result.to_dict(),
            "timestamp": datetime.now().isoformat()
        }
    except Exception as e:
        logger.error(f"Analysis failed: {e}")
        return {
            "success": False,
            "error": str(e)
        }
```

#### Godzina 4-6: Metryki i monitoring
```python
# Zadanie 3.3: Rzeczywiste metryki systemowe
import psutil
import time

class SystemMetrics:
    def __init__(self):
        self.start_time = time.time()
        self.files_processed = 0
        self.threats_detected = 0
    
    def get_metrics(self):
        return {
            "uptime": time.time() - self.start_time,
            "cpu_usage": psutil.cpu_percent(interval=1),
            "memory_usage": psutil.virtual_memory().percent,
            "disk_usage": psutil.disk_usage('/').percent,
            "files_processed": self.files_processed,
            "threats_detected": self.threats_detected,
        }

metrics = SystemMetrics()

@app.get("/api/metrics")
async def get_metrics():
    return metrics.get_metrics()
```

#### Godzina 7-8: Integracja z Core Engine
```rust
// Plik: core/src/scanner.rs

// Zadanie 3.4: Wywołanie AI Engine
async fn analyze_with_ai(&self, file_path: &Path) -> Result<AIAnalysisResult> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(&format!("{}/api/analyze", self.ai_engine_url))
        .header("Authorization", format!("Bearer {}", self.ai_token))
        .json(&json!({
            "file_path": file_path.to_str().unwrap()
        }))
        .send()
        .await?;
    
    let result: AIAnalysisResult = response.json().await?;
    Ok(result)
}
```

**Deliverables Dzień 3:**
- ✅ Autentykacja AI Engine
- ✅ Rzeczywista logika skanowania
- ✅ Metryki systemowe
- ✅ Integracja Core Engine <-> AI Engine

---

### 🔴 DZIEŃ 4: Network Guard Implementation (8h)

#### Godzina 1-4: Firewall Rules
```go
// Plik: network-guard/firewall.go

// Zadanie 4.1: Implementacja firewall manager
type FirewallManager struct {
    rules []FirewallRule
    mu    sync.RWMutex
}

func (fm *FirewallManager) AddRule(rule FirewallRule) error {
    fm.mu.Lock()
    defer fm.mu.Unlock()
    
    // Walidacja reguły
    if err := validateRule(rule); err != nil {
        return err
    }
    
    // Dodaj regułę
    rule.ID = generateID()
    rule.CreatedAt = time.Now()
    fm.rules = append(fm.rules, rule)
    
    // Zastosuj regułę w systemie
    if err := applyFirewallRule(rule); err != nil {
        return err
    }
    
    return nil
}

func (fm *FirewallManager) CheckConnection(conn NetworkConnection) bool {
    fm.mu.RLock()
    defer fm.mu.RUnlock()
    
    // Sprawdź wszystkie reguły
    for _, rule := range fm.rules {
        if rule.Enabled && matchesRule(conn, rule) {
            return rule.Action == "allow"
        }
    }
    
    // Domyślna akcja
    return false
}
```

#### Godzina 5-7: Network Monitoring
```go
// Zadanie 4.2: Monitoring połączeń
type NetworkMonitor struct {
    connections []NetworkConnection
    stats       NetworkStats
    mu          sync.RWMutex
}

func (nm *NetworkMonitor) Start() error {
    go nm.monitorLoop()
    return nil
}

func (nm *NetworkMonitor) monitorLoop() {
    ticker := time.NewTicker(5 * time.Second)
    defer ticker.Stop()
    
    for range ticker.C {
        // Pobierz aktywne połączenia
        connections, err := getActiveConnections()
        if err != nil {
            log.Printf("Error getting connections: %v", err)
            continue
        }
        
        nm.mu.Lock()
        nm.connections = connections
        nm.updateStats()
        nm.mu.Unlock()
    }
}
```

#### Godzina 8: API Endpoints
```go
// Zadanie 4.3: REST API
func setupRoutes(r *gin.Engine, ng *NetworkGuard) {
    api := r.Group("/api")
    
    api.GET("/firewall/rules", ng.GetRules)
    api.POST("/firewall/rules", ng.AddRule)
    api.PUT("/firewall/rules/:id", ng.UpdateRule)
    api.DELETE("/firewall/rules/:id", ng.DeleteRule)
    
    api.GET("/network/connections", ng.GetConnections)
    api.GET("/network/stats", ng.GetStats)
}
```

**Deliverables Dzień 4:**
- ✅ Firewall rules management
- ✅ Network monitoring
- ✅ Connection tracking
- ✅ REST API endpoints

---

### 🔴 DZIEŃ 5: Web Dashboard Integration (8h)

#### Godzina 1-3: API Integration
```typescript
// Plik: web-ui/src/services/api.ts

// Zadanie 5.1: Rzeczywiste wywołania API
export class ApiClient {
    private baseURL: string;
    private token: string | null = null;

    async getScanStats(): Promise<ScanStatsResponse> {
        const response = await this.get('/api/scan/stats');
        return response.data;
    }

    async startScan(request: StartScanRequest): Promise<any> {
        const response = await this.post('/api/scan/start', request);
        return response.data;
    }

    private async get(endpoint: string) {
        const response = await fetch(`${this.baseURL}${endpoint}`, {
            headers: this.getHeaders(),
        });
        
        if (!response.ok) {
            throw new Error(`API Error: ${response.statusText}`);
        }
        
        return await response.json();
    }

    private getHeaders() {
        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };
        
        if (this.token) {
            headers['Authorization'] = `Bearer ${this.token}`;
        }
        
        return headers;
    }
}
```

#### Godzina 4-6: Component Updates
```typescript
// Plik: web-ui/src/pages/Scanner.tsx

// Zadanie 5.2: Użycie prawdziwego API
const Scanner: React.FC = () => {
    const [stats, setStats] = useState<ScanStats | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        loadStats();
        const interval = setInterval(loadStats, 5000);
        return () => clearInterval(interval);
    }, []);

    const loadStats = async () => {
        try {
            const data = await apiClient.getScanStats();
            setStats(data);
            setError(null);
        } catch (err) {
            setError('Failed to load scan statistics');
            console.error(err);
        }
    };

    const handleStartScan = async (path: string, type: ScanType) => {
        setLoading(true);
        try {
            await apiClient.startScan({ path, scan_type: type, deep_scan: false });
            await loadStats();
        } catch (err) {
            setError('Failed to start scan');
        } finally {
            setLoading(false);
        }
    };

    // ... rest of component
};
```

#### Godzina 7-8: Error Handling & Loading States
```typescript
// Zadanie 5.3: Error handling
const ErrorBoundary: React.FC = ({ children }) => {
    const [hasError, setHasError] = useState(false);

    if (hasError) {
        return (
            <Alert severity="error">
                Something went wrong. Please try again.
            </Alert>
        );
    }

    return <>{children}</>;
};

// Zadanie 5.4: Loading states
const LoadingSpinner: React.FC = () => (
    <Box display="flex" justifyContent="center" p={4}>
        <CircularProgress />
    </Box>
);
```

**Deliverables Dzień 5:**
- ✅ Integracja z prawdziwym API
- ✅ Error handling
- ✅ Loading states
- ✅ Real-time updates

---

## 📅 TYDZIEŃ 2: Testy i Skrypty (40 godzin)

### 🟡 DZIEŃ 6: Rust Unit Tests (8h)

#### Godzina 1-2: Setup i naprawa testów
```bash
# Zadanie 6.1: Uruchom wszystkie testy
cd core
cargo test 2>&1 | tee test-results.txt

# Zadanie 6.2: Analiza błędów
# - Zidentyfikuj failing tests
# - Przygotuj plan naprawy
```

#### Godzina 3-5: Naprawa testów
```rust
// Zadanie 6.3: Naprawa test_scanner_creation
#[tokio::test]
async fn test_scanner_creation() {
    let config = Config::default();
    let scanner = Scanner::new(config);
    assert!(scanner.is_ok());
    
    let scanner = scanner.unwrap();
    assert_eq!(scanner.get_status(), ScannerStatus::Idle);
}

// Zadanie 6.4: Dodanie nowych testów
#[tokio::test]
async fn test_scan_file() {
    let config = Config::default();
    let scanner = Scanner::new(config).unwrap();
    
    // Utwórz testowy plik
    let temp_file = create_test_file();
    
    // Skanuj plik
    let result = scanner.scan_file(&temp_file).await;
    assert!(result.is_ok());
    
    // Cleanup
    remove_test_file(temp_file);
}
```

#### Godzina 6-8: Testy integracyjne
```rust
// Zadanie 6.5: Integration tests
#[tokio::test]
async fn test_scan_and_quarantine_flow() {
    let engine = GhostEngine::new().unwrap();
    
    // 1. Skanuj plik
    let scan_result = engine.scan_path("/tmp/test").await.unwrap();
    
    // 2. Jeśli znaleziono zagrożenie, kwarantanna
    if scan_result.threats_found > 0 {
        let threat_id = scan_result.threats[0].id.clone();
        engine.quarantine_threat(&threat_id).await.unwrap();
    }
    
    // 3. Weryfikuj kwarantannę
    let quarantine_files = engine.list_quarantine_files();
    assert!(!quarantine_files.is_empty());
}
```

**Deliverables Dzień 6:**
- ✅ Wszystkie testy Rust przechodzą
- ✅ Dodane nowe testy
- ✅ Integration tests

---

### 🟡 DZIEŃ 7: Python & Go Tests (8h)

#### Godzina 1-4: Python Tests
```bash
# Zadanie 7.1: Setup pytest
cd ai-engine
pip install pytest pytest-asyncio pytest-cov

# Zadanie 7.2: Uruchom testy
pytest tests/ -v --cov=src

# Zadanie 7.3: Naprawa błędów
# - Fix failing tests
# - Add missing tests
```

```python
# Zadanie 7.4: Dodatkowe testy
# tests/test_integration.py
import pytest
from pathlib import Path

@pytest.mark.asyncio
async def test_full_analysis_flow():
    engine = AIEngine(Config.default())
    
    # Create test file
    test_file = create_test_file()
    
    # Analyze
    result = await engine.analyze_file(test_file)
    
    # Verify
    assert result is not None
    assert result.threat_score >= 0
    assert result.threat_score <= 100
    
    # Cleanup
    test_file.unlink()
```

#### Godzina 5-8: Go Tests
```go
// Zadanie 7.5: Go unit tests
// network-guard/firewall_test.go
package main

import (
    "testing"
)

func TestFirewallManager_AddRule(t *testing.T) {
    fm := NewFirewallManager()
    
    rule := FirewallRule{
        Name:     "Test Rule",
        Action:   "allow",
        Protocol: "tcp",
        DestPort: 80,
        Enabled:  true,
    }
    
    err := fm.AddRule(rule)
    if err != nil {
        t.Errorf("Failed to add rule: %v", err)
    }
    
    rules := fm.GetRules()
    if len(rules) != 1 {
        t.Errorf("Expected 1 rule, got %d", len(rules))
    }
}

// Zadanie 7.6: Integration tests
func TestNetworkMonitor_Integration(t *testing.T) {
    nm := NewNetworkMonitor()
    
    err := nm.Start()
    if err != nil {
        t.Fatalf("Failed to start monitor: %v", err)
    }
    
    time.Sleep(2 * time.Second)
    
    connections := nm.GetConnections()
    if connections == nil {
        t.Error("Expected connections, got nil")
    }
    
    nm.Stop()
}
```

**Deliverables Dzień 7:**
- ✅ Python tests passing
- ✅ Go tests implemented
- ✅ Integration tests

---

### 🟡 DZIEŃ 8: Frontend Tests (8h)

#### Godzina 1-3: Setup Testing Framework
```bash
# Zadanie 8.1: Install dependencies
cd web-ui
npm install --save-dev @testing-library/react @testing-library/jest-dom vitest

# Zadanie 8.2: Configure vitest
# vite.config.ts
```

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  },
})
```

#### Godzina 4-6: Component Tests
```typescript
// Zadanie 8.3: Scanner component tests
// src/__tests__/Scanner.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { Scanner } from '../pages/Scanner'

describe('Scanner Component', () => {
  test('renders scanner page', () => {
    render(<Scanner />)
    expect(screen.getByText(/scanner/i)).toBeInTheDocument()
  })

  test('starts scan on button click', async () => {
    render(<Scanner />)
    
    const startButton = screen.getByRole('button', { name: /start scan/i })
    fireEvent.click(startButton)
    
    await waitFor(() => {
      expect(screen.getByText(/scanning/i)).toBeInTheDocument()
    })
  })

  test('displays scan results', async () => {
    render(<Scanner />)
    
    // Mock API response
    // ... test implementation
  })
})
```

#### Godzina 7-8: API Service Tests
```typescript
// Zadanie 8.4: API service tests
// src/services/__tests__/api.test.ts
import { ApiClient } from '../api'

describe('ApiClient', () => {
  let client: ApiClient

  beforeEach(() => {
    client = new ApiClient('http://localhost:8080')
  })

  test('getScanStats returns data', async () => {
    const stats = await client.getScanStats()
    expect(stats).toBeDefined()
    expect(stats.total_scanned).toBeGreaterThanOrEqual(0)
  })

  test('startScan sends correct request', async () => {
    const result = await client.startScan({
      path: '/tmp',
      scan_type: 'quick',
      deep_scan: false,
    })
    
    expect(result).toBeDefined()
    expect(result.scan_id).toBeDefined()
  })
})
```

**Deliverables Dzień 8:**
- ✅ Component tests
- ✅ API service tests
- ✅ Test coverage >70%

---

### 🟡 DZIEŃ 9: E2E Tests (8h)

#### Godzina 1-3: Cypress Setup
```bash
# Zadanie 9.1: Install Cypress
cd web-ui
npm install --save-dev cypress

# Zadanie 9.2: Configure Cypress
npx cypress open
```

```typescript
// cypress.config.ts
import { defineConfig } from 'cypress'

export default defineConfig({
  e2e: {
    baseUrl: 'http://localhost:3000',
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
})
```

#### Godzina 4-7: E2E Test Cases
```typescript
// Zadanie 9.3: Login flow
// cypress/e2e/login.cy.ts
describe('Login Flow', () => {
  it('successfully logs in', () => {
    cy.visit('/login')
    cy.get('input[name="username"]').type('admin')
    cy.get('input[name="password"]').type('admin123')
    cy.get('button[type="submit"]').click()
    
    cy.url().should('include', '/dashboard')
    cy.contains('Welcome').should('be.visible')
  })
})

// Zadanie 9.4: Scanning flow
// cypress/e2e/scanning.cy.ts
describe('Scanning Flow', () => {
  beforeEach(() => {
    cy.login() // Custom command
  })

  it('starts and completes a scan', () => {
    cy.visit('/scanner')
    cy.get('button').contains('Start Scan').click()
    
    cy.get('[data-testid="scan-progress"]', { timeout: 10000 })
      .should('be.visible')
    
    cy.get('[data-testid="scan-complete"]', { timeout: 60000 })
      .should('be.visible')
  })
})
```

#### Godzina 8: Performance Tests
```typescript
// Zadanie 9.5: Performance tests
describe('Performance', () => {
  it('loads dashboard quickly', () => {
    cy.visit('/dashboard')
    cy.window().then((win) => {
      const performance = win.performance
      const loadTime = performance.timing.loadEventEnd - performance.timing.navigationStart
      expect(loadTime).to.be.lessThan(3000) // 3 seconds
    })
  })
})
```

**Deliverables Dzień 9:**
- ✅ E2E tests for main flows
- ✅ Performance tests
- ✅ CI integration

---

### 🟡 DZIEŃ 10: Utility Scripts (8h)

#### Godzina 1-2: Build Script
```bash
# Zadanie 10.1: scripts/build-all.sh
#!/bin/bash

echo "Building all components..."

# Build Core Engine
echo "Building Core Engine..."
cd core
cargo build --release
cd ..

# Build AI Engine
echo "Building AI Engine..."
cd ai-engine
pip install -r requirements.txt
cd ..

# Build Network Guard
echo "Building Network Guard..."
cd network-guard
go build -o network-guard
cd ..

# Build Web Dashboard
echo "Building Web Dashboard..."
cd web-ui
npm install
npm run build
cd ..

echo "All components built successfully!"
```

#### Godzina 3-4: Test Runner Script
```bash
# Zadanie 10.2: scripts/run-tests.sh
#!/bin/bash

echo "Running all tests..."

# Rust tests
echo "Running Rust tests..."
cd core
cargo test
cd ..

# Python tests
echo "Running Python tests..."
cd ai-engine
pytest tests/ -v
cd ..

# Go tests
echo "Running Go tests..."
cd network-guard
go test -v ./...
cd ..

# Frontend tests
echo "Running Frontend tests..."
cd web-ui
npm test
cd ..

echo "All tests completed!"
```

#### Godzina 5-6: Development Setup Script
```bash
# Zadanie 10.3: scripts/setup-dev.sh
#!/bin/bash

echo "Setting up development environment..."

# Install Rust
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

# Install Go
if ! command -v go &> /dev/null; then
    echo "Please install Go from https://golang.org/dl/"
fi

# Install Node.js
if ! command -v node &> /dev/null; then
    echo "Please install Node.js from https://nodejs.org/"
fi

# Install Python dependencies
pip install -r ai-engine/requirements.txt

# Install Node dependencies
cd web-ui && npm install && cd ..

echo "Development environment ready!"
```

#### Godzina 7-8: Operations Scripts
```bash
# Zadanie 10.4: scripts/backup.sh
#!/bin/bash

BACKUP_DIR="backups/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup database
docker-compose exec postgres pg_dump -U ghost ghostantivirus > "$BACKUP_DIR/db.sql"

# Backup quarantine
cp -r data/quarantine "$BACKUP_DIR/"

# Backup logs
cp -r data/logs "$BACKUP_DIR/"

echo "Backup created: $BACKUP_DIR"

# Zadanie 10.5: scripts/rollback.sh
#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./rollback.sh <backup-directory>"
    exit 1
fi

BACKUP_DIR=$1

# Stop services
docker-compose down

# Restore database
docker-compose up -d postgres
sleep 5
docker-compose exec -T postgres psql -U ghost ghostantivirus < "$BACKUP_DIR/db.sql"

# Restore quarantine
rm -rf data/quarantine
cp -r "$BACKUP_DIR/quarantine" data/

# Start services
docker-compose up -d

echo "Rollback completed from: $BACKUP_DIR"
```

**Deliverables Dzień 10:**
- ✅ build-all.sh
- ✅ run-tests.sh
- ✅ setup-dev.sh
- ✅ backup.sh
- ✅ rollback.sh
- ✅ health-check.sh

---

## 📅 TYDZIEŃ 3: Optymalizacje i Finalizacja (28 godzin)

### 🟢 DZIEŃ 11-12: Performance Optimization (16h)

#### Zadania:
1. Profiling Core Engine (4h)
2. Database query optimization (4h)
3. Caching implementation (4h)
4. Connection pooling (2h)
5. Load testing (2h)

### 🟢 DZIEŃ 13: Security Hardening (8h)

#### Zadania:
1. Input validation (2h)
2. SQL injection prevention (2h)
3. XSS protection (2h)
4. CSRF tokens (1h)
5. Rate limiting enhancement (1h)

### 🟢 DZIEŃ 14: Code Quality & Documentation (4h)

#### Zadania:
1. Linting & formatting (2h)
2. Documentation updates (2h)

---

## ✅ Checklist Końcowy

### Implementacja:
- [ ] Wszystkie TODO usunięte
- [ ] Wszystkie funkcje działają z prawdziwymi danymi
- [ ] Integracja między komponentami działa
- [ ] Brak mock data w produkcji

### Testy:
- [ ] Wszystkie testy jednostkowe przechodzą
- [ ] Testy integracyjne przechodzą
- [ ] E2E testy przechodzą
- [ ] Pokrycie testami >70%

### Skrypty:
- [ ] Wszystkie skrypty działają
- [ ] Deployment automation działa
- [ ] Backup/restore działa
- [ ] Monitoring działa

### Dokumentacja:
- [ ] API documentation aktualna
- [ ] Code comments dodane
- [ ] Troubleshooting guide zaktualizowany
- [ ] Architecture diagrams aktualne

### Performance:
- [ ] Response time <100ms
- [ ] Load testing passed
- [ ] Memory leaks fixed
- [ ] Database optimized

### Security:
- [ ] Security audit passed
- [ ] Vulnerabilities fixed
- [ ] Input validation complete
- [ ] Authentication secure

---

## 📊 Tracking Progress

### Daily Standup Questions:
1. Co zostało zrobione wczoraj?
2. Co będzie zrobione dzisiaj?
3. Czy są jakieś blokery?

### Weekly Review:
1. Ile zadań ukończono?
2. Czy jesteśmy na czasie?
3. Czy trzeba dostosować plan?

---

*Plan utworzony: 8 Listopada 2024*
*Szacowany czas realizacji: 3 tygodnie (56-68 godzin)*
*Status: Gotowy do implementacji*