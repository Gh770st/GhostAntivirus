#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Fix all remaining ThreatInfo creations that have the old structure
# Find and replace any remaining instances with the old fields

# Fix pattern: file_path: xxx, threat_type: xxx, threat_name: xxx, severity: xxx, hash: xxx, size: xxx
import re

# Pattern to match old ThreatInfo creation
pattern = r'ThreatInfo\s*\{\s*file_path:\s*([^,]+),\s*threat_type:\s*([^,]+),\s*threat_name:\s*([^,]+),\s*severity:\s*([^,]+),\s*hash:\s*([^,]+),\s*size:\s*([^}]+)\s*\}'

def replace_threat_info(match):
    return f'''ThreatInfo {{
                    threat_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: {match.group(2)},
                    threat_name: {match.group(3)},
                    severity: {match.group(4)},
                    file_path: {match.group(1)},
                    file_hash: {match.group(5)},
                    detected_at: std::time::SystemTime::now(),
                    confidence: 0.8,
                }}'''

content = re.sub(pattern, replace_threat_info, content, flags=re.MULTILINE | re.DOTALL)

# Fix the size field reference that's causing the specific error
content = content.replace(
    '                    size,\n                }));',
    '                    confidence: 0.8,\n                }));'
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed all remaining ThreatInfo references")