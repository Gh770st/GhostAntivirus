#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Replace the last occurrence of file_size, with size: file_size,
import re
content = re.sub(
    r'hash: hash\.to_string\(\),\s+file_size,',
    'hash: hash.to_string(),\n                           size: file_size,',
    content
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✓ Fixed last file_size occurrence")