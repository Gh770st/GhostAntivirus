#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Fix the broken ThreatInfo struct definition
broken_struct = '''/// Threat information
#[derive(Debug, Clone, Serialize, Deserialize)]
                ThreatInfo {
                    threat_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Suspicious,
                    threat_name: "Detected Threat".to_string(),
                    severity: Severity::Medium,
                    file_path: file_path.to_path_buf(),
                    file_hash: hash.to_string(),
                    detected_at: std::time::SystemTime::now(),
                    confidence: 0.8,
                }'''

correct_struct = '''/// Threat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatInfo {
    pub threat_id: String,
    pub threat_type: ThreatType,
    pub threat_name: String,
    pub severity: Severity,
    pub file_path: PathBuf,
    pub file_hash: String,
    pub detected_at: std::time::SystemTime,
    pub confidence: f64,
}'''

content = content.replace(broken_struct, correct_struct)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed ThreatInfo struct definition")