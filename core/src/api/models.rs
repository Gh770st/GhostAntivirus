// API request/response models
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================================================
// Authentication Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// ============================================================================
// Scanner Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanStatsResponse {
    pub total_scanned: u64,
    pub threats_found: u64,
    pub files_quarantined: u64,
    pub last_scan: Option<DateTime<Utc>>,
    pub scan_in_progress: bool,
    pub scan_progress: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartScanRequest {
    pub path: String,
    pub scan_type: ScanType,
    pub deep_scan: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Quick,
    Full,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: String,
    pub path: String,
    pub threat_type: Option<String>,
    pub severity: String,
    pub timestamp: DateTime<Utc>,
    pub action_taken: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResultsResponse {
    pub results: Vec<ScanResult>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

// ============================================================================
// Threat Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub threat_type: String,
    pub severity: ThreatSeverity,
    pub detected_at: DateTime<Utc>,
    pub status: ThreatStatus,
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreatStatus {
    Detected,
    Quarantined,
    Removed,
    Restored,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatListResponse {
    pub threats: Vec<ThreatInfo>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatActionRequest {
    pub action: ThreatAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreatAction {
    Quarantine,
    Remove,
    Restore,
    Ignore,
}

// ============================================================================
// Quarantine Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarantineFile {
    pub id: String,
    pub original_path: String,
    pub quarantine_path: String,
    pub quarantined_at: DateTime<Utc>,
    pub threat_type: String,
    pub size: u64,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarantineListResponse {
    pub files: Vec<QuarantineFile>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarantineStatsResponse {
    pub total_files: usize,
    pub total_size: u64,
    pub oldest_file: Option<DateTime<Utc>>,
    pub newest_file: Option<DateTime<Utc>>,
}

// ============================================================================
// Firewall Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallRule {
    pub id: String,
    pub name: String,
    pub action: RuleAction,
    pub direction: RuleDirection,
    pub protocol: Protocol,
    pub source_ip: Option<String>,
    pub source_port: Option<u16>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<u16>,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Block,
    Deny,
    Log,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleDirection {
    Inbound,
    Outbound,
    Both,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallRulesResponse {
    pub rules: Vec<FirewallRule>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallStatsResponse {
    pub packets_allowed: u64,
    pub packets_blocked: u64,
    pub packets_logged: u64,
    pub active_rules: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFirewallRuleRequest {
    pub name: String,
    pub action: RuleAction,
    pub direction: RuleDirection,
    pub protocol: Protocol,
    pub source_ip: Option<String>,
    pub source_port: Option<u16>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<u16>,
    pub priority: u32,
}

// ============================================================================
// Network Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub id: String,
    pub process_name: String,
    pub process_id: u32,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: String,
    pub remote_port: u16,
    pub protocol: String,
    pub state: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub established_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConnectionsResponse {
    pub connections: Vec<NetworkConnection>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatsResponse {
    pub total_connections: usize,
    pub active_connections: usize,
    pub blocked_connections: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

// ============================================================================
// Settings Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub real_time_protection: bool,
    pub auto_scan: bool,
    pub scan_schedule: String,
    pub auto_update: bool,
    pub quarantine_days: u32,
    pub notification_enabled: bool,
    pub ai_enabled: bool,
    pub network_monitoring: bool,
    pub firewall_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub real_time_protection: Option<bool>,
    pub auto_scan: Option<bool>,
    pub scan_schedule: Option<String>,
    pub auto_update: Option<bool>,
    pub quarantine_days: Option<u32>,
    pub notification_enabled: Option<bool>,
    pub ai_enabled: Option<bool>,
    pub network_monitoring: Option<bool>,
    pub firewall_enabled: Option<bool>,
}

// ============================================================================
// System Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    pub version: String,
    pub os: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub uptime: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatsResponse {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_usage: NetworkUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkUsage {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

// ============================================================================
// Update Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCheckResponse {
    pub update_available: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyUpdateRequest {
    pub version: String,
    pub auto_restart: bool,
}

// ============================================================================
// WebSocket Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanProgressUpdate {
    pub progress: f32,
    pub current_file: String,
    pub files_scanned: u64,
    pub threats_found: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub threat_id: String,
    pub threat_name: String,
    pub severity: ThreatSeverity,
    pub path: String,
    pub action_taken: String,
}

// ============================================================================
// Common Response Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub services: ServiceStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub scanner: bool,
    pub ai_engine: bool,
    pub network_guard: bool,
    pub database: bool,
}