#!/usr/bin/env python3

with open('core/src/api/handlers/scanner.rs', 'r') as f:
    content = f.read()

# Fix the ScanType conversion
old_code = '''    // Start scan based on type
    let scan_result = engine.scanner.start_scan(
        &payload.path,
        payload.scan_type.clone(),
        payload.deep_scan
    ).await;'''

new_code = '''    // Convert API ScanType to scanner ScanType
    let scan_type = match payload.scan_type {
        crate::api::models::ScanType::Quick => crate::scanner::ScanType::Quick,
        crate::api::models::ScanType::Full => crate::scanner::ScanType::Full,
        crate::api::models::ScanType::Custom => crate::scanner::ScanType::Custom,
    };
    
    // Start scan based on type
    let scan_result = engine.scanner.start_scan(
        &payload.path,
        scan_type,
        payload.deep_scan
    ).await;'''

content = content.replace(old_code, new_code)

with open('core/src/api/handlers/scanner.rs', 'w') as f:
    f.write(content)

print("Fixed scanner.rs")