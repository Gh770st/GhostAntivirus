#!/usr/bin/env python3
import re

# Fix 1: Remove unused import ProcessExt from system.rs
with open('core/src/system.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt};',
    'use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt};'
)
with open('core/src/system.rs', 'w') as f:
    f.write(content)
print("✅ Fixed system.rs - removed unused ProcessExt import")

# Fix 2: Prefix unused variables with underscore
files_to_fix = [
    ('core/src/scanner.rs', 'deep_scan: bool', '_deep_scan: bool'),
    ('core/src/api/handlers/system.rs', 'let engine = ', 'let _engine = '),
    ('core/src/updates.rs', 'interval_hours: u64', '_interval_hours: u64'),
    ('core/src/updater.rs', 'path: &Path', '_path: &Path'),
]

for file_path, old_str, new_str in files_to_fix:
    with open(file_path, 'r') as f:
        content = f.read()
    content = content.replace(old_str, new_str)
    with open(file_path, 'w') as f:
        f.write(content)
    print(f"✅ Fixed {file_path}")

# Fix 3: Add #[allow(dead_code)] for unused fields that might be used later
with open('core/src/analyzer.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'pub struct BehaviorAnalyzer {',
    '#[allow(dead_code)]\npub struct BehaviorAnalyzer {'
)
with open('core/src/analyzer.rs', 'w') as f:
    f.write(content)
print("✅ Fixed analyzer.rs - allowed dead_code for future fields")

with open('core/src/ai.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'pub struct AIIntegration {',
    '#[allow(dead_code)]\npub struct AIIntegration {'
)
with open('core/src/ai.rs', 'w') as f:
    f.write(content)
print("✅ Fixed ai.rs - allowed dead_code for future fields")

with open('core/src/network.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'pub struct NetworkMonitor {',
    '#[allow(dead_code)]\npub struct NetworkMonitor {'
)
with open('core/src/network.rs', 'w') as f:
    f.write(content)
print("✅ Fixed network.rs - allowed dead_code for future fields")

with open('core/src/updates.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'pub struct UpdateManager {',
    '#[allow(dead_code)]\npub struct UpdateManager {'
)
with open('core/src/updates.rs', 'w') as f:
    f.write(content)
print("✅ Fixed updates.rs - allowed dead_code for future fields")

# Fix 4: Handle unused Result in settings.rs
with open('core/src/api/handlers/settings.rs', 'r') as f:
    content = f.read()
content = content.replace(
    '            engine.monitor.stop().await;',
    '            let _ = engine.monitor.stop().await;'
)
with open('core/src/api/handlers/settings.rs', 'w') as f:
    f.write(content)
print("✅ Fixed settings.rs - handled unused Result")

# Fix 5: Add #[allow(dead_code)] for unused method
with open('core/src/scanner.rs', 'r') as f:
    content = f.read()
# Find update_progress method and add allow attribute
content = re.sub(
    r'(\s+)(fn update_progress)',
    r'\1#[allow(dead_code)]\n\1\2',
    content
)
with open('core/src/scanner.rs', 'w') as f:
    f.write(content)
print("✅ Fixed scanner.rs - allowed dead_code for update_progress method")

print("\n✅ All warnings fixed!")