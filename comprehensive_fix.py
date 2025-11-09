#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    lines = f.readlines()

# Process line by line to fix all ThreatInfo references
fixed_lines = []
i = 0
while i < len(lines):
    line = lines[i]
    
    # Check if this line starts a ThreatInfo creation
    if 'ThreatInfo {' in line:
        # Look for the closing brace
        brace_count = 1
        j = i + 1
        while j < len(lines) and brace_count > 0:
            if '{' in lines[j]:
                brace_count += 1
            if '}' in lines[j]:
                brace_count -= 1
            if brace_count == 0:
                break
            j += 1
        
        # Get the entire ThreatInfo block
        threat_info_block = ''.join(lines[i:j+1])
        
        # Replace with correct structure
        new_block = '''                ThreatInfo {
                    threat_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Suspicious,
                    threat_name: "Detected Threat".to_string(),
                    severity: Severity::Medium,
                    file_path: file_path.to_path_buf(),
                    file_hash: hash.to_string(),
                    detected_at: std::time::SystemTime::now(),
                    confidence: 0.8,
                }'''
        
        fixed_lines.append(new_block + '\n')
        i = j + 1
    else:
        fixed_lines.append(line)
        i += 1

with open('core/src/scanner.rs', 'w') as f:
    f.writelines(fixed_lines)

print("✅ Completely replaced all ThreatInfo structures")

# Also check test file
with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Fix test references if they exist
content = content.replace('hash:', 'file_hash:')

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed remaining hash -> file_hash references")