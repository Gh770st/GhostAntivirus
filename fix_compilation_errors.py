#!/usr/bin/env python3
import re

print("Fixing compilation errors...")

# Fix 1: scanner.rs - Replace file_size with size in ThreatInfo structs
with open('core/src/scanner.rs', 'r') as f:
    scanner_content = f.read()

# Replace file_size with size in ThreatInfo struct initializations
scanner_content = re.sub(
    r'(ThreatInfo\s*\{[^}]*hash:[^,]+,\s*)file_size,',
    r'\1size: file_size,',
    scanner_content,
    flags=re.DOTALL
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(scanner_content)
print("✓ Fixed scanner.rs - ThreatInfo field names")

# Fix 2: Replace ThreatType::PUA with ThreatType::Adware
scanner_content = scanner_content.replace('ThreatType::PUA', 'ThreatType::Adware')
with open('core/src/scanner.rs', 'w') as f:
    f.write(scanner_content)
print("✓ Fixed scanner.rs - ThreatType::PUA -> ThreatType::Adware")

# Fix 3: updates.rs - Make engine mutable
with open('core/src/api/handlers/updates.rs', 'r') as f:
    updates_content = f.read()

updates_content = updates_content.replace(
    'let engine = state.engine.write().await;',
    'let mut engine = state.engine.write().await;'
)

with open('core/src/api/handlers/updates.rs', 'w') as f:
    f.write(updates_content)
print("✓ Fixed updates.rs - made engine mutable")

# Fix 4: signatures.rs - Fix config.data_dir
with open('core/src/signatures.rs', 'r') as f:
    sig_content = f.read()

# Replace config.data_dir with proper path construction
sig_content = sig_content.replace(
    'let mut db_path = config.data_dir.clone();',
    'let mut db_path = std::path::PathBuf::from("./data");'
)

sig_content = sig_content.replace(
    'let update_dir = self.config.data_dir.join("updates");',
    'let update_dir = std::path::PathBuf::from("./data/updates");'
)

sig_content = sig_content.replace(
    'let version_file = self.config.data_dir.join("current_version");',
    'let version_file = std::path::PathBuf::from("./data/current_version");'
)

with open('core/src/signatures.rs', 'w') as f:
    f.write(sig_content)
print("✓ Fixed signatures.rs - config.data_dir paths")

# Fix 5: websocket.rs - Fix clone issue
with open('core/src/websocket.rs', 'r') as f:
    ws_content = f.read()

# Replace the problematic get_broadcaster function
old_get_broadcaster = '''/// Get global WebSocket broadcaster
pub fn get_broadcaster() -> Option<WSBroadcaster> {
    let global = GLOBAL_BROADCASTER.lock().unwrap();
    global.clone()
}'''

new_get_broadcaster = '''/// Get global WebSocket broadcaster
pub fn get_broadcaster() -> Option<WSBroadcaster> {
    let global = GLOBAL_BROADCASTER.lock().unwrap();
    global.as_ref().map(|b| {
        WSBroadcaster {
            tx: b.tx.clone(),
            clients: Arc::clone(&b.clients),
            config: b.config.clone(),
        }
    })
}'''

ws_content = ws_content.replace(old_get_broadcaster, new_get_broadcaster)

with open('core/src/websocket.rs', 'w') as f:
    f.write(ws_content)
print("✓ Fixed websocket.rs - clone implementation")

print("\nAll fixes applied!")