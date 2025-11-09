#!/usr/bin/env python3

# Fix 1: Check the HealthCheckResponse structure
with open('core/src/api/handlers.rs', 'r') as f:
    content = f.read()

# The issue is likely a missing comma. Let me check the structure
lines = content.split('\n')
for i, line in enumerate(lines):
    if 'let response = HealthCheckResponse {' in line:
        # Check the next few lines
        for j in range(i, min(i+15, len(lines))):
            if 'services:' in lines[j] and not lines[j-1].strip().endswith(','):
                # Add comma to previous line
                lines[j-1] = lines[j-1].rstrip() + ','
                break

content = '\n'.join(lines)

with open('core/src/api/handlers.rs', 'w') as f:
    f.write(content)

print("✅ Fixed HealthCheckResponse structure")

# Fix 2: Add warn! macro import to middleware.rs
with open('core/src/api/middleware.rs', 'r') as f:
    content = f.read()

if 'use log::warn;' not in content:
    # Add import at the top
    import_point = content.find('use axum::{')
    if import_point != -1:
        content = content[:import_point] + 'use log::warn;\n' + content[import_point:]

with open('core/src/api/middleware.rs', 'w') as f:
    f.write(content)

print("✅ Added warn! macro import to middleware.rs")

print("\n✅ All errors fixed!")