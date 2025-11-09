#!/usr/bin/env python3

# Fix 1: Add auto_scan and schedule to ScannerConfig
with open('core/src/config.rs', 'r') as f:
    content = f.read()

# Find ScannerConfig and add fields
if 'pub auto_scan: bool' not in content:
    # Find ScannerConfig struct
    scanner_config_start = content.find('pub struct ScannerConfig {')
    if scanner_config_start != -1:
        # Find the closing brace
        brace_pos = content.find('}', scanner_config_start)
        # Insert new fields before the closing brace
        insert_text = '''    pub auto_scan: bool,
    pub schedule: Option<String>,
'''
        content = content[:brace_pos] + insert_text + content[brace_pos:]

# Add notifications to Config
if 'pub notifications: NotificationConfig' not in content:
    # Find Config struct
    config_start = content.find('pub struct Config {')
    if config_start != -1:
        # Find the closing brace
        brace_pos = content.find('}', config_start)
        # Insert new field before the closing brace
        insert_text = '''    pub notifications: NotificationConfig,
'''
        content = content[:brace_pos] + insert_text + content[brace_pos:]

# Add NotificationConfig struct
if 'pub struct NotificationConfig' not in content:
    # Add after Config struct
    config_end = content.find('impl Config {')
    if config_end != -1:
        new_struct = '''
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub email: Option<String>,
    pub webhook_url: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            email: None,
            webhook_url: None,
        }
    }
}

'''
        content = content[:config_end] + new_struct + content[config_end:]

with open('core/src/config.rs', 'w') as f:
    f.write(content)

print("✅ Added configuration fields to Config structs")

# Fix 2: Update settings handler to use real fields
with open('core/src/api/handlers/settings.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'auto_scan: false, // TODO: Add to ScannerConfig',
    'auto_scan: config.scanner.auto_scan,'
)

content = content.replace(
    'scan_schedule: String::new(), // TODO: Add to ScannerConfig',
    'scan_schedule: config.scanner.schedule.clone().unwrap_or_default(),'
)

content = content.replace(
    'notification_enabled: false, // TODO: Add notifications to Config',
    'notification_enabled: config.notifications.enabled,'
)

with open('core/src/api/handlers/settings.rs', 'w') as f:
    f.write(content)

print("✅ Updated settings handler to use real config fields")

# Fix 3: Update default config values
with open('core/src/config.rs', 'r') as f:
    content = f.read()

# Update ScannerConfig default
if 'auto_scan: false' not in content:
    # Find ScannerConfig Default impl
    scanner_default = content.find('impl Default for ScannerConfig')
    if scanner_default != -1:
        # Find the closing brace of the default impl
        self_start = content.find('Self {', scanner_default)
        self_end = content.find('}', self_start)
        # Add fields before closing
        insert_text = '''            auto_scan: false,
            schedule: None,
'''
        content = content[:self_end] + insert_text + content[self_end:]

# Update Config default
if 'notifications: NotificationConfig::default()' not in content:
    # Find Config Default impl
    config_default = content.find('impl Default for Config')
    if config_default != -1:
        # Find the closing brace of the default impl
        self_start = content.find('Self {', config_default)
        self_end = content.find('}', self_start)
        # Add field before closing
        insert_text = '''            notifications: NotificationConfig::default(),
'''
        content = content[:self_end] + insert_text + content[self_end:]

with open('core/src/config.rs', 'w') as f:
    f.write(content)

print("✅ Updated default config implementations")
print("\n✅ All configuration fields implemented!")