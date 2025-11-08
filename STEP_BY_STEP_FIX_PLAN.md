# 🔧 GhostAntivirus - Szczegółowy Plan Naprawczy Krok Po Kroku

## 📋 SPIS TREŚCI
1. [Faza 1: Naprawa Infrastruktury](#faza-1)
2. [Faza 2: Implementacja Kluczowych Funkcji](#faza-2)
3. [Faza 3: AI Engine](#faza-3)
4. [Faza 4: Testy](#faza-4)
5. [Faza 5: Dokumentacja](#faza-5)

---

## 🚨 FAZA 1: NAPRAWA INFRASTRUKTURY (6.5 godz) {#faza-1}

### ✅ KROK 1.1: Naprawa Cargo.toml (30 min)

**Cel:** Dodać brakujące zależności dla benchmarków

**Akcje:**
```bash
cd GhostAntivirus/core
```

**Edytuj `Cargo.toml`:**
```toml
[dev-dependencies]
tempfile = "3.0"
tokio-test = "0.4"
criterion = { version = "0.5", features = ["html_reports"] }  # ← DODAĆ

# Na końcu pliku dodać:
[[bench]]
name = "scanner_benchmarks"
harness = false

[[bench]]
name = "quarantine_benchmarks"
harness = false

[[bench]]
name = "system_benchmarks"
harness = false
```

**Weryfikacja:**
```bash
cargo build --benches
# Powinno się skompilować bez błędów
```

---

### ✅ KROK 1.2: Utworzenie Brakujących Modułów (3 godz)

#### 1.2.1: Utworzenie src/system.rs (1 godz)

**Utwórz plik:** `core/src/system.rs`

```rust
//! System monitoring module

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, ProcessExt, CpuExt, DiskExt, NetworkExt};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SystemMonitor {
    system: Arc<Mutex<System>>,
    is_monitoring: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}

impl SystemMonitor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            system: Arc::new(Mutex::new(System::new_all())),
            is_monitoring: Arc::new(Mutex::new(false)),
        })
    }

    pub fn get_metrics(&self) -> Result<SystemMetrics> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();

        // Get disk info
        let disks = sys.disks();
        let (disk_used, disk_total) = if let Some(disk) = disks.first() {
            (disk.total_space() - disk.available_space(), disk.total_space())
        } else {
            (0, 0)
        };

        Ok(SystemMetrics {
            cpu_usage,
            memory_used,
            memory_total,
            disk_used,
            disk_total,
            network_rx: 0, // TODO: Implement
            network_tx: 0, // TODO: Implement
        })
    }

    pub fn start_monitoring(&self) -> Result<()> {
        let mut is_monitoring = self.is_monitoring.lock().unwrap();
        *is_monitoring = true;
        Ok(())
    }

    pub fn stop_monitoring(&self) -> Result<()> {
        let mut is_monitoring = self.is_monitoring.lock().unwrap();
        *is_monitoring = false;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        *self.is_monitoring.lock().unwrap()
    }

    pub fn get_cpu_usage(&self) -> Result<f32> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_cpu();
        Ok(sys.global_cpu_info().cpu_usage())
    }

    pub fn get_memory_usage(&self) -> Result<(u64, u64)> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_memory();
        Ok((sys.used_memory(), sys.total_memory()))
    }

    pub fn get_disk_usage(&self) -> Result<(u64, u64)> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_disks_list();
        
        let disks = sys.disks();
        if let Some(disk) = disks.first() {
            let used = disk.total_space() - disk.available_space();
            let total = disk.total_space();
            Ok((used, total))
        } else {
            Ok((0, 0))
        }
    }

    pub fn get_uptime(&self) -> Result<u64> {
        let sys = self.system.lock().unwrap();
        Ok(sys.uptime())
    }

    pub fn get_process_count(&self) -> Result<usize> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_processes();
        Ok(sys.processes().len())
    }

    pub fn get_processes(&self) -> Result<Vec<ProcessInfo>> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_processes();
        
        let processes: Vec<ProcessInfo> = sys.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect();
        
        Ok(processes)
    }

    pub fn get_process(&self, pid: u32) -> Result<ProcessInfo> {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_processes();
        
        let pid = sysinfo::Pid::from_u32(pid);
        if let Some(process) = sys.process(pid) {
            Ok(ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
        } else {
            Err(anyhow::anyhow!("Process not found"))
        }
    }

    pub fn get_system_info(&self) -> Result<SystemInfo> {
        let sys = self.system.lock().unwrap();
        
        Ok(SystemInfo {
            os_name: System::name().unwrap_or_default(),
            os_version: System::os_version().unwrap_or_default(),
            hostname: System::host_name().unwrap_or_default(),
            kernel_version: System::kernel_version().unwrap_or_default(),
        })
    }

    pub fn check_health(&self) -> Result<HealthStatus> {
        let metrics = self.get_metrics()?;
        
        Ok(HealthStatus {
            cpu_healthy: metrics.cpu_usage < 90.0,
            memory_healthy: (metrics.memory_used as f64 / metrics.memory_total as f64) < 0.9,
            disk_healthy: (metrics.disk_used as f64 / metrics.disk_total as f64) < 0.9,
        })
    }

    pub fn get_top_cpu_processes(&self, limit: usize) -> Result<Vec<ProcessInfo>> {
        let mut processes = self.get_processes()?;
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(limit);
        Ok(processes)
    }

    pub fn get_top_memory_processes(&self, limit: usize) -> Result<Vec<ProcessInfo>> {
        let mut processes = self.get_processes()?;
        processes.sort_by(|a, b| b.memory.cmp(&a.memory));
        processes.truncate(limit);
        Ok(processes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub kernel_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub cpu_healthy: bool,
    pub memory_healthy: bool,
    pub disk_healthy: bool,
}
```

#### 1.2.2: Aktualizacja src/lib.rs (15 min)

**Edytuj `core/src/lib.rs`:**
```rust
// Dodać na początku z innymi modułami:
pub mod system;

// Dodać do re-exports:
pub use system::{SystemMonitor, SystemMetrics, ProcessInfo, SystemInfo};
```

#### 1.2.3: Aktualizacja src/updater.rs (1 godz)

**Dodaj alias dla kompatybilności:**

**Edytuj `core/src/lib.rs`:**
```rust
// Dodać alias
pub use updater::UpdateManager as UpdatesManager;

// Lub utworzyć src/updates.rs jako alias:
pub use crate::updater::*;
```

---

### ✅ KROK 1.3: Naprawa Testów - Importy (2 godz)

**Cel:** Naprawić wszystkie importy w testach

#### 1.3.1: Skrypt do automatycznej naprawy (30 min)

**Utwórz:** `fix_test_imports.sh`

```bash
#!/bin/bash

# Skrypt do naprawy importów w testach

echo "Naprawianie importów w testach..."

# Znajdź wszystkie pliki testowe
find core/tests -name "*.rs" -type f | while read file; do
    echo "Przetwarzanie: $file"
    
    # Zamień ghost_antivirus na ghost_core
    sed -i 's/use ghost_antivirus::/use ghost_core::/g' "$file"
    
    # Napraw system imports
    sed -i 's/ghost_core::system::/ghost_core::system::/g' "$file"
    
    # Napraw updates imports  
    sed -i 's/ghost_core::updates::/ghost_core::updater::/g' "$file"
done

echo "Gotowe!"
```

**Uruchom:**
```bash
chmod +x fix_test_imports.sh
./fix_test_imports.sh
```

#### 1.3.2: Manualna weryfikacja (1.5 godz)

**Sprawdź każdy plik testowy:**
```bash
cd core
cargo test --no-run 2>&1 | grep "error"
```

**Napraw wszystkie błędy kompilacji ręcznie**

---

### ✅ KROK 1.4: Weryfikacja (30 min)

**Uruchom testy:**
```bash
cd core
cargo test
```

**Uruchom benchmarki:**
```bash
cargo bench --no-run
```

**Oczekiwany rezultat:**
- ✅ Wszystkie testy kompilują się
- ✅ Benchmarki kompilują się
- ⚠️ Niektóre testy mogą failować (to normalne na tym etapie)

---

## 🔨 FAZA 2: IMPLEMENTACJA KLUCZOWYCH FUNKCJI (31 godz) {#faza-2}

### ✅ KROK 2.1: Baza Sygnatur (4 godz)

#### 2.1.1: Utworzenie struktury bazy (1 godz)

**Utwórz:** `core/src/signatures.rs`

```rust
//! Signature database for malware detection

use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SignatureDatabase {
    pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub id: i64,
    pub hash: String,
    pub threat_name: String,
    pub threat_type: String,
    pub severity: String,
    pub description: String,
    pub created_at: i64,
}

impl SignatureDatabase {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS signatures (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT NOT NULL UNIQUE,
                threat_name TEXT NOT NULL,
                threat_type TEXT NOT NULL,
                severity TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(Self { pool })
    }

    pub async fn check_hash(&self, hash: &str) -> Result<Option<Signature>> {
        let signature = sqlx::query_as!(
            Signature,
            "SELECT * FROM signatures WHERE hash = ?",
            hash
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(signature)
    }

    pub async fn add_signature(&self, signature: &Signature) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO signatures (hash, threat_name, threat_type, severity, description, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            signature.hash,
            signature.threat_name,
            signature.threat_type,
            signature.severity,
            signature.description,
            signature.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }

    pub async fn get_all_signatures(&self) -> Result<Vec<Signature>> {
        let signatures = sqlx::query_as!(
            Signature,
            "SELECT * FROM signatures ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(signatures)
    }

    pub async fn delete_signature(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM signatures WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn update_signature(&self, signature: &Signature) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE signatures 
            SET threat_name = ?, threat_type = ?, severity = ?, description = ?
            WHERE id = ?
            "#,
            signature.threat_name,
            signature.threat_type,
            signature.severity,
            signature.description,
            signature.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_count(&self) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM signatures")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count)
    }
}
```

#### 2.1.2: Dodanie przykładowych sygnatur (1 godz)

**Utwórz:** `core/data/initial_signatures.sql`

```sql
-- EICAR test file
INSERT INTO signatures (hash, threat_name, threat_type, severity, description, created_at)
VALUES (
    '44d88612fea8a8f36de82e1278abb02f',
    'EICAR-Test-File',
    'Test',
    'Low',
    'EICAR antivirus test file',
    strftime('%s', 'now')
);

-- Known malware hashes (examples)
INSERT INTO signatures (hash, threat_name, threat_type, severity, description, created_at)
VALUES 
    ('5d41402abc4b2a76b9719d911017c592', 'Generic.Trojan', 'Trojan', 'High', 'Generic trojan detection', strftime('%s', 'now')),
    ('098f6bcd4621d373cade4e832627b4f6', 'Win32.Worm', 'Worm', 'Critical', 'Windows worm', strftime('%s', 'now')),
    ('5f4dcc3b5aa765d61d8327deb882cf99', 'Ransom.Cryptolocker', 'Ransomware', 'Critical', 'Cryptolocker ransomware', strftime('%s', 'now'));
```

#### 2.1.3: Integracja z Scanner (2 godz)

**Edytuj `core/src/scanner.rs`:**

```rust
use crate::signatures::SignatureDatabase;

pub struct Scanner {
    config: Config,
    stats: Arc<Mutex<ScanStats>>,
    current_state: Arc<Mutex<Option<ScanState>>>,
    signature_db: Arc<SignatureDatabase>,  // ← DODAĆ
}

impl Scanner {
    pub async fn new(config: Config) -> Result<Self> {
        // Inicjalizuj bazę sygnatur
        let signature_db = SignatureDatabase::new("sqlite:signatures.db").await?;
        
        Ok(Self {
            config,
            stats: Arc::new(Mutex::new(ScanStats::default())),
            current_state: Arc::new(Mutex::new(None)),
            signature_db: Arc::new(signature_db),
        })
    }

    async fn scan_file_internal(&self, path: &Path) -> Result<Option<ThreatInfo>> {
        // 1. Oblicz hash
        let hash = self.calculate_file_hash(path)?;
        
        // 2. Sprawdź w bazie sygnatur
        if let Some(signature) = self.signature_db.check_hash(&hash).await? {
            return Ok(Some(ThreatInfo {
                file_path: path.to_path_buf(),
                threat_type: self.parse_threat_type(&signature.threat_type),
                threat_name: signature.threat_name,
                severity: self.parse_severity(&signature.severity),
                hash,
                size: fs::metadata(path)?.len(),
            }));
        }
        
        // 3. TODO: Heuristic analysis
        // 4. TODO: AI analysis
        
        Ok(None)
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        use sha2::{Sha256, Digest};
        
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn parse_threat_type(&self, type_str: &str) -> ThreatType {
        match type_str {
            "Virus" => ThreatType::Virus,
            "Trojan" => ThreatType::Trojan,
            "Worm" => ThreatType::Worm,
            "Ransomware" => ThreatType::Ransomware,
            "Spyware" => ThreatType::Spyware,
            "Adware" => ThreatType::Adware,
            "Rootkit" => ThreatType::Rootkit,
            "Backdoor" => ThreatType::Backdoor,
            _ => ThreatType::Unknown,
        }
    }

    fn parse_severity(&self, severity_str: &str) -> Severity {
        match severity_str {
            "Low" => Severity::Low,
            "Medium" => Severity::Medium,
            "High" => Severity::High,
            "Critical" => Severity::Critical,
            _ => Severity::Medium,
        }
    }
}
```

---

### ✅ KROK 2.2: AI Engine Integration (6 godz)

#### 2.2.1: Naprawa komunikacji HTTP (2 godz)

**Edytuj `core/src/ai.rs`:**

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct AIIntegration {
    client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct ScanRequest {
    file_path: String,
    file_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct AIResponse {
    pub is_threat: bool,
    pub confidence: f64,
    pub threat_type: Option<String>,
    pub details: Option<String>,
}

impl AIIntegration {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            client,
            base_url: config.ai.endpoint.clone(),
            api_key: config.ai.api_key.clone(),
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        // Test connection
        let response = self.client
            .get(format!("{}/health", self.base_url))
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;
        
        if response.status().is_success() {
            log::info!("AI Engine connection established");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to connect to AI Engine"))
        }
    }

    pub async fn analyze_file(&self, file_path: &Path, file_hash: &str) -> Result<AIResponse> {
        let request = ScanRequest {
            file_path: file_path.to_string_lossy().to_string(),
            file_hash: file_hash.to_string(),
        };
        
        let response = self.client
            .post(format!("{}/scan", self.base_url))
            .header("X-API-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let ai_response: AIResponse = response.json().await?;
            Ok(ai_response)
        } else {
            Err(anyhow::anyhow!("AI analysis failed: {}", response.status()))
        }
    }

    pub async fn analyze_file_with_retry(&self, file_path: &Path, file_hash: &str, max_retries: u32) -> Result<AIResponse> {
        let mut attempts = 0;
        
        loop {
            match self.analyze_file(file_path, file_hash).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_retries {
                        return Err(e);
                    }
                    log::warn!("AI analysis attempt {} failed, retrying...", attempts);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }
}
```

---

### ✅ KROK 2.3: API Authentication (6 godz)

#### 2.3.1: JWT Implementation (3 godz)

**Utwórz:** `core/src/api/auth.rs`

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: usize,   // expiration
    pub iat: usize,   // issued at
    pub role: String, // user role
}

pub struct JWTManager {
    secret: String,
}

impl JWTManager {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token(&self, user_id: &str, role: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            role: role.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
```

#### 2.3.2: Authentication Middleware (2 godz)

**Edytuj:** `core/src/api/middleware.rs`

```rust
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::{StatusCode, HeaderMap},
};
use crate::api::auth::JWTManager;

pub async fn auth_middleware(
    State(jwt_manager): State<JWTManager>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // Validate token
    match jwt_manager.validate_token(token) {
        Ok(claims) => {
            // Token is valid, proceed
            // You can add claims to request extensions here
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
```

#### 2.3.3: Rate Limiting (1 godz)

**Edytuj:** `core/src/api/middleware.rs`

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check_rate_limit(&self, client_id: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();

        let client_requests = requests.entry(client_id.to_string()).or_insert_with(Vec::new);

        // Remove old requests
        client_requests.retain(|&time| now.duration_since(time) < self.window);

        if client_requests.len() >= self.max_requests {
            false
        } else {
            client_requests.push(now);
            true
        }
    }
}

pub async fn rate_limit_middleware(
    State(rate_limiter): State<Arc<RateLimiter>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get client IP or user ID
    let client_id = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    if rate_limiter.check_rate_limit(client_id) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}
```

---

## 🤖 FAZA 3: AI ENGINE (20 godz) {#faza-3}

### ✅ KROK 3.1: Implementacja Scan Logic (4 godz)

**Edytuj:** `ai-engine/src/api.py`

```python
@app.post("/scan")
async def scan_file(request: ScanRequest):
    """Scan a file for threats using ML model"""
    
    try:
        # 1. Validate file exists
        file_path = Path(request.file_path)
        if not file_path.exists():
            raise HTTPException(status_code=404, detail="File not found")
        
        # 2. Check if model is loaded
        if not engine.is_model_loaded():
            log.warning("Model not loaded, using fallback detection")
            return fallback_detection(file_path, request.file_hash)
        
        # 3. Analyze file
        result = await engine.analyze_file(file_path)
        
        # 4. Return result
        return {
            "is_threat": result.is_threat,
            "confidence": result.confidence,
            "threat_type": result.threat_type.value if result.threat_type else None,
            "severity": result.severity.value if result.severity else None,
            "details": result.details
        }
        
    except Exception as e:
        log.error(f"Scan failed: {e}")
        raise HTTPException(status_code=500, detail=str(e))

def fallback_detection(file_path: Path, file_hash: str) -> dict:
    """Fallback detection when ML model is not available"""
    
    # Simple heuristic checks
    is_threat = False
    threat_type = None
    confidence = 0.0
    
    # Check file extension
    suspicious_extensions = ['.exe', '.dll', '.scr', '.bat', '.cmd', '.vbs', '.js']
    if file_path.suffix.lower() in suspicious_extensions:
        confidence += 0.3
    
    # Check file size (very small or very large files)
    size = file_path.stat().st_size
    if size < 1024 or size > 100 * 1024 * 1024:  # < 1KB or > 100MB
        confidence += 0.2
    
    # Check for EICAR test string
    try:
        with open(file_path, 'rb') as f:
            content = f.read(1024)
            if b'EICAR' in content:
                is_threat = True
                threat_type = "Test"
                confidence = 1.0
    except:
        pass
    
    if confidence > 0.5:
        is_threat = True
        threat_type = "Suspicious"
    
    return {
        "is_threat": is_threat,
        "confidence": confidence,
        "threat_type": threat_type,
        "details": "Fallback heuristic detection"
    }
```

---

### ✅ KROK 3.2: Trening Prostego Modelu (16 godz)

**Uwaga:** To jest uproszczona wersja. Pełny trening wymaga dużego datasetu.

#### 3.2.1: Przygotowanie Datasetu (4 godz)

**Utwórz:** `ai-engine/scripts/prepare_dataset.py`

```python
import pandas as pd
from pathlib import Path
import hashlib

def prepare_dataset():
    """Prepare training dataset from samples"""
    
    data = []
    
    # Collect benign samples
    benign_dir = Path("data/benign")
    for file_path in benign_dir.glob("**/*"):
        if file_path.is_file():
            features = extract_features(file_path)
            features['label'] = 0  # benign
            data.append(features)
    
    # Collect malware samples
    malware_dir = Path("data/malware")
    for file_path in malware_dir.glob("**/*"):
        if file_path.is_file():
            features = extract_features(file_path)
            features['label'] = 1  # malware
            data.append(features)
    
    df = pd.DataFrame(data)
    df.to_csv("data/dataset.csv", index=False)
    print(f"Dataset prepared: {len(df)} samples")

def extract_features(file_path):
    """Extract basic features from file"""
    stat = file_path.stat()
    
    return {
        'size': stat.st_size,
        'extension': file_path.suffix,
        'entropy': calculate_entropy(file_path),
        # Add more features...
    }

def calculate_entropy(file_path):
    """Calculate file entropy"""
    import math
    from collections import Counter
    
    with open(file_path, 'rb') as f:
        data = f.read()
    
    if not data:
        return 0
    
    counter = Counter(data)
    length = len(data)
    
    entropy = 0
    for count in counter.values():
        p = count / length
        entropy -= p * math.log2(p)
    
    return entropy

if __name__ == "__main__":
    prepare_dataset()
```

#### 3.2.2: Trening Modelu (8 godz)

**Utwórz:** `ai-engine/scripts/train_model.py`

```python
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.preprocessing import StandardScaler
from joblib import dump
import numpy as np

def train_model():
    """Train ML model for malware detection"""
    
    # Load dataset
    df = pd.read_csv("data/dataset.csv")
    
    # Prepare features and labels
    X = df.drop('label', axis=1)
    y = df['label']
    
    # Split data
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.2, random_state=42
    )
    
    # Scale features
    scaler = StandardScaler()
    X_train_scaled = scaler.fit_transform(X_train)
    X_test_scaled = scaler.transform(X_test)
    
    # Train model
    model = RandomForestClassifier(
        n_estimators=100,
        max_depth=10,
        random_state=42
    )
    
    model.fit(X_train_scaled, y_train)
    
    # Evaluate
    train_score = model.score(X_train_scaled, y_train)
    test_score = model.score(X_test_scaled, y_test)
    
    print(f"Train accuracy: {train_score:.4f}")
    print(f"Test accuracy: {test_score:.4f}")
    
    # Save model
    dump(model, "models/malware_detector.pkl")
    dump(scaler, "models/scaler.pkl")
    
    print("Model saved successfully")

if __name__ == "__main__":
    train_model()
```

---

## 🧪 FAZA 4: TESTY (16 godz) {#faza-4}

### ✅ KROK 4.1: Uruchomienie i Naprawa Testów (8 godz)

```bash
# Uruchom testy i zapisz błędy
cd core
cargo test 2>&1 | tee test_errors.log

# Napraw każdy błąd po kolei
# Priorytet: testy kompilacji > testy funkcjonalne
```

### ✅ KROK 4.2: Dodanie Testów dla Nowych Funkcji (8 godz)

**Przykład - Test dla SignatureDatabase:**

```rust
#[tokio::test]
async fn test_signature_database() {
    let db = SignatureDatabase::new("sqlite::memory:").await.unwrap();
    
    let signature = Signature {
        id: 0,
        hash: "test_hash".to_string(),
        threat_name: "Test.Malware".to_string(),
        threat_type: "Virus".to_string(),
        severity: "High".to_string(),
        description: "Test".to_string(),
        created_at: 0,
    };
    
    // Test add
    let id = db.add_signature(&signature).await.unwrap();
    assert!(id > 0);
    
    // Test check
    let found = db.check_hash("test_hash").await.unwrap();
    assert!(found.is_some());
    
    // Test delete
    db.delete_signature(id).await.unwrap();
    let not_found = db.check_hash("test_hash").await.unwrap();
    assert!(not_found.is_none());
}
```

---

## 📚 FAZA 5: DOKUMENTACJA (7 godz) {#faza-5}

### ✅ KROK 5.1: Aktualizacja README (2 godz)

**Edytuj główny README.md z rzeczywistym statusem**

### ✅ KROK 5.2: API Documentation (3 godz)

**Wygeneruj OpenAPI/Swagger docs**

### ✅ KROK 5.3: Deployment Guide (2 godz)

**Utwórz szczegółową instrukcję wdrożenia**

---

## ✅ CHECKLIST KOŃCOWY

### Przed uznaniem za ukończone:

- [ ] Wszystkie testy kompilują się
- [ ] Co najmniej 80% testów przechodzi
- [ ] Scanner wykrywa EICAR test file
- [ ] AI Engine odpowiada na requesty
- [ ] API wymaga autentykacji
- [ ] Rate limiting działa
- [ ] Benchmarki się uruchamiają
- [ ] Dokumentacja jest aktualna

---

**Szacowany całkowity czas:** 80.5 godzin (~2 tygodnie pełnego czasu)

**Priorytet wykonania:** Faza 1 → Faza 2 → Faza 3 → Faza 4 → Faza 5