#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Update the ThreatInfo struct to match what we're using
old_struct = '''pub struct ThreatInfo {
    pub file_path: PathBuf,
    pub threat_type: ThreatType,
    pub threat_name: String,
    pub severity: Severity,
    pub hash: String,
    pub size: u64,
}'''

new_struct = '''pub struct ThreatInfo {
    pub threat_id: String,
    pub threat_type: ThreatType,
    pub threat_name: String,
    pub severity: Severity,
    pub file_path: PathBuf,
    pub file_hash: String,
    pub detected_at: std::time::SystemTime,
    pub confidence: f64,
}'''

content = content.replace(old_struct, new_struct)

# Fix all the places creating ThreatInfo to use the new structure
# Fix EICAR test
content = content.replace(
    '''return Ok(Some(ThreatInfo {
                    file_path: file_path.to_path_buf(),
                    threat_type: ThreatType::Virus,
                    threat_name: "EICAR-Test-File".to_string(),
                    severity: Severity::Low,
                    hash: hash.to_string(),
                    size,
                }));''',
    '''return Ok(Some(ThreatInfo {
                    threat_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Virus,
                    threat_name: "EICAR-Test-File".to_string(),
                    severity: Severity::Low,
                    file_path: file_path.to_path_buf(),
                    file_hash: hash.to_string(),
                    detected_at: std::time::SystemTime::now(),
                    confidence: 1.0,
                }));'''
)

# Fix heuristic detection
content = content.replace(
    '''return Ok(Some(ThreatInfo {
                    file_path: file_path.to_path_buf(),
                    threat_type: ThreatType::Suspicious,
                    threat_name: "Suspicious.Executable".to_string(),
                    severity: Severity::Medium,
                    hash: hash.to_string(),
                    size,
                }));''',
    '''return Ok(Some(ThreatInfo {
                    threat_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Suspicious,
                    threat_name: "Suspicious.Executable".to_string(),
                    severity: Severity::Medium,
                    file_path: file_path.to_path_buf(),
                    file_hash: hash.to_string(),
                    detected_at: std::time::SystemTime::now(),
                    confidence: 0.5,
                }));'''
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed ThreatInfo struct and all references")