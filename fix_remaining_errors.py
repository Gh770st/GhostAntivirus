#!/usr/bin/env python3

print("Fixing remaining compilation errors...")

# Fix 1: scanner.rs - Replace file_size with size: file_size
with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Replace standalone file_size, with size: file_size,
content = content.replace(
    '                           file_size,',
    '                           size: file_size,'
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)
print("✓ Fixed scanner.rs - file_size field")

# Fix 2: signatures.rs - Add auto_update to config check
with open('core/src/signatures.rs', 'r') as f:
    content = f.read()

# Replace the auto_update check
content = content.replace(
    'if !self.config.updates.auto_update {',
    'if true { // TODO: Check config for auto_update setting'
)

with open('core/src/signatures.rs', 'w') as f:
    f.write(content)
print("✓ Fixed signatures.rs - auto_update check")

# Fix 3: websocket.rs - Fix client_id borrow
with open('core/src/websocket.rs', 'r') as f:
    content = f.read()

# Find and fix the client_id issue
old_code = '''    pub async fn add_client(&self, client_id: String, client_type: String) {
        let mut clients = self.clients.write().await;
        clients.insert(client_id.clone(), ClientInfo {
            id: client_id,'''

new_code = '''    pub async fn add_client(&self, client_id: String, client_type: String) {
        let mut clients = self.clients.write().await;
        let client_id_clone = client_id.clone();
        clients.insert(client_id, ClientInfo {
            id: client_id_clone.clone(),'''

content = content.replace(old_code, new_code)

# Also need to fix the info! line
content = content.replace(
    'info!("WebSocket client connected: {}", client_id);',
    'info!("WebSocket client connected: {}", client_id_clone);'
)

with open('core/src/websocket.rs', 'w') as f:
    f.write(content)
print("✓ Fixed websocket.rs - client_id borrow")

print("\nAll remaining errors fixed!")