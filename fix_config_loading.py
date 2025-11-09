#!/usr/bin/env python3

with open('core/src/config.rs', 'r') as f:
    content = f.read()

# Make the new fields optional in deserialization
content = content.replace(
    'pub auto_scan: bool,',
    '#[serde(default)]\n    pub auto_scan: bool,'
)

content = content.replace(
    'pub schedule: Option<String>,',
    '#[serde(default)]\n    pub schedule: Option<String>,'
)

content = content.replace(
    'pub notifications: NotificationConfig,',
    '#[serde(default)]\n    pub notifications: NotificationConfig,'
)

with open('core/src/config.rs', 'w') as f:
    f.write(content)

print("✅ Made new config fields optional with #[serde(default)]")