#!/usr/bin/env python3

with open('core/src/updater.rs', 'r') as f:
    content = f.read()

# The issue is that we renamed 'path' parameter to '_path' but the function body uses 'path'
# We need to either:
# 1. Keep the parameter as 'path' (it IS used)
# 2. Or rename all usages to '_path'

# Let's keep it as 'path' since it's actually used
content = content.replace('_path: &Path', 'path: &Path')

with open('core/src/updater.rs', 'w') as f:
    f.write(content)

print("✅ Fixed updater.rs - restored 'path' parameter name (it is actually used)")