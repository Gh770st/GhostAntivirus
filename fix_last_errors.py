#!/usr/bin/env python3

with open('core/src/api/handlers/firewall.rs', 'r') as f:
    content = f.read()

# Fix 1: Add SystemTime import at the top
# Find the imports section and add SystemTime
if 'use std::time::SystemTime;' not in content:
    # Add after the first use statement
    content = content.replace(
        'use axum::{',
        'use std::time::SystemTime;\nuse axum:{'
    )

# Fix 2: Remove Some() wrapper from protocol (it's already Option<Protocol>)
content = content.replace(
    '        protocol: Some(protocol),',
    '        protocol,  // protocol is already Option<Protocol>'
)

with open('core/src/api/handlers/firewall.rs', 'w') as f:
    f.write(content)

print("Fixed firewall.rs - added SystemTime import and fixed protocol")