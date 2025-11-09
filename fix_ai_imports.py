#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Add ai module imports at the top
if 'use crate::ai::' not in content:
    # Find the imports section
    import_point = content.find("use anyhow::")
    if import_point != -1:
        content = content[:import_point] + '''use crate::ai::{FileFeatures, PEFeatures};
''' + content[import_point:]

# Replace ai::FileFeatures with just FileFeatures
content = content.replace('ai::FileFeatures', 'FileFeatures')
content = content.replace('ai::PEFeatures', 'PEFeatures')

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed AI imports in scanner.rs")