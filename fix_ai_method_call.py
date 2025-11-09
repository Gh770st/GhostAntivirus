#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Update the AI detection to use analyze_features instead of analyze_file
content = content.replace(
    "match self.ai_engine.analyze_file(&file_features).await",
    "match self.ai_engine.analyze_features(&file_features).await"
)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed AI method call - using analyze_features instead of analyze_file")