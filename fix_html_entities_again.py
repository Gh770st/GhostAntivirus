#!/usr/bin/env python3
import os

# Fix HTML entities in all Rust files
files_to_fix = [
    'core/src/scanner.rs',
    'core/src/config.rs',
    'core/src/api/handlers/settings.rs',
    'core/src/api/handlers.rs',
    'core/src/api/middleware.rs',
    'core/src/monitor.rs',
]

for file_path in files_to_fix:
    if os.path.exists(file_path):
        with open(file_path, 'rb') as f:
            content = f.read()
        
        # Replace HTML entities
        content = content.replace(b'&amp;', b'&')
        content = content.replace(b'&lt;', b'<')
        content = content.replace(b'&gt;', b'>')
        content = content.replace(b'&quot;', b'"')
        
        with open(file_path, 'wb') as f:
            f.write(content)
        
        print(f"✅ Fixed HTML entities in {file_path}")

print("\n✅ All HTML entities fixed!")