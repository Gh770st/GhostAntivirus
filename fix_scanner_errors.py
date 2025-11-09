#!/usr/bin/env python3
import re

# Read the file
with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Fix 1: Replace 'size,' with 'file_size,' in the AI detection section
content = re.sub(
    r'(hash: hash\.to_string\(\),\s+)size,',
    r'\1file_size,',
    content
)

# Write back
with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("Fixed scanner.rs - replaced 'size' with 'file_size'")