#!/usr/bin/env python3

with open('core/src/api/handlers.rs', 'r') as f:
    content = f.read()

# Add lazy_static for storing start time
if 'lazy_static!' not in content:
    # Add imports at the top
    import_point = content.find('use axum::{')
    if import_point != -1:
        content = content[:import_point] + '''use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;

lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

''' + content[import_point:]

# Replace the uptime calculation
content = content.replace(
    'uptime: 0, // TODO: Calculate actual uptime',
    '''uptime: START_TIME.elapsed()
            .unwrap_or_default()
            .as_secs()'''
)

with open('core/src/api/handlers.rs', 'w') as f:
    f.write(content)

print("✅ Implemented uptime calculation")